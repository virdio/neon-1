use std::cell::RefCell;
use std::ffi::c_void;
use std::fmt::{self, Debug, Display};
use std::ops::{Deref, Range};

use crate::context::Context;

pub struct Ledger<'cx, C> {
    cx: &'cx mut C,
    owned: RefCell<Vec<Range<*const c_void>>>,
    shared: RefCell<Vec<Range<*const c_void>>>,
}

impl<'cx, C> Ledger<'cx, C> {
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
    ) -> Result<Ref<'a, C, T>, BorrowError> {
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
    ) -> Result<RefMut<'a, C, T>, BorrowMutError> {
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

pub trait Borrow<'env, C, T>
where
    C: Context<'env>,
{
    fn try_borrow<'b>(
        &self,
        ledger: &'b Ledger<'b, C>,
    ) -> Result<Ref<'b, C, T>, BorrowError>;

    fn try_borrow_mut<'b, 'cx>(
        &mut self,
        ledger: &'b Ledger<'b, C>,
    ) -> Result<RefMut<'b, C, T>, BorrowMutError>;
}

pub struct Ref<'a, C, T> {
    ledger: &'a Ledger<'a, C>,
    data: &'a [T],
}

impl<'a, C, T> Deref for Ref<'a, C, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, C, T> Drop for Ref<'a, C, T> {
    fn drop(&mut self) {
        let range = self.data.as_ptr_range();
        let range = range.start.cast::<c_void>()..range.end.cast();
        let mut shared = self.ledger.shared.borrow_mut();
        let pos = shared.iter().position(move |item| item == &range);

        shared.remove(pos.unwrap());
    }
}

pub struct RefMut<'a, C, T> {
    ledger: &'a Ledger<'a, C>,
    data: &'a mut [T],
}

impl<'a, C, T> Drop for RefMut<'a, C, T> {
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
