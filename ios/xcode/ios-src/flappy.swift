//
//  flappy.swift
//  flappy_bevy
//
//  Created by SY L on 8/27/24.
//

import Foundation
import GoogleMobileAds

@_cdecl("ffi_kv_get")
public func ffi_kv_get(key: UnsafePointer<CChar>) -> UnsafePointer<CChar>? {
    let keyString = String(cString: key)
    let value = UserDefaults.standard.string(forKey: keyString) ?? ""
    return (value as NSString).utf8String
}

@_cdecl("ffi_kv_set")
public func ffi_kv_set(key: UnsafePointer<CChar>, val: UnsafePointer<CChar>) {
    let keyString = String(cString: key)
    let valString = String(cString: val)
    UserDefaults.standard.set(valString, forKey: keyString);
}

@_cdecl("ffi_greet_to_swift")
public func ffi_greet_to_swift() {
    ffi_greet_to_rust();
}

let myAd = MyAd();

@_cdecl("ffi_ad_init")
public func ffi_ad_init() {
    GADMobileAds.sharedInstance().start() { _ in
        print("init end");
        myAd.load();
    }
}

@_cdecl("ffi_show_ad")
public func ffi_show_ad() {
    myAd.show();
}

class MyAd: NSObject, GADFullScreenContentDelegate {
    private var interstitial: GADInterstitialAd?
    
    func load() {
        Task {
            do {
              interstitial = try await GADInterstitialAd.load(
                withAdUnitID: "ca-app-pub-3940256099942544/4411468910", request: GADRequest())
              interstitial?.fullScreenContentDelegate = self;
            } catch {
              print("Failed to load interstitial ad with error: \(error.localizedDescription)")
            }
        }
    }
    func show() {
        print("show");
        guard let interstitial = interstitial else {
          return print("Ad wasn't ready.")
        }

        // The UIViewController parameter is an optional.
        interstitial.present(fromRootViewController: nil)
    }
    
    /// Tells the delegate that the ad failed to present full screen content.
     func ad(_ ad: GADFullScreenPresentingAd, didFailToPresentFullScreenContentWithError error: Error) {
       print("Ad did fail to present full screen content.")
     }

     /// Tells the delegate that the ad will present full screen content.
     func adWillPresentFullScreenContent(_ ad: GADFullScreenPresentingAd) {
       print("Ad will present full screen content.")
     }

     /// Tells the delegate that the ad dismissed full screen content.
     func adDidDismissFullScreenContent(_ ad: GADFullScreenPresentingAd) {
       print("Ad did dismiss full screen content.")
         self.interstitial = nil;
         self.load()
         ffi_ad_dismiss();
     }
}

@_cdecl("ffi_rwh_test")
public func ffi_rwh_test(vc: UIViewController) {
    print("vc: \(vc)")
    let actionSheet = UIAlertController(title: "Choose Option", message: nil, preferredStyle: .actionSheet)
    actionSheet.addAction(UIAlertAction(title: "Option 1", style: .default, handler: nil))
    actionSheet.addAction(UIAlertAction(title: "Option 2", style: .default, handler: nil))
    actionSheet.addAction(UIAlertAction(title: "Cancel", style: .cancel, handler: nil))
    vc.present(actionSheet, animated: true, completion: nil)
}
