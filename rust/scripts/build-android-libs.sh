#!/bin/sh

cd "`dirname "$(realpath -- "$0")"`/.."

export PATH="~/Android/toolchains/android-21-arm/bin:$PATH"
export PATH="~/Android/toolchains/android-21-x86/bin:$PATH"

build() {
	cargo build --target $1 --release
}

build arm-linux-androideabi
build i686-linux-android

