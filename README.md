# SysMon — System Monitor

Real-time system monitoring tool built with Python + tkinter.  
Shows CPU, GPU, MEM, NET, DISK usage with live charts and daily traffic logging.

![screenshot](SysMon/dark.svg)

## Features

- **CPU** — per-core usage bars + frequency + 60s trend chart
- **GPU** — NVIDIA 3D/MEM/Enc/Dec %, temperature, power draw, VRAM
- **MEM** — RAM usage + SWAP
- **NET** — real-time down/up speed, 60s dual-line chart, cumulative total
- **DSK** — per-disk R/W speed, per-partition usage bars
- **📅 TODAY** — daily network traffic total (persisted to `%APPDATA%/SysMon/daily.json`)
- **📋 APPS** — installed applications list from registry
- **🌙 Dark/Light** — auto-detects Windows theme
- **📌 Pin** — always-on-top toggle
- **🔄 Auto-start** — toggle in header (`● AUTO`)
- **⎯ Minimize to tray** — keeps running in background

## Quick Start

```bash
pip install psutil pyinstaller
python SysMon/netmon.py
```

Or download the latest `SysMon.exe` from [Releases](https://github.com/thumb2086/SysMon/releases).

## Build

```bash
pip install pyinstaller psutil
pyinstaller --onefile --windowed --name SysMon SysMon/netmon.py
```

CI builds automatically on push to `main` via GitHub Actions.
