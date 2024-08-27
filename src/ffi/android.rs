use jni::{
    objects::{JObject, JString, JValue},
    JavaVM,
};

use super::{Ffi, FfiKv};

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
