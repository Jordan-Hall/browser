use crate::ByteSize;

impl ByteSize {
    /// Check the signed integer domain used by persistence without truncation.
    pub fn try_bytes_i64(self) -> Result<i64, std::num::TryFromIntError> {
        i64::try_from(self.as_bytes())
    }

    /// Check the current platform's address-sized domain, not available memory.
    /// Callers must still apply their own allocation and resource budgets.
    pub fn try_bytes_usize(self) -> Result<usize, std::num::TryFromIntError> {
        usize::try_from(self.as_bytes())
    }
}
