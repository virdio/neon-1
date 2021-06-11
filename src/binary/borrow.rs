use crate::binary::{BorrowError, BorrowMutError, Ledger, Ref, RefMut};
use crate::context::Context;
use crate::types::JsArrayBuffer;

use super::Borrow;
use crate::handle::Managed;

impl<C> Borrow<C, u8> for JsArrayBuffer
where
    for<'env> C: Context<'env>,
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
