# SysMon

Lightweight system monitor with network traffic tracking.

## Features

- CPU, Memory, GPU monitoring
- Network traffic tracking with daily/monthly limits
- Alert notifications when limits reached
- Auto-disconnect when limit exceeded
- SQLite database for history
- Minimalist dark theme UI
- Auto-start with Windows
- Single instance only

## Download

Download the latest release from: https://github.com/thumb2086/SysMon/releases

### Windows

- `sysmon.exe` - Standalone executable
- Run `installer\install.bat` as Administrator to install

### Linux

```bash
chmod +x sysmon
./sysmon
```

### macOS

```bash
chmod +x sysmon
./sysmon
```

## Configuration

Edit `config.toml` to customize:

- Traffic limits (daily/monthly)
- Alert thresholds
- Interface theme
- Auto-start settings

## Building from Source

```bash
cargo build --release
```

## License

MIT
