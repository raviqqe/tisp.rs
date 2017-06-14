use std::sync::Arc;

type ThunkRef = Arc<Thunk>;

pub struct Args {}

impl Args {
    pub fn new() -> Args {
        return Args {};
    }
}

pub enum Thunk {
    App { func: ThunkRef, args: Args },
    Num(f64),
}

impl Thunk {
    pub fn app(func: ThunkRef, args: Args) -> ThunkRef {
        return Arc::new(Thunk::App { func, args });
    }

    pub fn num(n: f64) -> ThunkRef {
        return Arc::new(Thunk::Num(n));
    }
}
