# Android Rust JNI tests

Test Android app built to stress jni-rs crate. Created to report a bug and propose a fix in a
Pull Request on github.

## How to build the Android application

1. Run the script `./rust/scripts/make-android-toolchains.sh` to generate the required armeabi
and i686 standalone Android NDK toolchains.
2. Cargo is pre-configured in `rust/.cargo/config`, compile the JNI library which the Android
app relies on. Run `rust/scripts/build-android-libs.sh` to do so.
3. Copy the .so files in the expected location by running `./rust/scripts/copy-libs-to-app.sh`
4. Build, install and launch the Android app with gradle or Android Studio.
In console, `./gradlew installDebug` will do the trick. The app will be listed in your launcher.
5. adb logcat output will show a crash with jni-rs 0.5.2 or no crash with my proposed fix.

## Proposed fix
* Test the proposed fix by swapping the jni dependency in `rust/Cargo.toml`.
* After this change, remember to rebuild and copy the JNI libs:
`rust/scripts/build-android-libs.sh` && `rust/scripts/copy-libs-to-app.sh` will do that for you.

### Corresponding commit:
https://github.com/supercurio/jni-rs/commit/b6de87479c2bad04878b6c012a269d82a31e0068

## Note
On Android 8.0, ART java runtime does generate a crash but without calling `env.delete_local_ref`, 
the iteration speed when walking though this example's array progressively slow downs.

## Typical crash output

```
I android_rust_jni_tests: array len: 200001
I android_rust_jni_tests: at item value: 0
I android_rust_jni_tests: at item value: 50
I android_rust_jni_tests: at item value: 100
I android_rust_jni_tests: at item value: 150
I android_rust_jni_tests: at item value: 200
I android_rust_jni_tests: at item value: 250
I android_rust_jni_tests: at item value: 300
I android_rust_jni_tests: at item value: 350
I android_rust_jni_tests: at item value: 400
I android_rust_jni_tests: at item value: 450
I android_rust_jni_tests: at item value: 500
F art     : art/runtime/indirect_reference_table.cc:115] JNI ERROR (app bug): local reference table overflow (max=512)
F art     : art/runtime/indirect_reference_table.cc:115] local reference table dump:
F art     : art/runtime/indirect_reference_table.cc:115]   Last 10 entries (of 511):
F art     : art/runtime/indirect_reference_table.cc:115]       510: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       509: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       508: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       507: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       506: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       505: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       504: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       503: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       502: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]       501: 0x12c85a30 java.lang.Class<supercurio.androidrustjnitests.Item>
F art     : art/runtime/indirect_reference_table.cc:115]   Summary:
F art     : art/runtime/indirect_reference_table.cc:115]       507 of java.lang.Class (3 unique instances)
F art     : art/runtime/indirect_reference_table.cc:115]         1 of java.lang.String[] (3 elements)
F art     : art/runtime/indirect_reference_table.cc:115]         3 of java.lang.String (3 unique instances)
F art     : art/runtime/indirect_reference_table.cc:115] 

```

## Minimum versions for test devices or emulators
* Android version 5.0; SDK 21, NDK 21
