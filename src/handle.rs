use tokio::runtime::EnterGuard;

// The trait MyDrop is important to make sure the Drop::drop is tied to where it got entered(in dylib)
// otherwise the main executable will try to call the main EnterGuard handler
trait MyDrop<'a> {}
impl<'a> MyDrop<'a> for EnterGuard<'a> {}
pub struct TokioEnterGuard<'a> {
    _guard: Box<dyn MyDrop<'a> + 'a>,
}

impl<'a> TokioEnterGuard<'a> {
    pub(crate) fn new(guard: EnterGuard<'a>) -> Self {
        let guard = Box::new(guard) as Box<dyn MyDrop<'a> + 'a>;

        Self { _guard: guard }
    }
}
