use std::cell::RefCell;
use std::ffi::c_void;
use std::fmt::{self, Debug, Display};
use std::ops::Range;

use crate::context::Context;

pub struct Ledger<'cx, C> {
    cx: &'cx mut C,
    owned: RefCell<Vec<Range<*const c_void>>>,
    shared: RefCell<Vec<Range<*const c_void>>>,
}

impl<'env: 'cx, 'cx, C> Ledger<'cx, C>
where
    C: Context<'env>,
{
    pub fn new(cx: &'cx mut C) -> Self {
        Self {
            cx,
            owned: Default::default(),
            shared: Default::default(),
        }
    }

    pub(super) fn context(&self) -> &C {
        &self.cx
    }

    pub(super) fn try_borrow_internal<'a, T>(
        &'a self,
        data: &'a [T],
    ) -> Result<Ref<'cx, 'a, C, T>, BorrowError> {
        let range = data.as_ptr_range();
        let range = range.start.cast::<c_void>()..range.end.cast();
        let has_overlap = !self
            .owned
            .borrow()
            .iter()
            .all(|owned| range.end <= owned.start || range.start >= owned.end);

        if has_overlap {
            return Err(BorrowError { _private: () });
        }

        self.shared.borrow_mut().push(range);

        Ok(Ref { ledger: self, data })
    }

    pub(super) fn try_borrow_mut_internal<'a, T>(
        &'a self,
        data: &'a mut [T],
    ) -> Result<RefMut<'cx, 'a, C, T>, BorrowMutError> {
        let range = data.as_ptr_range();
        let range = range.start.cast::<c_void>()..range.end.cast();
        let has_overlap = !self
            .owned
            .borrow()
            .iter()
            .all(|owned| range.end <= owned.start || range.start >= owned.end);

        let has_overlap = has_overlap
            || !self
                .shared
                .borrow()
                .iter()
                .all(|shared| range.end <= shared.start || range.start >= shared.end);

        if has_overlap {
            return Err(BorrowMutError { _private: () });
        }

        self.owned.borrow_mut().push(range);

        Ok(RefMut { ledger: self, data })
    }
}

pub trait Borrow<C, T>
where
    for<'env> C: Context<'env>,
{
    fn try_borrow<'b, 'cx>(
        &self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<Ref<'b, 'cx, C, T>, BorrowError>;

    fn try_borrow_mut<'b, 'cx>(
        &mut self,
        ledger: &'b Ledger<'cx, C>,
    ) -> Result<RefMut<'b, 'cx, C, T>, BorrowMutError>;
}

pub struct Ref<'cx: 'a, 'a, C, T> {
    ledger: &'a Ledger<'cx, C>,
    data: &'a [T],
}

impl<'cx: 'a, 'a, C, T> Drop for Ref<'cx, 'a, C, T> {
    fn drop(&mut self) {
        let range = self.data.as_ptr_range();
        let range = range.start.cast::<c_void>()..range.end.cast();
        let mut shared = self.ledger.shared.borrow_mut();
        let pos = shared.iter().position(move |item| item == &range);

        shared.remove(pos.unwrap());
    }
}

pub struct RefMut<'cx: 'a, 'a, C, T> {
    ledger: &'a Ledger<'cx, C>,
    data: &'a mut [T],
}

impl<'cx: 'a, 'a, C, T> Drop for RefMut<'cx, 'a, C, T> {
    fn drop(&mut self) {
        let range = self.data.as_ptr_range();
        let range = range.start.cast::<c_void>()..range.end.cast();
        let mut owned = self.ledger.owned.borrow_mut();
        let pos = owned.iter().position(move |item| item == &range);

        owned.remove(pos.unwrap());
    }
}

/// An error returned by `try_borrow`
pub struct BorrowError {
    _private: (),
}

impl Debug for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BorrowError").finish()
    }
}

impl Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("already mutably borrowed", f)
    }
}

/// An error returned by `try_borrow_mut`
pub struct BorrowMutError {
    _private: (),
}

impl Debug for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BorrowMutError").finish()
    }
}

impl Display for BorrowMutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt("already borrowed", f)
    }
}
