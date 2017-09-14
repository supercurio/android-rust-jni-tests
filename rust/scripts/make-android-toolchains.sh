#!/usr/bin/env bash

make_toolchain() {
    PLATFORM=${1}
    ARCH=${2}
    INSTALL_DIR="$HOME/Android/toolchains/${PLATFORM}-${ARCH}"

    # clean dir
    rm -rf ${INSTALL_DIR}

    $ANDROID_NDK/build/tools/make-standalone-toolchain.sh --verbose \
        --platform=${PLATFORM} \
        --install-dir=${INSTALL_DIR} \
        --arch=${ARCH}
}


make_toolchain android-21 arm
make_toolchain android-21 x86
