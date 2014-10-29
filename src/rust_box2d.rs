#![license = "GPLv3"]

#![feature(macro_rules, globs, unsafe_destructor)]

extern crate libc;

pub use common::math;
pub use common::settings;

#[link(name = "Box2D")] extern {}
#[link(name = "stdc++")] extern {}

macro_rules! wrapped(
    ($wrapped:ty into custom $wrap:ident) => (        
        impl Wrapped<$wrapped> for $wrap {            
            unsafe fn ptr(&self) -> *const $wrapped {
                self.ptr as *const $wrapped
            }
            
            unsafe fn mut_ptr(&mut self) -> *mut $wrapped {
                self.ptr
            }
        }
    );
    
    ($wrapped:ty into $wrap:ident) => (
        wrapped!($wrapped into custom $wrap)
        
        pub struct $wrap {
            ptr: *mut $wrapped,
            mb_owned: MaybeOwned
        }
        
        impl BuildWrapped<$wrapped, MaybeOwned> for $wrap {
            unsafe fn with(ptr: *mut $wrapped, mb_owned: MaybeOwned) -> $wrap {
                assert!(!ptr.is_null())
                $wrap {
                    ptr: ptr,
                    mb_owned: mb_owned
                }
            }
        }
    );
        
    ($wrapped:ty into custom $wrap:ident with base $base:ty
     >> $as_base:path) => (
        
        wrapped!($wrapped into custom $wrap)
        
        impl WrappedBase<$base> for $wrap {            
            unsafe fn base_ptr(&self) -> *const $base {
                $as_base(self.ptr) as *const $base
            }
            
            unsafe fn mut_base_ptr(&mut self) -> *mut $base {
                $as_base(self.ptr)
            }
        }
    );
    
    ($wrapped:ty into $wrap:ident with base $base:ty
     << $base_as:path
     >> $as_base:path) => (
     
        wrapped!($wrapped into $wrap)
        
        impl WrappedBase<$base> for $wrap {            
            unsafe fn base_ptr(&self) -> *const $base {
                $as_base(self.ptr) as *const $base
            }
            
            unsafe fn mut_base_ptr(&mut self) -> *mut $base {
                $as_base(self.ptr)
            }
        }
        
        impl BuildWrappedBase<$base, MaybeOwned> for $wrap {
            unsafe fn with(ptr: *mut $base, mb_owned: MaybeOwned) -> $wrap {
                assert!(!ptr.is_null())
                $wrap {
                    ptr: $base_as(ptr),
                    mb_owned: mb_owned
                }
            }
        }
    );
)

pub mod b2 {
    pub use super::*;
    pub use super::common::*;
    pub use super::dynamics::*;
    pub use super::collision::*;
    pub use super::math::*;
    pub use super::settings::*;
}

mod ffi;
pub mod dynamics;
pub mod common;
pub mod collision;

#[deriving(PartialEq)]
pub enum MaybeOwned {
    Owned,
    NotOwned
}

pub trait Wrapped<T> {
    unsafe fn ptr(&self) -> *const T;
    unsafe fn mut_ptr(&mut self) -> *mut T;
}

pub trait WrappedBase<B> {
    unsafe fn base_ptr(&self) -> *const B;
    unsafe fn mut_base_ptr(&mut self) -> *mut B;
}

pub trait BuildWrapped<T, A> {
    unsafe fn with(ptr: *mut T, a: A) -> Self;
}

pub trait BuildWrappedBase<B, A> {
    unsafe fn with(ptr: *mut B, a: A) -> Self;
}

pub struct RefMut<'l, T> {
    object: T
}

impl<'l, T> RefMut<'l, T> {
    pub fn new(t: T) -> RefMut<'l, T> {
        RefMut { object: t }
    }
}

impl<'l, T> Deref<T> for RefMut<'l, T> {
    fn deref<'a>(&'a self) -> &'a T {
        &self.object
    }
}

impl<'l, T> DerefMut<T> for RefMut<'l, T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.object
    }
}

pub struct Ref<'l, T> {
    object: T
}

impl<'l, T> Ref<'l, T> {
    pub fn new(t: T) -> Ref<'l, T> {
        Ref { object: t }
    }
}

impl<'l, T> Deref<T> for Ref<'l, T> {
    fn deref<'a>(&'a self) -> &'a T {
        &self.object
    }
}
