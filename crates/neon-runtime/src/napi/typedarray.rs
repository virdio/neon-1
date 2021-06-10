use std::ffi::c_void;
use std::mem::MaybeUninit;

use crate::napi::bindings::{self as napi, TypedArrayType};
use crate::raw::{Env, Local};

pub struct TypedArrayInfo {
    pub typ: TypedArrayType,
    pub length: usize,
    pub data: *mut c_void,
    pub buf: Local,
    pub offset: usize,
}

pub unsafe fn info(env: Env, value: Local) -> TypedArrayInfo {
    let mut info = MaybeUninit::<TypedArrayInfo>::zeroed();
    let ptr = info.as_mut_ptr();

    assert_eq!(
        napi::get_typedarray_info(
            env,
            value,
            &mut (*ptr).typ,
            &mut (*ptr).length,
            &mut (*ptr).data,
            &mut (*ptr).buf,
            &mut (*ptr).offset,
        ),
        napi::Status::Ok,
    );

    info.assume_init()
}
