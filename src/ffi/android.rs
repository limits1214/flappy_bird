use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv, JavaVM,
};
use jni_fn::jni_fn;

use crate::events::ffi::FfiEvent;

use super::{Ffi, FfiAd, FfiGreet, FfiKv, SENDER};

impl FfiKv for Ffi {
    fn get(key: &str) -> String {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        let result = env
            .call_method(
                ctx,
                "ffiKvGet",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[keyjval],
            )
            .unwrap()
            .l()
            .unwrap();
        let jstr = JString::from(result);
        env.get_string(&jstr).unwrap().into()
    }

    fn set(key: &str, val: &str) {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        let valjstr = env.new_string(val).unwrap();
        let valjobj = JObject::from(valjstr);
        let valjval = JValue::from(&valjobj);
        env.call_method(
            ctx,
            "ffiKvSet",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[keyjval, valjval],
        )
        .unwrap();
    }
}

impl FfiGreet for Ffi {
    fn greet() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffi_greet_to_android", "()V", &[])
            .unwrap();
    }
}

impl FfiAd for Ffi {
    fn show() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdShow", "()V", &[]).unwrap();
    }
}

#[jni_fn("xyz.lsy969999.flappy_bevy.RustBinding")]
pub fn ffi_greet_to_rust(_: JNIEnv, _: JClass) {
    SENDER.get().unwrap().send(FfiEvent::Greet);
}

#[jni_fn("xyz.lsy969999.flappy_bevy.RustBinding")]
pub fn ffi_ad_dismiss(_: JNIEnv, _: JClass) {
    SENDER.get().unwrap().send(FfiEvent::AdDismiss);
}
