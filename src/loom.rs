#[cfg(not(all(test, loom)))]
pub(crate) mod sync {
    pub(crate) mod atomic {
        #[cfg(not(feature = "use-atomic-polyfill"))]
        pub(crate) use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

        #[cfg(feature = "use-atomic-polyfill")]
        pub(crate) use atomic_polyfill::{AtomicPtr, AtomicUsize, Ordering};

        pub(crate) trait AtomicMut<T> {
            fn with_mut<F, R>(&mut self, f: F) -> R
            where
                F: FnOnce(&mut *mut T) -> R;
        }

        impl<T> AtomicMut<T> for AtomicPtr<T> {
            fn with_mut<F, R>(&mut self, f: F) -> R
            where
                F: FnOnce(&mut *mut T) -> R,
            {
                f(self.get_mut())
            }
        }
    }
}

#[cfg(all(test, loom))]
pub(crate) mod sync {
    pub(crate) mod atomic {
        pub(crate) use loom::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

        pub(crate) trait AtomicMut<T> {}
    }
}
