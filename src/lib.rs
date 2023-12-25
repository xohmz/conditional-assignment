#![doc = include_str!("../README.md")]

pub trait ConditionalAssignment<O> {
    fn pick(self, a: O, b: O) -> O;
}

impl<O> ConditionalAssignment<O> for bool {
    fn pick(self: bool, positive: O, negative: O) -> O {
        if self {
            positive
        } else {
            negative
        }
    }
}

impl<T, O> ConditionalAssignment<O> for Option<T> {
    fn pick(self: Option<T>, positive: O, negative: O) -> O {
        self.is_some().pick(positive, negative)
    }
}

impl<T, U, O> ConditionalAssignment<O> for Result<T, U> {
    fn pick(self: Result<T, U>, positive: O, negative: O) -> O {
        self.is_ok().pick(positive, negative)
    }
}
