#![doc = include_str!("../README.md")]

pub trait ConditionalAssignment<O> {
    fn pick(self, positive: O, negative: O) -> O;
    fn pick_lazy<P, N>(self, positive: P, negative: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O;
}

impl<O> ConditionalAssignment<O> for bool {
    fn pick(self: bool, positive: O, negative: O) -> O {
        if self {
            positive
        } else {
            negative
        }
    }

    fn pick_lazy<P, N>(self: bool, positive: P, negative: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O,
    {
        if self {
            positive()
        } else {
            negative()
        }
    }
}

impl<T, O> ConditionalAssignment<O> for Option<T> {
    fn pick(self: Option<T>, positive: O, negative: O) -> O {
        self.is_some().pick(positive, negative)
    }

    fn pick_lazy<P, N>(self: Option<T>, positive: P, negative: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O,
    {
        self.is_some().pick_lazy(positive, negative)
    }
}

impl<T, U, O> ConditionalAssignment<O> for Result<T, U> {
    fn pick(self: Result<T, U>, positive: O, negative: O) -> O {
        self.is_ok().pick(positive, negative)
    }

    fn pick_lazy<P, N>(self: Result<T, U>, positive: P, negative: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O,
    {
        self.is_ok().pick_lazy(positive, negative)
    }
}
