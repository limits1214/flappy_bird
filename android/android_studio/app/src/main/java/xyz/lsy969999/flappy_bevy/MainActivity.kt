package xyz.lsy969999.flappy_bevy

import android.app.Activity
import android.app.NativeActivity
import android.content.Context
import android.content.SharedPreferences
import android.os.Bundle
import android.util.Log
import com.google.android.gms.ads.AdError
import com.google.android.gms.ads.AdRequest
import com.google.android.gms.ads.FullScreenContentCallback
import com.google.android.gms.ads.LoadAdError
import com.google.android.gms.ads.MobileAds
import com.google.android.gms.ads.interstitial.InterstitialAd
import com.google.android.gms.ads.interstitial.InterstitialAdLoadCallback

class MainActivity : NativeActivity() {
    private lateinit var spf: SharedPreferences
    private lateinit var myad: MyAd
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        spf = getSharedPreferences("test", Context.MODE_PRIVATE)
        MobileAds.initialize(this)
        myad = MyAd()
        myad.load(this)
    }
    fun ffiKvGet(key: String): String {
        val aa = spf.getString(key, "");
        return aa.toString();
    }
    fun ffiKvSet(key: String, value: String) {
        val editor = spf.edit();
        editor.putString(key, value).apply();
    }
    fun ffi_greet_to_android() {
        // something
        RustBinding.ffi_greet_to_rust();
    }
    fun ffiAdShow() {
        runOnUiThread {
            this.myad.show(this);
        }
    }
}

class RustBinding {
    companion object {
        init {
            System.loadLibrary("android");
        }

        @JvmStatic
        external fun ffi_greet_to_rust();

        @JvmStatic
        external fun ffi_ad_dismiss();
    }
}

class MyAd {
    private var mInterstitialAd: InterstitialAd? = null
    private final val TAG = "MainActivity"
//    private var directShow = false;
    fun load(ctx: Activity) {
        ctx.runOnUiThread {
            val adRequest = AdRequest.Builder().build()
            InterstitialAd.load(ctx,"ca-app-pub-3940256099942544/1033173712", adRequest, object : InterstitialAdLoadCallback() {
                override fun onAdFailedToLoad(adError: LoadAdError) {
                    Log.d(TAG, "${adError?.toString()}")
                    mInterstitialAd = null
                }

                override fun onAdLoaded(interstitialAd: InterstitialAd) {
                    Log.d(TAG, "Ad was loaded.")
                    mInterstitialAd = interstitialAd
                    mInterstitialAd?.fullScreenContentCallback = object: FullScreenContentCallback() {
                        override fun onAdClicked() {
                            // Called when a click is recorded for an ad.
                            Log.d(TAG, "Ad was clicked.")
                        }

                        override fun onAdDismissedFullScreenContent() {
                            // Called when ad is dismissed.
                            Log.d(TAG, "Ad dismissed fullscreen content.")
                        mInterstitialAd = null
                        this@MyAd.load(ctx);
                        RustBinding.ffi_ad_dismiss()
                        }

                        override fun onAdImpression() {
                            // Called when an impression is recorded for an ad.
                            Log.d(TAG, "Ad recorded an impression.")
                        }

                        override fun onAdShowedFullScreenContent() {
                            // Called when ad is shown.
                            Log.d(TAG, "Ad showed fullscreen content.")
                        }
                    }

//                if (directShow) {
//                    this@MyAd.show(ctx as Activity);
//                    this@MyAd.directShow = false;
//                }
                }
            })
        }

    }

    fun show(activity: Activity) {
        if (mInterstitialAd != null) {
            mInterstitialAd?.show(activity)
        } else {
            Log.d("TAG", "The interstitial ad wasn't ready yet.")
//            this@MyAd.load(activity);
//            this.directShow = true;
        }
    }
}