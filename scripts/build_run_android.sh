#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

TARGET="${TARGET:-aarch64}"
SKIP_BUILD=0
MODE="dev"

usage() {
  cat <<'EOF'
Usage: scripts/build_run_android.sh [options]

Builds and runs the Android app on an emulator or connected device.

Options:
  --target <target>    Rust Android target (default: "aarch64")
                       Options: aarch64, armv7, x86_64, i686
  --release            Build in release mode (default: dev/debug)
  --skip-build         Skip the build step (reuse existing APK)
  --open               Open the Android project in Android Studio
  -h, --help           Show this help

Prerequisites:
  - Android Studio with SDK and NDK installed
  - ANDROID_HOME and NDK_HOME environment variables set
  - Rust Android targets installed:
      rustup target add aarch64-linux-android armv7-linux-androideabi \
        x86_64-linux-android i686-linux-android
  - JDK 17+
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --target)
      TARGET="${2:-}"
      shift 2
      ;;
    --release)
      MODE="release"
      shift
      ;;
    --skip-build)
      SKIP_BUILD=1
      shift
      ;;
    --open)
      echo "Opening Android project in Android Studio..."
      if command -v studio >/dev/null 2>&1; then
        studio src-tauri/gen/android
      elif [[ -d "/Applications/Android Studio.app" ]]; then
        open -a "Android Studio" src-tauri/gen/android
      else
        echo "Android Studio not found in PATH or /Applications." >&2
        echo "Open src-tauri/gen/android/ manually in Android Studio." >&2
        exit 1
      fi
      exit 0
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

check_prerequisites() {
  local missing=0

  if [[ -z "${ANDROID_HOME:-}" ]]; then
    echo "ERROR: ANDROID_HOME is not set." >&2
    echo "  Set it to your Android SDK path (e.g. ~/Android/Sdk or ~/Library/Android/sdk)." >&2
    missing=1
  fi

  if [[ -z "${NDK_HOME:-}" ]] && [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
    echo "WARNING: NDK_HOME / ANDROID_NDK_HOME is not set." >&2
    echo "  The build may fail if the NDK cannot be auto-detected." >&2
  fi

  if ! command -v java >/dev/null 2>&1; then
    echo "ERROR: java not found in PATH. Install JDK 17+." >&2
    missing=1
  fi

  if ! rustup target list --installed | grep -q "linux-android"; then
    echo "ERROR: No Android Rust targets installed." >&2
    echo "  Run: rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android" >&2
    missing=1
  fi

  if [[ "$missing" -eq 1 ]]; then
    exit 1
  fi
}

resolve_npm() {
  if command -v npm >/dev/null 2>&1; then
    command -v npm
    return
  fi

  for candidate in /opt/homebrew/bin/npm /usr/local/bin/npm; do
    if [[ -x "$candidate" ]]; then
      echo "$candidate"
      return
    fi
  done

  if [[ -n "${NVM_DIR:-}" && -s "${NVM_DIR}/nvm.sh" ]]; then
    # shellcheck source=/dev/null
    . "${NVM_DIR}/nvm.sh"
    if command -v npm >/dev/null 2>&1; then
      command -v npm
      return
    fi
  fi

  return 1
}

check_prerequisites

NPM_BIN="$(resolve_npm || true)"
if [[ -z "$NPM_BIN" ]]; then
  echo "Unable to find npm in PATH or common install locations." >&2
  echo "Install Node/npm, or run from a shell where npm is available." >&2
  exit 1
fi

if [[ ! -d "src-tauri/gen/android" ]]; then
  echo "Android project not found. Initializing..."
  "$NPM_BIN" run tauri -- android init
fi

if [[ "$SKIP_BUILD" -eq 0 ]]; then
  echo "Building Android app (target: $TARGET, mode: $MODE)..."
  if [[ "$MODE" == "release" ]]; then
    "$NPM_BIN" run tauri -- android build --target "$TARGET" --config src-tauri/tauri.android.conf.json
  else
    "$NPM_BIN" run tauri -- android dev --target "$TARGET" --config src-tauri/tauri.android.conf.json
  fi
fi

echo
echo "Android build complete (target: $TARGET, mode: $MODE)."
