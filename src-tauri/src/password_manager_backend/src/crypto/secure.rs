use core::fmt;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureMemory<T: Zeroize> {
    data: T,
}

impl<T: Zeroize> SecureMemory<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }

    /// Consumes the `SecureMemory` instance and returns the inner data.
    ///
    /// # Safety
    ///
    /// The caller becomes responsible for zeroizing the returned data after use.
    /// Failing to do so may lead to sensitive data lingering in memory.
    ///
    /// # Example
    ///
    /// ```
    /// let secure_mem = SecureMemory::new(sensitive_data);
    /// let data = unsafe { secure_mem.into_inner() };
    /// // Use `data` and then zeroize it when done
    /// data.zeroize();
    /// ```
    pub unsafe fn into_inner(self) -> T {
        let this = std::mem::ManuallyDrop::new(self);
        std::ptr::read(&this.data)
    }
}

impl<T: Zeroize> AsRef<T> for SecureMemory<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: Zeroize> AsMut<T> for SecureMemory<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T: Zeroize> fmt::Debug for SecureMemory<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecureMemory")
            .field("data", &"[REDACTED]")
            .finish()
    }
}
