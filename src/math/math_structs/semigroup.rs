#[codesnip::entry("Semigroup")]
/// [Semigroup](https://en.wikipedia.org/wiki/Semigroup) is an algebraic structure consisting of a set together with an associative binary operation.
///
/// # Associativity
///
/// ~~~text
/// ∀ a, b, c ∈ Set, (a ∘ b) ∘ c = a ∘ (b ∘ c)
/// ~~~
pub trait Semigroup {
    type Set: Clone;

    /// It must be satisfy [associativity](https://en.wikipedia.org/wiki/Associative_property) `(a * b) * c = a * (b * c)`.
    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
}
