use std::sync::Arc;

type ThunkRef = Arc<Thunk>;

pub struct Args {}

impl Args {
    pub fn new() -> Args {
        return Args {};
    }
}

pub enum Thunk {
    app { func: ThunkRef, args: Args },
    num(f64),
}

impl Thunk {
    pub fn newApp(func: ThunkRef, args: Args) -> ThunkRef {
        return Arc::new(Thunk::app { func, args });
    }

    pub fn newNum(n: f64) -> ThunkRef {
        return Arc::new(Thunk::num(n));
    }
}
