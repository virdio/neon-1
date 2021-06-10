//! Types and traits representing binary JavaScript data.

use crate::context::internal::Env;
use crate::context::Context;
#[cfg(feature = "napi-1")]
use crate::handle::Handle;
use crate::handle::Managed;
use crate::result::JsResult;
use crate::types::internal::ValueInternal;
use crate::types::{build, Object, Value};
use neon_runtime;
use neon_runtime::raw;
use std::mem;

/// The Node [`Buffer`](https://nodejs.org/api/buffer.html) type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JsBuffer(raw::Local);

impl JsBuffer {
    /// Constructs a new `Buffer` object, safely zero-filled.
    pub fn new<'a, C: Context<'a>>(cx: &mut C, size: u32) -> JsResult<'a, JsBuffer> {
        let env = cx.env();
        build(env, |out| unsafe {
            neon_runtime::buffer::new(env.to_raw(), out, size)
        })
    }

    /// Constructs a new `Buffer` object, safely zero-filled.
    pub unsafe fn uninitialized<'a, C: Context<'a>>(
        cx: &mut C,
        size: u32,
    ) -> JsResult<'a, JsBuffer> {
        let env = cx.env();
        build(env, |out| {
            neon_runtime::buffer::uninitialized(env.to_raw(), out, size)
        })
    }

    #[cfg(feature = "napi-1")]
    /// Construct a new `Buffer` from bytes allocated by Rust
    pub fn external<'a, C, T>(cx: &mut C, data: T) -> Handle<'a, JsBuffer>
    where
        C: Context<'a>,
        T: AsMut<[u8]> + Send,
    {
        let env = cx.env().to_raw();
        let value = unsafe { neon_runtime::buffer::new_external(env, data) };

        Handle::new_internal(JsBuffer(value))
    }
}

impl Managed for JsBuffer {
    fn to_raw(self) -> raw::Local {
        self.0
    }

    fn from_raw(_env: Env, h: raw::Local) -> Self {
        JsBuffer(h)
    }
}

impl ValueInternal for JsBuffer {
    fn name() -> String {
        "Buffer".to_string()
    }

    fn is_typeof<Other: Value>(env: Env, other: Other) -> bool {
        unsafe { neon_runtime::tag::is_buffer(env.to_raw(), other.to_raw()) }
    }
}

impl Value for JsBuffer {}

impl Object for JsBuffer {}

/// The standard JS [`ArrayBuffer`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/ArrayBuffer) type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JsArrayBuffer(raw::Local);

impl JsArrayBuffer {
    /// Constructs a new `ArrayBuffer` object with the given size, in bytes.
    pub fn new<'a, C: Context<'a>>(cx: &mut C, size: u32) -> JsResult<'a, JsArrayBuffer> {
        build(cx.env(), |out| unsafe {
            neon_runtime::arraybuffer::new(out, mem::transmute(cx.env()), size)
        })
    }

    #[cfg(feature = "napi-1")]
    /// Construct a new `ArrayBuffer` from bytes allocated by Rust
    pub fn external<'a, C, T>(cx: &mut C, data: T) -> Handle<'a, JsArrayBuffer>
    where
        C: Context<'a>,
        T: AsMut<[u8]> + Send,
    {
        let env = cx.env().to_raw();
        let value = unsafe { neon_runtime::arraybuffer::new_external(env, data) };

        Handle::new_internal(JsArrayBuffer(value))
    }
}

impl Managed for JsArrayBuffer {
    fn to_raw(self) -> raw::Local {
        self.0
    }

    fn from_raw(_env: Env, h: raw::Local) -> Self {
        JsArrayBuffer(h)
    }
}

impl ValueInternal for JsArrayBuffer {
    fn name() -> String {
        "ArrayBuffer".to_string()
    }

    fn is_typeof<Other: Value>(env: Env, other: Other) -> bool {
        unsafe { neon_runtime::tag::is_arraybuffer(env.to_raw(), other.to_raw()) }
    }
}

impl Value for JsArrayBuffer {}

impl Object for JsArrayBuffer {}
