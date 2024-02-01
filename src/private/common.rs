use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

pub struct Newtype<T>(pub T);

pub struct UnsafeCellSendSync<T>(pub UnsafeCell<T>);

unsafe impl<T> Send for UnsafeCellSendSync<T> {}
unsafe impl<T> Sync for UnsafeCellSendSync<T> {}

#[derive(Debug)]
#[repr(transparent)]
pub struct PtrSendSync<T>(*mut T);

impl<T> Deref for PtrSendSync<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl<T> DerefMut for PtrSendSync<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.0 }
    }
}

impl<T> PtrSendSync<T> {
    pub fn new(ptr: *mut T) -> Self {
        PtrSendSync(ptr)
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.0
    }

    pub fn as_ptr(&self) -> *const T {
        self.0
    }
}

unsafe impl<T> Send for PtrSendSync<T> {}

unsafe impl<T> Sync for PtrSendSync<T> {}
