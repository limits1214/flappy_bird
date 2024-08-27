//
//  flappy.swift
//  flappy_bevy
//
//  Created by SY L on 8/27/24.
//

import Foundation

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
