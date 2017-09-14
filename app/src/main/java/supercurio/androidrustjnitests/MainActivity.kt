package supercurio.androidrustjnitests

import android.app.Activity
import android.os.Bundle

class MainActivity : Activity() {

    external fun nativeTest(items: Array<Item>)

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        System.loadLibrary("android_rust_jni_tests")

        val list = (0..200_000).map { Item(it) }

        nativeTest(list.toTypedArray())
    }
}

data class Item(val value: Int)
