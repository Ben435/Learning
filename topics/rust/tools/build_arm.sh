#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=pi@192.168.1.245
readonly TARGET_PATH=/home/pi/tools
readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/tools

cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}
