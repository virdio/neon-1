use std::marker::PhantomData;
use std::slice;

use neon_runtime::{raw, TypedArrayType};

use crate::context::internal::Env;
use crate::context::Context;
use crate::handle::Managed;
use crate::object::Object;
use crate::types::internal::ValueInternal;
use crate::types::{JsArrayBuffer, JsBuffer, Value};

/// The standard JS [`TypedArray`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/TypedArray) type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JsTypedArray<T> {
    local: raw::Local,
    _type: PhantomData<T>,
}

impl<T> JsTypedArray<T>
where
    Self: Managed,
{
    pub fn as_slice<'a: 'b, 'b, C>(&'b self, cx: &'b C) -> &'b [T]
    where
        C: Context<'a>,
    {
        let env = cx.env().to_raw();
        let value = self.to_raw();

        unsafe {
            let info = neon_runtime::typedarray::info(env, value);

            slice::from_raw_parts(info.data.cast(), info.length)
        }
    }

    pub fn as_mut_slice<'a: 'b, 'b, C>(&'b mut self, cx: &'b mut C) -> &'b mut [T]
    where
        C: Context<'a>,
    {
        let env = cx.env().to_raw();
        let value = self.to_raw();

        unsafe {
            let info = neon_runtime::typedarray::info(env, value);

            slice::from_raw_parts_mut(info.data.cast(), info.length)
        }
    }
}

impl<T> Value for JsTypedArray<T> where Self: ValueInternal {}

impl<T: Copy> Object for JsTypedArray<T> where Self: Value {}

impl<T: Copy> Managed for JsTypedArray<T> {
    fn to_raw(self) -> raw::Local {
        self.local
    }

    fn from_raw(_env: Env, local: raw::Local) -> Self {
        Self {
            local,
            _type: PhantomData,
        }
    }
}

macro_rules! impl_typed_array_value_internal {
    ($name:expr, $typ:ty, $($pattern:pat)|+) => {
        impl ValueInternal for JsTypedArray<$typ> {
            fn name() -> String {
                $name.to_string()
            }

            fn is_typeof<Other: Value>(env: Env, other: Other) -> bool {
                let env = env.to_raw();
                let other = other.to_raw();

                if unsafe { !neon_runtime::tag::is_typedarray(env, other) } {
                    return false;
                }

                let info = unsafe { neon_runtime::typedarray::info(env, other) };

                matches!(info.typ, $($pattern)|+)
            }
        }
    };
}

impl_typed_array_value_internal!("Int8Array", i8, TypedArrayType::I8);
impl_typed_array_value_internal!(
    "Uint8Array",
    u8,
    TypedArrayType::U8 | TypedArrayType::U8Clamped
);
impl_typed_array_value_internal!("Int16Array", i16, TypedArrayType::I16);
impl_typed_array_value_internal!("Uint16Array", u16, TypedArrayType::U16);
impl_typed_array_value_internal!("Int32Array", i32, TypedArrayType::I32);
impl_typed_array_value_internal!("Uint32Array", u32, TypedArrayType::U32);
impl_typed_array_value_internal!("Float32Array", f32, TypedArrayType::F32);
impl_typed_array_value_internal!("Float64Array", f64, TypedArrayType::F64);
impl_typed_array_value_internal!("BigInt64Array", i64, TypedArrayType::I64);
impl_typed_array_value_internal!("BigUint64Array", u64, TypedArrayType::U64);

impl JsBuffer {
    pub fn as_slice<'a: 'b, 'b, C>(&'b self, cx: &'b C) -> &'b [u8]
    where
        C: Context<'a>,
    {
        unsafe { neon_runtime::buffer::as_mut_slice(cx.env().to_raw(), self.to_raw()) }
    }

    pub fn as_mut_slice<'a: 'b, 'b, C>(&'b mut self, cx: &'b mut C) -> &'b mut [u8]
    where
        C: Context<'a>,
    {
        unsafe { neon_runtime::buffer::as_mut_slice(cx.env().to_raw(), self.to_raw()) }
    }
}

impl JsArrayBuffer {
    pub fn as_slice<'a: 'b, 'b, C>(&'b self, cx: &'b C) -> &'b [u8]
    where
        C: Context<'a>,
    {
        unsafe { neon_runtime::arraybuffer::as_mut_slice(cx.env().to_raw(), self.to_raw()) }
    }

    pub fn as_mut_slice<'a: 'b, 'b, C>(&'b mut self, cx: &'b mut C) -> &'b mut [u8]
    where
        C: Context<'a>,
    {
        unsafe { neon_runtime::arraybuffer::as_mut_slice(cx.env().to_raw(), self.to_raw()) }
    }
}
