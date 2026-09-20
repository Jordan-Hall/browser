use std::ops::{Deref, DerefMut};
use zeroize::Zeroize;

pub(crate) struct ErasingBytes(Vec<u8>);

impl ErasingBytes {
    pub(crate) const fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Deref for ErasingBytes {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ErasingBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Debug for ErasingBytes {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ErasingBytes")
            .field("len", &self.0.len())
            .finish_non_exhaustive()
    }
}

impl Drop for ErasingBytes {
    fn drop(&mut self) {
        self.0.as_mut_slice().zeroize();
        #[cfg(test)]
        DISPOSALS.with(|events| {
            events
                .borrow_mut()
                .push((self.0.len(), self.0.iter().all(|byte| *byte == 0)))
        });
        self.0.zeroize();
    }
}

#[cfg(test)]
thread_local! {
    static DISPOSALS: std::cell::RefCell<Vec<(usize, bool)>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(crate) fn take_disposals() -> Vec<(usize, bool)> {
    DISPOSALS.with(|events| std::mem::take(&mut *events.borrow_mut()))
}
