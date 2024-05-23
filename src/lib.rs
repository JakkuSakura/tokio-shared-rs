#![doc = include_str!("../README.md")]

use std::fmt::{Debug, Formatter};

pub use crate::handle::TokioEnterGuard;

mod handle;
fn get_data() -> u64 {
    get_features as *const () as u64
}

fn get_features() -> u16 {
    0
}

#[derive(Clone)]
#[repr(C)]
pub struct SharedTokioHandle {
    data: u64,
    features: u16,
    handle: ::tokio::runtime::Handle,
}
impl Debug for SharedTokioHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("SharedTokioHandle");

        dbg.field("data", &format_args!("{:p}", self.data as *const ()))
            .field("features", &self.features);
        dbg.finish()
    }
}
impl SharedTokioHandle {
    pub fn new() -> Self {
        SharedTokioHandle {
            data: get_data(),
            features: get_features(),
            handle: tokio::runtime::Handle::current(),
        }
    }

    #[inline(never)]
    pub fn enter(&self) -> TokioEnterGuard {
        TokioEnterGuard::new(self.handle.enter())
    }
}

#[no_mangle]
#[inline(never)]
pub fn setup_shared_tokio_ref(handle: &SharedTokioHandle) -> TokioEnterGuard {
    handle.enter()
}
