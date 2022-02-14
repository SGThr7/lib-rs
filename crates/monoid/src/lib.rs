pub mod types;

/// [Semigroup](https://en.wikipedia.org/wiki/Semigroup) is an algebraic structure consisting of a set together with an associative binary operation.
///
/// # Associativity
///
/// ~~~text
/// ∀ a, b, c ∈ Set, (a ∘ b) ∘ c = a ∘ (b ∘ c)
/// ~~~
pub trait Semigroup {
    type Set: Clone;

    /// A binary operator that satisfy [associativity](https://en.wikipedia.org/wiki/Associative_property).
    fn operate(lhs: Self::Set, rhs: Self::Set) -> Self::Set;
}

/// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
///
/// # Identity element
///
/// ~~~text
/// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
/// ~~~
pub trait Monoid: Semigroup {
    /// The element that unchanged every elements on binary operation.
    fn id() -> Self::Set;
}
