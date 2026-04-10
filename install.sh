#!/bin/sh
# dkit installer script
# Usage: curl -sSL https://raw.githubusercontent.com/syang0531/dkit/main/install.sh | sh

set -eu

REPO="syang0531/dkit"
BINARY_NAME="dkit"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

get_latest_version() {
    curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' \
        | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/'
}

detect_target() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "${OS}" in
        Linux)
            case "${ARCH}" in
                x86_64)  echo "x86_64-unknown-linux-musl" ;;
                aarch64) echo "aarch64-unknown-linux-gnu" ;;
                *)       echo "Error: unsupported architecture ${ARCH}" >&2; exit 1 ;;
            esac
            ;;
        Darwin)
            case "${ARCH}" in
                x86_64)  echo "x86_64-apple-darwin" ;;
                arm64)   echo "aarch64-apple-darwin" ;;
                *)       echo "Error: unsupported architecture ${ARCH}" >&2; exit 1 ;;
            esac
            ;;
        *)
            echo "Error: unsupported OS ${OS}. Use Windows binaries from GitHub Releases." >&2
            exit 1
            ;;
    esac
}

main() {
    echo "Installing ${BINARY_NAME}..."

    VERSION="${VERSION:-$(get_latest_version)}"
    TARGET="$(detect_target)"
    ARCHIVE_NAME="${BINARY_NAME}-${VERSION}-${TARGET}.tar.gz"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE_NAME}"
    CHECKSUM_URL="${DOWNLOAD_URL%.tar.gz}.sha256"

    echo "  Version: ${VERSION}"
    echo "  Target:  ${TARGET}"
    echo "  Install: ${INSTALL_DIR}"

    TMPDIR="$(mktemp -d)"
    trap 'rm -rf "${TMPDIR}"' EXIT

    echo "Downloading ${ARCHIVE_NAME}..."
    curl -sSfL "${DOWNLOAD_URL}" -o "${TMPDIR}/${ARCHIVE_NAME}"

    echo "Verifying checksum..."
    curl -sSfL "${CHECKSUM_URL}" -o "${TMPDIR}/checksum.sha256"
    (cd "${TMPDIR}" && shasum -a 256 -c checksum.sha256)

    echo "Extracting..."
    tar xzf "${TMPDIR}/${ARCHIVE_NAME}" -C "${TMPDIR}"

    echo "Installing to ${INSTALL_DIR}..."
    if [ -w "${INSTALL_DIR}" ]; then
        install -m 755 "${TMPDIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    else
        sudo install -m 755 "${TMPDIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    fi

    echo ""
    echo "${BINARY_NAME} ${VERSION} has been installed to ${INSTALL_DIR}/${BINARY_NAME}"
    echo "Run '${BINARY_NAME} --help' to get started."
}

main
