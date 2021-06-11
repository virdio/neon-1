use std::slice;

use crate::binary::{BorrowError, BorrowMutError, Ledger, Ref, RefMut};
use crate::context::Context;
use crate::types::{JsArrayBuffer, JsTypedArray};

use super::Borrow;
use crate::handle::Managed;

impl<'env, C> Borrow<'env, C, u8> for JsArrayBuffer
where
    C: Context<'env>,
{
    fn try_borrow<'b, 'cx>(
        &self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<Ref<'b, 'cx, C, u8>, BorrowError> {
        let env = ledger.context().env().to_raw();
        let data = unsafe { neon_runtime::arraybuffer::as_mut_slice(env, self.to_raw()) };

        ledger.try_borrow_internal(data)
    }

    fn try_borrow_mut<'b, 'cx>(
        &mut self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<RefMut<'b, 'cx, C, u8>, BorrowMutError> {
        let env = ledger.context().env().to_raw();
        let data = unsafe { neon_runtime::arraybuffer::as_mut_slice(env, self.to_raw()) };

        ledger.try_borrow_mut_internal(data)
    }
}

impl<'env, C> Borrow<'env, C, u32> for JsTypedArray<u32>
where
    C: Context<'env>,
{
    fn try_borrow<'b, 'cx>(
        &self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<Ref<'b, 'cx, C, u32>, BorrowError> {
        let env = ledger.context().env().to_raw();
        let value = self.to_raw();
        let data = unsafe {
            let info = neon_runtime::typedarray::info(env, value);

            slice::from_raw_parts(info.data.cast(), info.length)
        };

        ledger.try_borrow_internal(data)
    }

    fn try_borrow_mut<'b, 'cx>(
        &mut self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<RefMut<'b, 'cx, C, u32>, BorrowMutError> {
        let env = ledger.context().env().to_raw();
        let value = self.to_raw();
        let data = unsafe {
            let info = neon_runtime::typedarray::info(env, value);

            slice::from_raw_parts_mut(info.data.cast(), info.length)
        };

        ledger.try_borrow_mut_internal(data)
    }
}
