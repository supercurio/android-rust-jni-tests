# Android Rust JNI tests

Test Android app built to stress jni-rs crate. Created to report a bug and propose a fix in a
Pull Request on github.

## How to build the Android application

1. Run the script `./rust/scripts/make-android-toolchains.sh` to generate the required armeabi
and i686 standalone Android NDK toolchains.
2. Since cargo is pre-configured in `rust/.cargo/config`, compile the JNI library which the Android
app relies on. Run `rust/scripts/build-android-libs.sh` to do so.
3. Copy the .so files in the expected location by running `./rust/scripts/copy-libs-to-app.sh`
4. Build, install and launch the Android app with gradle or Android Studio.
5. The logcat output will show a crash with jni-rs 0.5.2 or no crash with my proposed fix.

## Proposed fix
Test the proposed fix by swapping the jni dependency in `rust/Cargo.toml`.

### Corresponding commit:
https://github.com/supercurio/jni-rs/commit/b6de87479c2bad04878b6c012a269d82a31e0068

## Note
On Android 8.0, ART java runtime does generate a crash but without calling `env.delete_local_ref`, 
the iteration speed when walking though this example's array progressively slow downs.

## Typical crash output

```
09-14 21:12:04.013  9615  9615 I android_rust_jni_tests: array len: 200001
09-14 21:12:04.013  9615  9615 I android_rust_jni_tests: at item value: 0
09-14 21:12:04.015  9615  9615 I android_rust_jni_tests: at item value: 50
09-14 21:12:04.016  9615  9615 I android_rust_jni_tests: at item value: 100
09-14 21:12:04.017  9615  9615 I android_rust_jni_tests: at item value: 150
09-14 21:12:04.018  9615  9615 I android_rust_jni_tests: at item value: 200
09-14 21:12:04.020  9615  9615 I android_rust_jni_tests: at item value: 250
09-14 21:12:04.021  9615  9615 I android_rust_jni_tests: at item value: 300
09-14 21:12:04.022  9615  9615 I android_rust_jni_tests: at item value: 350
09-14 21:12:04.024  9615  9615 I android_rust_jni_tests: at item value: 400
09-14 21:12:04.025  9615  9615 I android_rust_jni_tests: at item value: 450
09-14 21:12:04.028  9615  9615 I android_rust_jni_tests: at item value: 500
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115] JNI ERROR (app bug): local reference table overflow (max=512)
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115] local reference table dump:
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]   Last 10 entries (of 511):
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       510: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       509: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       508: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       507: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       506: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       505: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       504: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       503: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       502: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       501: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]   Summary:
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]       507 of java.lang.Class (3 unique instances)
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]         1 of java.lang.String[] (3 elements)
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115]         3 of java.lang.String (3 unique instances)
09-14 21:12:04.030  9615  9615 F art     : art/runtime/indirect_reference_table.cc:115] 

```

## Minimum versions for test devices
* Android version 5.0; SDK 21, NDK 21
