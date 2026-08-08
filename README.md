# SysMon

Lightweight system monitor with network traffic tracking.

## Features

- CPU, Memory, GPU monitoring
- Network traffic tracking with daily/monthly limits
- Alert notifications when limits reached
- Auto-disconnect when limit exceeded
- System tray integration
- Auto-start with Windows
- Single instance only

## Build

```bash
cargo build --release
```

## Usage

Run `sysmon.exe` (Windows) or `sysmon` (Linux/macOS).

## Configuration

Edit `config.toml` to customize:
- Traffic limits
- Alert thresholds
- Interface theme
- Auto-start settings

## License

MIT
