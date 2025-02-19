#!/bin/bash
set -e

RELEASE="latest"
OS="$(uname -s)"
INSTALL_DIR="$HOME/.snm"

# Parse Flags
parse_args() {
  while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
    -d | --install-dir)
      INSTALL_DIR="$2"
      shift # past argument
      shift # past value
      ;;
    -s | --skip-shell)
      SKIP_SHELL="true"
      shift # past argument
      ;;
    -r | --release)
      RELEASE="$2"
      shift # past release argument
      shift # past release value
      ;;
    *)
      echo "Unrecognized argument $key"
      exit 1
      ;;
    esac
  done
}

set_filename() {
  # 首先处理不支持的情况
  if [ "$OS" != "Linux" ] && [ "$OS" != "Darwin" ]; then
    echo "OS $OS is not supported."
    echo "If you think that's a bug - please file an issue to https://github.com/sheinsight/snm/issues"
    exit 1
  fi

  # 处理具体的系统和架构
  case "$OS" in
  "Linux")
    case "$(uname -m)" in
    arm | armv7*)
      echo "OS $OS is not supported."
      exit 1
      ;;
    *)
      FILENAME="x86_64-unknown-linux-musl.tar.gz"
      ;;
    esac
    ;;
  "Darwin")
    case "$(uname -m)" in
    "arm64")
      FILENAME="aarch64-apple-darwin.tar.gz"
      ;;
    *)
      FILENAME="x86_64-apple-darwin.tar.gz"
      ;;
    esac
    echo "Downloading the latest snm binary from GitHub..."
    ;;
  esac
}

download_snm() {
  if [ "$RELEASE" = "latest" ]; then
    URL="https://github.com/sheinsight/snm/releases/latest/download/$FILENAME"
  else
    if [[ $RELEASE != v* ]]; then
      RELEASE="v$RELEASE"
    fi
    URL="https://github.com/sheinsight/snm/releases/download/$RELEASE/$FILENAME"
  fi
  DOWNLOAD_DIR=$(mktemp -d)
  echo "Downloading $URL..."
   # 创建安装目录
  if ! mkdir -p "$INSTALL_DIR" 2>/dev/null; then
    echo "Failed to create directory: $INSTALL_DIR"
    echo "Please check permissions and try again."
    exit 1
  fi
  
  if ! curl --progress-bar --fail -L "$URL" -o "$DOWNLOAD_DIR/$FILENAME"; then
    echo "Download failed.  Check that the release/filename are correct."
    exit 1
  fi
  # unzip -q "$DOWNLOAD_DIR/$FILENAME" -d "$DOWNLOAD_DIR"
  tar -xzf "$DOWNLOAD_DIR/$FILENAME" -C "$DOWNLOAD_DIR"

  for file in "$DOWNLOAD_DIR"/*; do
    chmod u+x "$file"
    mv "$file" "$INSTALL_DIR"
  done

  echo "Downloaded to $DOWNLOAD_DIR"
}

check_dependencies() {
  echo "Checking dependencies for the installation script..."

  echo -n "Checking availability of curl... "
  if hash curl 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  echo -n "Checking availability of tar... "
  if hash tar 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  if [ "$SHOULD_EXIT" = "true" ]; then
    echo "Not installing fnm due to missing dependencies."
    exit 1
  fi
}

ensure_containing_dir_exists() {
  local CONTAINING_DIR
  CONTAINING_DIR="$(dirname "$1")"
  if [ ! -d "$CONTAINING_DIR" ]; then
    echo " >> Creating directory $CONTAINING_DIR"
    mkdir -p "$CONTAINING_DIR"
  fi
}

setup_shell() {
  CURRENT_SHELL="$(basename "$SHELL")"

  case "$CURRENT_SHELL" in
  "zsh")
    CONF_FILE="${ZDOTDIR:-$HOME}/.zshrc"
    SHELL_NAME="Zsh"
    ;;
  "bash")
    CONF_FILE="$HOME/$([ "$OS" = "Darwin" ] && echo ".profile" || echo ".bashrc")"
    SHELL_NAME="Bash"
    ;;
  *)
    echo "Could not infer shell type. Please set up manually."
    exit 1
    ;;
  esac

  ensure_containing_dir_exists "$CONF_FILE"

  echo "Installing for $SHELL_NAME. Appending the following to $CONF_FILE:"
  echo ""
  echo "  # snm"
  echo "  export PATH=\"$INSTALL_DIR:\$PATH\""

  {
    echo ""
    echo "# snm"
    echo "export PATH=\"$INSTALL_DIR:\$PATH\""
  } >>"$CONF_FILE"

  echo -e "\nIn order to apply the changes, open a new terminal or run the following command:\n"
  echo "  source $CONF_FILE"
}

parse_args "$@"
set_filename
check_dependencies
download_snm

if [ "$SKIP_SHELL" != "true" ]; then
  setup_shell
fi
