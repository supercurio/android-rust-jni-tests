#!/bin/sh

cd "`dirname "$(realpath -- "$0")"`/.."

copy() {
	DST="../app/src/main/jniLibs/$2"
	mkdir -p ${DST}
	cp -v ./target/$1/release/libandroid_rust_jni_tests.so ${DST}/
}

copy i686-linux-android x86
copy arm-linux-androideabi armeabi
