use std::cell::UnsafeCell;
use shrinkwraprs::Shrinkwrap;

#[repr(transparent)]
#[derive(Shrinkwrap)]
pub struct UnsafeCellWrapper<T>(pub UnsafeCell<T>);

impl<T> UnsafeCellWrapper<T> {
    pub fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }
}

impl<T: Clone> Clone for UnsafeCellWrapper<T> {
    fn clone(&self) -> Self {
        unsafe {
            Self(UnsafeCell::new((*self.0.get()).clone()))
        }
    }
}

unsafe impl<T: Sync + Send> Sync for UnsafeCellWrapper<T> {}