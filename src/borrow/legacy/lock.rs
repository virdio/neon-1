use std::cell::RefCell;
use std::marker::PhantomData;

use super::internal::Ledger;
use crate::context::internal::Env;

/// A temporary lock of an execution context.
///
/// While a lock is alive, no JavaScript code can be executed in the execution context.
///
/// Objects that support the `Borrow` and `BorrowMut` traits can be inspected while the context is locked by passing a reference to a `Lock` to their methods.
pub struct Lock<'a> {
    pub(crate) ledger: RefCell<Ledger>,
    pub(crate) env: Env,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Lock<'a> {
    pub(crate) fn new(env: Env) -> Self {
        Lock {
            ledger: RefCell::new(Ledger::new()),
            env,
            phantom: PhantomData,
        }
    }
}
