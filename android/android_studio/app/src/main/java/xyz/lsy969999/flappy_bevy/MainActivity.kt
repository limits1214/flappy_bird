package xyz.lsy969999.flappy_bevy

import android.app.NativeActivity
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle

class MainActivity : NativeActivity() {
    private lateinit var spf: SharedPreferences
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        spf = getSharedPreferences("test", Context.MODE_PRIVATE)
    }
    fun ffiKvGet(key: String): String {
        val aa = spf.getString(key, "");
        return aa.toString();
    }
    fun ffiKvSet(key: String, value: String) {
        val editor = spf.edit();
        editor.putString(key, value).apply();
    }
}