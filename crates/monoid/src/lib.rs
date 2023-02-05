pub mod types;

/// [Semigroup](https://en.wikipedia.org/wiki/Semigroup) is an algebraic structure consisting of a set together with an associative binary operation.
///
/// # Associativity
///
/// A binary operation `∘` on a `Set` is called associative if it satisfies the following property:
///
/// ~~~text
/// ∀ a, b, c ∈ Set, (a ∘ b) ∘ c = a ∘ (b ∘ c)
/// ~~~
pub trait Semigroup {
    type Set;

    /// A binary operator that satisfy [associativity](https://en.wikipedia.org/wiki/Associative_property).
    fn operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
}

/// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
///
/// # Identity element
///
/// An element `e` of a `Set` is called an identity element if satisfies the following property:
///
/// ~~~text
/// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
/// ~~~
pub trait Monoid: Semigroup {
    /// The element that unchanged every elements on binary operation.
    fn id() -> Self::Set;
}

/// [`PartialGroup`] is a [`Monoid`] with inverse operation.
/// It is implemented inverse operation but not necessary inverse element in the `Set`.
///
/// # Inverse operation
///
/// A binary operation `◦⁻¹` on a `Set` is called inverse operation if it satisfies the following property:
///
/// ~~~text
/// ∀ a, b ∈ Set, a ◦⁻¹ a ◦ b = b ◦ a ◦⁻¹ a = b
/// ~~~
pub trait PartialGroup: Monoid {
    fn inverse_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set;
}

/// [Group](https://en.wikipedia.org/wiki/Group_(mathematics)) is a [`Monoid`] with inverse element.
///
/// It is also a [`PartialGroup`] with inverse operation.
///
/// # Inverse element
///
/// An element `a⁻¹` of a `Set` is called an inverse element of `a` if it satisfies the following property:
///
/// ~~~text
/// ∀ a ∈ Set, ∃ a⁻¹ ∈ Set, a ◦ a⁻¹ = a⁻¹ ◦ a = e
/// ~~~
pub trait MathGroup: PartialGroup {
    /// The inverse element of `self`.
    fn inverse(x: &Self::Set) -> Self::Set;
}
