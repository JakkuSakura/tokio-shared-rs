/// can simply re-export if linked as a cdylib, i.e. .so/.dll/.dylib
///
/// or linked as a dylib. it will be linked automatically on startup
pub use tokio_shared::setup_shared_tokio_ref;

#[no_mangle]
pub fn run(src: &str) {
    println!("{} println!", src);

    let src = src.to_string();
    tokio::task::spawn(async move { println!("{} println! from tokio task", src) });
}

pub mod exports {
    use tokio_shared::{SharedTokioHandle, TokioEnterGuard};

    pub type FnSetupTokio = fn(&SharedTokioHandle) -> TokioEnterGuard;
    pub type FnRun = fn(&str);
}
