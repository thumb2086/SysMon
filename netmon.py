import tkinter as tk
import psutil
import subprocess
import time
import threading
import ctypes
from collections import deque


def fmt_spd(bps):
    mbps = bps * 8 / 10**6
    if mbps >= 1000:
        return f"{mbps / 1000:.2f} Gbps".rjust(12)
    elif mbps >= 1:
        return f"{mbps:.1f} Mbps".rjust(12)
    else:
        return f"{mbps * 1000:.0f} Kbps".rjust(12)


def fmt_bytes(b):
    if b >= 2**40:
        return f"{b / 2**40:.2f} TB"
    elif b >= 2**30:
        return f"{b / 2**30:.1f} GB"
    elif b >= 2**20:
        return f"{b / 2**20:.0f} MB"
    else:
        return f"{b / 2**10:.0f} KB"


class SysMon:
    BG = "#f0f0f0"
    CARD = "#ffffff"
    BORDER = "#e0e0e0"
    TXT = "#1a1a1a"
    DIM = "#888888"
    GRN = "#16a34a"
    ORG = "#d97706"
    RED = "#dc2626"
    BLU = "#2563eb"
    PPL = "#9333ea"
    CYN = "#0891b2"
    PNK = "#db2777"
    YEL = "#ca8a04"
    BGRN = "#dcfce7"
    BORG = "#fef3c7"
    BRED = "#fee2e2"

    def __init__(self):
        self.root = tk.Tk()
        self.root.title("SysMon")
        self.root.configure(bg=self.BG)
        self.root.attributes("-topmost", True)
        self.root.resizable(False, False)

        self._dsk_r = self._dsk_w = self._dsk_t = 0
        self._net_s = self._net_r = self._net_t = 0
        self._net_h = deque(maxlen=60)
        self._net_up_h = deque(maxlen=60)
        self._dr_h = deque(maxlen=60)
        self._dw_h = deque(maxlen=60)
        self._cpu_hist = deque(maxlen=60)
        self._gpu_hist = deque(maxlen=60)
        self._net_dn_max = 1
        self._net_up_max = 1

        self._gpu_ok = False
        self._gpu_name = ""
        self._probe_gpu()

        self._core_freqs = []
        self._freq_tick = 0
        self._disk_activity = []

        self._bg_cpu_temp = None
        self._bg_cpu_freqs = []
        self._bg_gpu_data = None
        self._bg_disk_act = []
        self._last_parts = set()
        self._bg_cpu_power = None
        self._bg_sys_power = None

        self._build()
        self._place()
        self._rebuild_disk()
        self._bg_probe_all()
        self._tick()

    # ── probes ──

    def _probe_gpu(self):
        try:
            r = subprocess.run(["nvidia-smi", "--query-gpu=name", "--format=csv,noheader"],
                               capture_output=True, text=True, timeout=3,
                               creationflags=subprocess.CREATE_NO_WINDOW)
            if r.returncode == 0 and r.stdout.strip():
                self._gpu_name = r.stdout.strip().replace("NVIDIA ", "")
                self._gpu_ok = True
        except Exception:
            pass

    def _nvidia(self, fields):
        if not self._gpu_ok:
            return None
        try:
            r = subprocess.run(["nvidia-smi", f"--query-gpu={fields}", "--format=csv,noheader,nounits"],
                               capture_output=True, text=True, timeout=3,
                               creationflags=subprocess.CREATE_NO_WINDOW)
            if r.returncode == 0:
                return [p.strip() for p in r.stdout.strip().split(", ")]
        except Exception:
            pass
        return None

    def _bg_probe_all(self):
        def work():
            while True:
                try:
                    r = subprocess.run(
                        ["powershell", "-NoProfile", "-Command",
                         "Get-CimInstance -Namespace root/WMI -ClassName MSAcpi_ThermalZoneTemperature | Select-Object -ExpandProperty CurrentTemperature"],
                        capture_output=True, text=True, timeout=5,
                        creationflags=subprocess.CREATE_NO_WINDOW)
                    vals = [int(x) for x in r.stdout.strip().split() if x.isdigit()]
                    if vals:
                        self._bg_cpu_temp = max(vals) / 10.0 - 273.15
                except Exception:
                    self._bg_cpu_temp = None

                try:
                    r = subprocess.run(
                        ["powershell", "-NoProfile", "-Command",
                         "Get-CimInstance -Namespace root/cimv2 -ClassName Win32_PerfFormattedData_Counters_ProcessorInformation | Where-Object Name -ne '_Total' | Select-Object ActualFrequency"],
                        capture_output=True, text=True, timeout=5,
                        creationflags=subprocess.CREATE_NO_WINDOW)
                    vals = [int(x) for x in r.stdout.strip().split() if x.isdigit()]
                    if vals:
                        self._bg_cpu_freqs = vals
                except Exception:
                    pass

                if self._gpu_ok:
                    try:
                        r = subprocess.run(
                            ["nvidia-smi", "--query-gpu=utilization.gpu,utilization.memory,utilization.encoder,utilization.decoder,temperature.gpu,power.draw,memory.used,memory.total",
                             "--format=csv,noheader,nounits"],
                            capture_output=True, text=True, timeout=5,
                            creationflags=subprocess.CREATE_NO_WINDOW)
                        if r.returncode == 0:
                            parts = [p.strip() for p in r.stdout.strip().split(", ")]
                            if len(parts) >= 8:
                                self._bg_gpu_data = [float(x) for x in parts]
                    except Exception:
                        pass

                try:
                    r = subprocess.run(
                        ["powershell", "-NoProfile", "-Command",
                         "Get-CimInstance Win32_PerfFormattedData_PerfDisk_PhysicalDisk | Where-Object Name -ne '_Total' | Select-Object Name, PercentDiskTime | ConvertTo-Csv -NoTypeInformation"],
                        capture_output=True, text=True, timeout=5,
                        creationflags=subprocess.CREATE_NO_WINDOW)
                    lines = r.stdout.strip().splitlines()
                    result = []
                    for line in lines[1:]:
                        parts = line.strip('"').split('","')
                        if len(parts) >= 2:
                            result.append((parts[0].strip('"'), float(parts[1].strip('"'))))
                    self._bg_disk_act = result
                except Exception:
                    pass

                time.sleep(2)

        t = threading.Thread(target=work, daemon=True)
        t.start()

    # ── UI helpers ──

    def _bar(self, canvas, pct, fg, tag="fill"):
        canvas.delete(tag)
        w = canvas.winfo_width() or 260
        h = int(canvas["height"])
        canvas.create_rectangle(0, 0, w, h, fill=self.BG, outline="", tags="bg")
        bw = max(0, int(w * min(pct, 100) / 100))
        if bw > 0:
            canvas.create_rectangle(0, 0, bw, h, fill=fg, outline="", tags=tag)

    def _tag_row(self, parent, label, value, color, width=4):
        f = tk.Frame(parent, bg=self.CARD)
        f.pack(side=tk.LEFT, padx=(0, 10))
        tk.Label(f, text=label, bg=self.CARD, fg=self.DIM,
                 font=("Consolas", 7), width=width, anchor="w").pack(side=tk.TOP, anchor="w")
        lb = tk.Label(f, text=value, bg=self.CARD, fg=color,
                      font=("Consolas", 10, "bold"))
        lb.pack(side=tk.TOP, anchor="w")
        return lb

    def _draw_chart(self, canvas, data, color):
        canvas.delete("all")
        gw = canvas.winfo_width() or 260
        gh = 30
        vals = list(data)
        if len(vals) < 2:
            return
        mx = max(vals) or 1
        step = gw / (len(vals) - 1)
        pts = []
        for i, v in enumerate(vals):
            x = i * step
            y = gh - 2 - (v / mx) * (gh - 4)
            pts.append((x, y))
        canvas.create_line([c for p in pts for c in p], fill=color, width=1.5, smooth=True)
        canvas.create_line(0, gh - 1, gw, gh - 1, fill=self.DIM, dash=(2, 4))

    # ── build ──

    def _build(self):
        self._hdr = tk.Frame(self.root, bg=self.CARD, height=28)
        self._hdr.pack(fill=tk.X)
        self._hdr.pack_propagate(False)
        tk.Label(self._hdr, text="  SYS MON", bg=self.CARD, fg=self.DIM,
                 font=("Segoe UI", 8, "bold")).pack(side=tk.LEFT, fill=tk.Y)
        self._uptime = tk.Label(self._hdr, text="", bg=self.CARD, fg=self.DIM,
                                font=("Consolas", 7))
        self._uptime.pack(side=tk.LEFT, fill=tk.Y, padx=(8, 0))
        self._pin_btn = tk.Label(self._hdr, text="📌", bg=self.CARD, fg=self.GRN,
                                 font=("Segoe UI", 9), cursor="hand2")
        self._pin_btn.pack(side=tk.RIGHT, fill=tk.Y, padx=(0, 4))
        self._pin_btn.bind("<Button-1>", self._toggle_pin)
        tk.Label(self._hdr, text="✕  ", bg=self.CARD, fg=self.DIM,
                 font=("Segoe UI", 9), cursor="hand2").pack(side=tk.RIGHT, fill=tk.Y)
        self._hdr.winfo_children()[-1].bind("<Button-1>", lambda e: self.root.quit())

        body = tk.Frame(self.root, bg=self.BG)
        body.pack(fill=tk.BOTH, expand=True, padx=10, pady=(6, 10))

        self._build_cpu(body)
        self._build_gpu(body)
        self._build_mem(body)
        self._build_net(body)
        self._build_dsk(body)

    def _section(self, parent, title):
        f = tk.Frame(parent, bg=self.CARD, padx=10, pady=6)
        f.pack(fill=tk.X, pady=(0, 4))
        hdr = tk.Frame(f, bg=self.CARD)
        hdr.pack(fill=tk.X)
        tk.Label(hdr, text=title, bg=self.CARD, fg=self.DIM,
                 font=("Consolas", 7, "bold")).pack(side=tk.LEFT)
        return f, hdr

    # ── CPU ──

    def _build_cpu(self, p):
        self._cpu_f, self._cpu_h = self._section(p, "CPU")
        self._cpu_pct = tk.Label(self._cpu_h, bg=self.CARD, fg=self.GRN,
                                 font=("Consolas", 14, "bold"))
        self._cpu_pct.pack(side=tk.LEFT, padx=(6, 4))
        self._cpu_frq = tk.Label(self._cpu_h, bg=self.CARD, fg=self.DIM,
                                 font=("Consolas", 9))
        self._cpu_frq.pack(side=tk.LEFT)

        self._cpu_bar = tk.Canvas(self._cpu_f, height=8, bg=self.BG, highlightthickness=0)
        self._cpu_bar.pack(fill=tk.X)

        self._cpu_chart = tk.Canvas(self._cpu_f, height=30, bg=self.CARD, highlightthickness=0)
        self._cpu_chart.pack(fill=tk.X, pady=(2, 0))

        n = psutil.cpu_count(logical=True) or 4
        cols = min(n, 16)
        self._cpu_g = tk.Frame(self._cpu_f, bg=self.CARD)
        self._cpu_g.pack(fill=tk.X, pady=(4, 0))
        for c in range(cols):
            self._cpu_g.columnconfigure(c, weight=1)
        self._cpu_cells = []
        ch = 26
        for i in range(n):
            r, c = i // cols, i % cols
            cell = tk.Frame(self._cpu_g, height=ch, bg=self.BG)
            cell.grid(row=r, column=c, padx=1, pady=1, sticky="ew")
            cell.grid_propagate(False)
            bar = tk.Frame(cell, bg=self.GRN)
            bar.place(x=1, y=ch - 3, width=1, height=0)
            lb = tk.Label(cell, text="", bg=self.BG, fg=self.DIM,
                          font=("Consolas", 6))
            lb.place(x=1, y=0)
            self._cpu_cells.append((cell, bar, lb, ch))

    def _upd_cpu(self):
        if self._freq_tick % 2 == 0:
            self._core_freqs = self._bg_cpu_freqs
        self._freq_tick += 1

        pct = psutil.cpu_percent(interval=None)
        cores = psutil.cpu_percent(interval=None, percpu=True)
        f = psutil.cpu_freq()

        fg = self.GRN if pct < 50 else (self.ORG if pct < 80 else self.RED)
        bg = self.BGRN if pct < 50 else (self.BORG if pct < 80 else self.BRED)
        self._cpu_pct.config(text=f"{pct:.0f}%", fg=fg)
        avg = sum(self._core_freqs) // len(self._core_freqs) if self._core_freqs else (f.current if f else 0)
        self._cpu_frq.config(text=f"{avg / 1000:.2f} GHz" if avg else "")
        self._cpu_f.config(bg=bg)
        self._bar(self._cpu_bar, pct, fg)

        self._cpu_hist.append(pct)
        self._draw_chart(self._cpu_chart, self._cpu_hist, self.GRN)

        for i, cp in enumerate(cores):
            if i >= len(self._cpu_cells):
                break
            cell, bar, lb, ch = self._cpu_cells[i]
            fg2 = self.GRN if cp < 50 else (self.ORG if cp < 80 else self.RED)
            bh = max(0, int((ch - 6) * cp / 100))
            bar.config(bg=fg2)
            bar.place(x=1, y=ch - 3 - bh, width=max(1, cell.winfo_width() - 2), height=bh)
            cf = self._core_freqs[i] if i < len(self._core_freqs) else 0
            ghz = f"{cf / 1000:.1f}" if cf else ""
            lb.config(text=ghz, fg=fg2)

    # ── GPU ──

    def _build_gpu(self, p):
        self._gpu_f, self._gpu_h = self._section(p, "GPU")
        self._gpu_pct = tk.Label(self._gpu_h, bg=self.CARD, fg=self.GRN,
                                 font=("Consolas", 14, "bold"))
        self._gpu_pct.pack(side=tk.LEFT, padx=(6, 4))
        self._gpu_tmp = tk.Label(self._gpu_h, bg=self.CARD, fg=self.GRN,
                                 font=("Consolas", 10))
        self._gpu_tmp.pack(side=tk.LEFT, padx=(0, 4))
        self._gpu_pwr = tk.Label(self._gpu_h, bg=self.CARD, fg=self.DIM,
                                 font=("Consolas", 9))
        self._gpu_pwr.pack(side=tk.LEFT, padx=(0, 4))
        self._gpu_vrm = tk.Label(self._gpu_h, bg=self.CARD, fg=self.DIM,
                                 font=("Consolas", 9))
        self._gpu_vrm.pack(side=tk.LEFT)

        self._gpu_bar = tk.Canvas(self._gpu_f, height=8, bg=self.BG, highlightthickness=0)
        self._gpu_bar.pack(fill=tk.X)

        self._gpu_chart = tk.Canvas(self._gpu_f, height=30, bg=self.CARD, highlightthickness=0)
        self._gpu_chart.pack(fill=tk.X, pady=(2, 0))

        tags = tk.Frame(self._gpu_f, bg=self.CARD)
        tags.pack(fill=tk.X, pady=(4, 0))
        self._gt = {}
        for name, clr in [("3D", self.GRN), ("MEM", self.BLU), ("Enc", self.PNK), ("Dec", self.CYN)]:
            self._gt[name] = self._tag_row(tags, name, "--", clr)

    def _upd_gpu(self):
        if not self._gpu_ok:
            self._gpu_pct.config(text="N/A", fg=self.DIM)
            for lb in self._gt.values():
                lb.config(text="--", fg=self.DIM)
            self._bar(self._gpu_bar, 0, self.DIM)
            return
        data = self._bg_gpu_data
        if not data:
            return
        try:
            g3d = data[0]; gmem = data[1]
            genc = data[2]; gdec = data[3]
            gtmp = data[4]; gpw = data[5]
            vu = data[6]; vt = data[7]
        except (IndexError, TypeError):
            return

        fg = self.GRN if g3d < 50 else (self.ORG if g3d < 80 else self.RED)
        bg = self.BGRN if g3d < 50 else (self.BORG if g3d < 80 else self.BRED)
        self._gpu_pct.config(text=f"{g3d:.0f}%", fg=fg)
        self._gpu_tmp.config(text=f"{gtmp:.0f}°C",
                             fg=self.GRN if gtmp < 60 else (self.ORG if gtmp < 80 else self.RED))
        self._gpu_pwr.config(text=f"{gpw:.1f}W")
        self._gpu_vrm.config(text=f"{vu:.0f}/{vt:.0f}M")
        self._gpu_f.config(bg=bg)
        self._bar(self._gpu_bar, g3d, fg)
        self._gpu_hist.append(g3d)
        self._draw_chart(self._gpu_chart, self._gpu_hist, self.GRN)
        vals = {"3D": (f"{g3d:.0f}%", fg), "MEM": (f"{gmem:.0f}%", self.BLU),
                "Enc": (f"{genc:.0f}%", self.PNK), "Dec": (f"{gdec:.0f}%", self.CYN)}
        for k, (v, c) in vals.items():
            self._gt[k].config(text=v, fg=c)

    # ── MEM ──

    def _build_mem(self, p):
        self._mem_f, self._mem_h = self._section(p, "MEM")
        self._mem_pct = tk.Label(self._mem_h, bg=self.CARD, fg=self.GRN,
                                 font=("Consolas", 14, "bold"))
        self._mem_pct.pack(side=tk.LEFT, padx=(6, 4))
        self._mem_use = tk.Label(self._mem_h, bg=self.CARD, fg=self.DIM,
                                 font=("Consolas", 10))
        self._mem_use.pack(side=tk.LEFT, padx=(0, 4))
        self._mem_swp = tk.Label(self._mem_h, bg=self.CARD, fg=self.DIM,
                                 font=("Consolas", 9))
        self._mem_swp.pack(side=tk.LEFT)

        self._mem_bar = tk.Canvas(self._mem_f, height=8, bg=self.BG, highlightthickness=0)
        self._mem_bar.pack(fill=tk.X)

    def _upd_mem(self):
        m = psutil.virtual_memory()
        sw = psutil.swap_memory()
        p = m.percent
        fg = self.GRN if p < 50 else (self.ORG if p < 80 else self.RED)
        bg = self.BGRN if p < 50 else (self.BORG if p < 80 else self.BRED)
        self._mem_pct.config(text=f"{p:.0f}%", fg=fg)
        self._mem_use.config(text=f"{m.used / 2**30:.1f} / {m.total / 2**30:.1f} GB")
        self._mem_swp.config(text=f"SWAP {sw.used / 2**30:.1f}G" if sw.total > 0 and sw.used / 2**30 > 0.1 else "")
        self._mem_f.config(bg=bg)
        self._bar(self._mem_bar, p, fg)

    # ── NET ──

    def _build_net(self, p):
        self._net_f, self._net_hdr = self._section(p, "NET")

        row1 = tk.Frame(self._net_f, bg=self.CARD)
        row1.pack(fill=tk.X, pady=(0, 2))
        self._nd = tk.Label(row1, bg=self.CARD, fg=self.GRN,
                            font=("Consolas", 10, "bold"))
        self._nd.pack(side=tk.LEFT, padx=(0, 4))
        self._ndt = tk.Label(row1, bg=self.CARD, fg=self.DIM,
                             font=("Consolas", 8), width=9, anchor="e")
        self._ndt.pack(side=tk.RIGHT)
        self._nd_bar = tk.Canvas(row1, height=6, bg=self.BG, highlightthickness=0)
        self._nd_bar.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 6))

        row2 = tk.Frame(self._net_f, bg=self.CARD)
        row2.pack(fill=tk.X, pady=(0, 2))
        self._nu = tk.Label(row2, bg=self.CARD, fg=self.ORG,
                            font=("Consolas", 10, "bold"))
        self._nu.pack(side=tk.LEFT, padx=(0, 4))
        self._nut = tk.Label(row2, bg=self.CARD, fg=self.DIM,
                             font=("Consolas", 8), width=9, anchor="e")
        self._nut.pack(side=tk.RIGHT)
        self._nu_bar = tk.Canvas(row2, height=6, bg=self.BG, highlightthickness=0)
        self._nu_bar.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=(0, 6))

        self._ng = tk.Canvas(self._net_f, height=40, bg=self.CARD, highlightthickness=0)
        self._ng.pack(fill=tk.X, pady=(4, 0))

    def _upd_net(self):
        t = time.time()
        io = psutil.net_io_counters()
        if self._net_s > 0:
            dt = t - self._net_t
            dn = max(0, (io.bytes_recv - self._net_r) / dt)
            up = max(0, (io.bytes_sent - self._net_s) / dt)
        else:
            dn = up = 0
        self._net_s = io.bytes_sent
        self._net_r = io.bytes_recv
        self._net_t = t
        self._net_h.append(dn)
        self._net_up_h.append(up)

        self._nd.config(text=f"▼ {fmt_spd(dn)}")
        self._nu.config(text=f"▲ {fmt_spd(up)}")
        self._ndt.config(text=f"{fmt_bytes(io.bytes_recv)}")
        self._nut.config(text=f"{fmt_bytes(io.bytes_sent)}")

        self._net_dn_max = max(self._net_dn_max * 0.95, dn, 1)
        self._net_up_max = max(self._net_up_max * 0.95, up, 1)
        self._bar(self._nd_bar, dn / self._net_dn_max * 100, self.GRN)
        self._bar(self._nu_bar, up / self._net_up_max * 100, self.ORG)

        self._ng.delete("all")
        gw = self._ng.winfo_width() or 260
        gh = 40
        dn_data = list(self._net_h)
        up_data = list(self._net_up_h)
        mx_all = max(max(dn_data, default=1), max(up_data, default=1), 1)

        if len(dn_data) > 1:
            step = gw / (len(dn_data) - 1)
            for h_data, color in [(dn_data, self.GRN), (up_data, self.ORG)]:
                pts = []
                for i, v in enumerate(h_data):
                    x = i * step
                    y = gh - 2 - (v / mx_all) * (gh - 4)
                    pts.append((x, y))
                if len(pts) >= 2:
                    self._ng.create_line([coord for p in pts for coord in p],
                                         fill=color, width=1.5, smooth=True)

        self._ng.create_line(0, gh // 2, gw, gh // 2, fill=self.DIM, dash=(2, 4))

    # ── DSK ──

    def _build_dsk(self, p):
        self._dsk_f, self._dsk_h = self._section(p, "DSK")
        self._dr = tk.Label(self._dsk_h, bg=self.CARD, fg=self.PPL,
                            font=("Consolas", 10, "bold"))
        self._dr.pack(side=tk.LEFT, padx=(6, 6))
        self._dw = tk.Label(self._dsk_h, bg=self.CARD, fg=self.CYN,
                            font=("Consolas", 10, "bold"))
        self._dw.pack(side=tk.LEFT, padx=(0, 6))
        self._drt = tk.Label(self._dsk_h, bg=self.CARD, fg=self.DIM,
                             font=("Consolas", 8))
        self._drt.pack(side=tk.LEFT, padx=(0, 2))
        self._dwt = tk.Label(self._dsk_h, bg=self.CARD, fg=self.DIM,
                             font=("Consolas", 8))
        self._dwt.pack(side=tk.LEFT)

        self._dg = tk.Canvas(self._dsk_f, height=18, bg=self.CARD, highlightthickness=0)
        self._dg.pack(fill=tk.X, pady=(4, 0))

        self._dsk_phys = tk.Frame(self._dsk_f, bg=self.CARD)
        self._dsk_phys.pack(fill=tk.X, pady=(4, 0))
        self._dsk_phys_rows = []

        self._dsk_parts = tk.Frame(self._dsk_f, bg=self.CARD)
        self._dsk_parts.pack(fill=tk.X, pady=(2, 0))
        self._dsk_part_rows = []

    def _rebuild_disk(self):
        for w in self._dsk_phys.winfo_children():
            w.destroy()
        self._dsk_phys_rows = []
        self._disk_activity = self._bg_disk_act
        for name, pct in self._disk_activity:
            row = tk.Frame(self._dsk_phys, bg=self.CARD)
            row.pack(fill=tk.X, pady=1)
            tk.Label(row, text=name.replace("PhysicalDrive", "D"), bg=self.CARD, fg=self.DIM,
                     font=("Consolas", 7, "bold"), width=3, anchor="w").pack(side=tk.LEFT)
            bar = tk.Canvas(row, height=5, bg=self.BG, highlightthickness=0)
            bar.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=3)
            bar.create_rectangle(0, 0, 200, 5, fill=self.BG, outline="", tags="bg")
            bar.create_rectangle(0, 0, 0, 5, fill=self.GRN, outline="", tags="fill")
            lb = tk.Label(row, text="", bg=self.CARD, fg=self.DIM,
                          font=("Consolas", 6), width=4, anchor="e")
            lb.pack(side=tk.RIGHT)
            self._dsk_phys_rows.append({"bar": bar, "lb": lb, "name": name})

        for w in self._dsk_parts.winfo_children():
            w.destroy()
        self._dsk_part_rows = []
        parts = psutil.disk_partitions()
        valid = []
        for pt in parts:
            if pt.fstype and 'cdrom' not in pt.opts:
                try:
                    u = psutil.disk_usage(pt.mountpoint)
                    valid.append((pt.mountpoint, u))
                except Exception:
                    pass
        valid.sort(key=lambda x: x[1].total, reverse=True)
        for mp, u in valid:
            row = tk.Frame(self._dsk_parts, bg=self.CARD)
            row.pack(fill=tk.X, pady=1)
            lbl = mp.replace(":\\", "").replace("\\", "") or "C"
            tk.Label(row, text=lbl, bg=self.CARD, fg=self.DIM,
                     font=("Consolas", 7, "bold"), width=3, anchor="w").pack(side=tk.LEFT)
            txt = tk.Label(row, text="", bg=self.CARD, fg=self.TXT,
                           font=("Consolas", 8), width=12, anchor="e")
            txt.pack(side=tk.RIGHT)
            bar = tk.Canvas(row, height=5, bg=self.BG, highlightthickness=0)
            bar.pack(side=tk.LEFT, fill=tk.X, expand=True, padx=3)
            bar.create_rectangle(0, 0, 200, 5, fill=self.BG, outline="", tags="bg")
            bar.create_rectangle(0, 0, 0, 5, fill=self.GRN, outline="", tags="fill")
            self._dsk_part_rows.append({"bar": bar, "txt": txt, "mp": mp})

        for dr in self._dsk_part_rows:
            try:
                u = psutil.disk_usage(dr["mp"])
                dr["txt"].config(text=f"{u.used / 2**30:.0f}/{u.total / 2**30:.0f}G")
            except Exception:
                pass

    def _upd_disk(self):
        t = time.time()
        io = psutil.disk_io_counters()
        if not io:
            return
        if self._dsk_r > 0:
            dt = t - self._dsk_t
            rs = max(0, (io.read_bytes - self._dsk_r) / dt)
            ws = max(0, (io.write_bytes - self._dsk_w) / dt)
        else:
            rs = ws = 0
        self._dsk_r = io.read_bytes
        self._dsk_w = io.write_bytes
        self._dsk_t = t
        self._dr_h.append(rs)
        self._dw_h.append(ws)

        self._dr.config(text=f"◀ {fmt_spd(rs)}")
        self._dw.config(text=f"▶ {fmt_spd(ws)}")
        self._drt.config(text=f"{fmt_bytes(io.read_bytes)}")
        self._dwt.config(text=f"{fmt_bytes(io.write_bytes)}")

        self._dg.delete("all")
        gw = self._dg.winfo_width() or 260
        gh = 18
        mx = max(max(self._dr_h) if self._dr_h else 1, max(self._dw_h) if self._dw_h else 1)
        if mx > 0 and len(self._dr_h) > 1:
            bw = max(2, gw / len(self._dr_h) - 1)
            for i in range(len(self._dr_h)):
                x = i * (bw + 1)
                rh = (self._dr_h[i] / mx) * (gh - 4)
                wh = (self._dw_h[i] / mx) * (gh - 4)
                self._dg.create_rectangle(x, gh - 2 - rh, x + bw // 2, gh - 2, fill=self.PPL, outline="")
                self._dg.create_rectangle(x + bw // 2, gh - 2 - wh, x + bw, gh - 2, fill=self.CYN, outline="")

        self._disk_activity = self._bg_disk_act
        for dr in self._dsk_phys_rows:
            match = [a for a in self._disk_activity if a[0] == dr["name"]]
            if match:
                pct = match[0][1]
                fgc = self.GRN if pct < 30 else (self.ORG if pct < 60 else self.RED)
                bw = dr["bar"].winfo_width() or 180
                dr["bar"].itemconfig("fill", fill=fgc)
                dr["bar"].coords("fill", 0, 0, max(1, int(bw * pct / 100)), 5)
                dr["lb"].config(text=f"{pct:.0f}%")

        for dr in self._dsk_part_rows:
            try:
                u = psutil.disk_usage(dr["mp"])
            except Exception:
                continue
            p = u.percent
            fgc = self.GRN if p < 70 else (self.ORG if p < 90 else self.RED)
            bw = dr["bar"].winfo_width() or 180
            dr["bar"].itemconfig("fill", fill=fgc)
            dr["bar"].coords("fill", 0, 0, max(1, int(bw * p / 100)), 5)
            dr["txt"].config(text=f"{u.used / 2**30:.0f}/{u.total / 2**30:.0f}G")

    # ── lifecycle ──

    def _place(self):
        sw = self.root.winfo_screenwidth()
        sh = self.root.winfo_screenheight()
        self.root.geometry(f"380x640+{sw - 400}+{sh - 700}")

    def _toggle_pin(self, e=None):
        cur = self.root.attributes("-topmost")
        self.root.attributes("-topmost", not cur)
        self._pin_btn.config(fg=self.GRN if not cur else self.DIM)

    def _tick(self):
        boot = psutil.boot_time()
        uptime_s = time.time() - boot
        d = int(uptime_s // 86400)
        h = int((uptime_s % 86400) // 3600)
        m = int((uptime_s % 3600) // 60)
        self._uptime.config(text=f"UP {d}d {h}h {m}m" if d else f"UP {h}h {m}m")

        cur_parts = set()
        for pt in psutil.disk_partitions():
            if pt.fstype and 'cdrom' not in pt.opts:
                cur_parts.add(pt.device)
        if cur_parts != self._last_parts:
            self._last_parts = cur_parts
            self._rebuild_disk()

        self._upd_cpu()
        self._upd_gpu()
        self._upd_mem()
        self._upd_net()
        self._upd_disk()
        self.root.after(500, self._tick)

    def run(self):
        self.root.mainloop()


if __name__ == "__main__":
    SysMon().run()
