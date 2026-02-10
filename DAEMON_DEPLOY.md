# CodexMonitor Daemon — Standalone Deployment Guide

Deploy the daemon on a remote PC so your phone (or any CodexMonitor mobile client) can connect to it without needing the full desktop UI.

## What You Need on the Remote PC

### Minimum Requirements

1. **`codex_monitor_daemon.exe`** — the daemon binary (~30 MB)
2. **`codex` CLI** — the Codex CLI must be installed and in `PATH`
3. **Tailscale** (or same LAN) — for network connectivity between phone and PC

That's it. No Node.js, no Rust toolchain, no build tools needed on the remote machine.

### Files to Copy

From your build machine, copy this single file to the remote PC:

```
src-tauri\target\debug\codex_monitor_daemon.exe
```

For a smaller, optimized binary, build in release mode on your build machine:

```powershell
cargo build --bin codex_monitor_daemon --release
# Output: src-tauri\target\release\codex_monitor_daemon.exe (~10 MB)
```

## Running the Daemon (Headless, No UI)

### Basic Usage

```powershell
codex_monitor_daemon.exe --token YOUR_SECRET_TOKEN --port 4732
```

The daemon will start listening on `0.0.0.0:4732` and accept connections authenticated with the given token.

### Run as Background Process

```powershell
# PowerShell — run in background
Start-Process -FilePath ".\codex_monitor_daemon.exe" `
  -ArgumentList "--token", "YOUR_SECRET_TOKEN", "--port", "4732" `
  -WindowStyle Hidden
```

### Run on Startup (Task Scheduler)

1. Open **Task Scheduler** (`taskschd.msc`)
2. Create Basic Task → name it "CodexMonitor Daemon"
3. Trigger: **At startup**
4. Action: **Start a program**
   - Program: `C:\path\to\codex_monitor_daemon.exe`
   - Arguments: `--token YOUR_SECRET_TOKEN --port 4732`
   - Start in: `C:\path\to\` (directory containing the exe)
5. Check **Run whether user is logged on or not**

### Run as Windows Service (via NSSM)

```powershell
# Install NSSM (Non-Sucking Service Manager)
winget install nssm

# Register as service
nssm install CodexMonitorDaemon "C:\path\to\codex_monitor_daemon.exe"
nssm set CodexMonitorDaemon AppParameters "--token YOUR_SECRET_TOKEN --port 4732"
nssm set CodexMonitorDaemon Start SERVICE_AUTO_START

# Start the service
nssm start CodexMonitorDaemon
```

## Firewall

Allow inbound TCP on the daemon port:

```powershell
# Run as Administrator
New-NetFirewallRule -DisplayName "CodexMonitor Daemon" `
  -Direction Inbound -Protocol TCP -LocalPort 4732 -Action Allow
```

## Connecting from Mobile

1. Open CodexMonitor on your phone
2. Go to **Settings → Server**
3. Set **Connection type** to **TCP**
4. **Host**: `<remote-pc-ip>:4732`
   - Tailscale IP (e.g. `100.64.0.22:4732`) — works from anywhere
   - LAN IP (e.g. `192.168.25.21:4732`) — works on same network only
5. **Token**: same token used when starting the daemon
6. Tap **Connect & test**

## Tailscale Setup (Recommended)

Tailscale gives you a stable IP that works from anywhere, even outside your home network.

1. Install Tailscale on both the remote PC and your phone
2. Sign in with the same account on both
3. Note the Tailscale IP of the remote PC (usually `100.x.x.x`)
4. Use that IP as the host in the mobile app

## Troubleshooting

| Problem | Solution |
|---------|----------|
| Connection refused | Check firewall rule, verify daemon is running |
| Token mismatch | Ensure same token on daemon and mobile app |
| Can't reach IP | Verify Tailscale is connected on both devices |
| Daemon crashes | Check that `codex` CLI is in PATH on the remote PC |
| Port in use | Change port with `--port 4733` |

### Verify Daemon is Running

```powershell
# Check if daemon process is running
Get-Process codex_monitor_daemon -ErrorAction SilentlyContinue

# Check if port is listening
netstat -ano | Select-String "4732"
```

### Test Connectivity

```powershell
# From another machine
Test-NetConnection -ComputerName <remote-ip> -Port 4732
```

## To fetch from upstream

```powershell
git fetch upstream
git merge upstream/main
```
# Solve conflicts, if any
```powershell
git push origin main
```



