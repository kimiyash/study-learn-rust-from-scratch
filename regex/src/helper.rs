pub trait SafeAdd: Sized {
    fn safe_add(&self, n: &Self) -> Option<Self>;
}

impl SafeAdd for usize {
    fn safe_add(&self, n: &Self) -> Option<Self> {
        self.check_add(*n)
    }
}

pub fn safe_add<T, F, E>(dst: &mut T, src: &T, f: F) -> Result<(), E>
    where
        T: SafeAdd,
        T: Fn() -> E,
{
    if let Some(n) = dst.safe_add(src) {
        *dst = n;
        Ok(())
    } else  {
        Err(f())
    }
}

pub type DynError = Box<dyn std::error::Error + Send + Sync + 'static>;
