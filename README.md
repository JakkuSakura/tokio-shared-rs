# tokio-shared-rs

Share a tokio Runtime between a dylib/cdylib and the main binary

Caution: cdylib is not properly supported yet

## Usage

```toml
[dependencies]
tokio-shared = "0.1"

```

checkout examples/example.rs

### dylib's case

```rust
use tokio_shared::SharedTokioHandle;

fn main() {
    let handle = SharedTokioHandle::new();
    let _guard = example_lib::setup_shared_tokio_ref(&handle);
    example_lib::run("dylib");
}
```

### cdylib's case

```rust
use tokio_shared::SharedTokioHandle;

fn main() {
    let dylib = unsafe { libloading::Library::new(dylib) }.expect("error loading dylib");
    let setup_tokio: FnSetupTokio = unsafe { *dylib.get(b"setup_shared_tokio_ref").unwrap() };
    let _guard = setup_tokio(&logger);
    let run: FnRun = unsafe { *dylib.get(b"run").unwrap() };
    run("cdylib");
}
```

