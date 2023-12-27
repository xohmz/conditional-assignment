#![doc = include_str!("../README.md")]

pub trait Pick<O>: private::Sealed {
    fn pick(self, when_true: O, when_false: O) -> O;
    fn pick_lazy<P, N>(self, when_true: P, when_false: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O;
}

// Prevent this trait from being implemented outside of this crate.
// https://rust-lang.github.io/api-guidelines/future-proofing.html#c-sealed
mod private {
    pub trait Sealed {}
    impl Sealed for bool {}
}

impl<O> Pick<O> for bool {
    ///
    ///
    /// Example:
    /// ```
    /// use conditional_assignment::Pick;
    /// let condition = 0 < 1;
    /// let outcome = if condition {
    ///    "true"
    /// } else {
    ///    "false"
    /// };
    /// ```
    #[inline]
    fn pick(self: bool, when_true: O, when_false: O) -> O {
        if self {
            when_true
        } else {
            when_false
        }
    }

    ///
    ///
    /// Example:
    /// ```
    /// use conditional_assignment::Pick;
    /// let condition = 0 < 1;
    /// let outcome = condition.pick_lazy(
    ///     || {
    ///         assert!(condition);
    ///         "true"
    ///     },
    ///     || {
    ///         assert!(!condition);
    ///         "false"
    ///     },
    /// );
    /// ```
    #[inline]
    fn pick_lazy<P, N>(self: bool, when_true: P, when_false: N) -> O
    where
        P: FnOnce() -> O,
        N: FnOnce() -> O,
    {
        if self {
            when_true()
        } else {
            when_false()
        }
    }
}
