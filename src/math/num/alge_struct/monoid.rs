use super::Semigroup;

/// [Monoid](https://en.wikipedia.org/wiki/Monoid) is an semigroup with identity element.
///
/// # Identity element
///
/// ~~~text
/// ∃ e ∈ Set, ∀ a ∈ Set, e ◦ a = a ◦ e = a
/// ~~~
#[codesnip::entry("Monoid", include("Semigroup"))]
pub trait Monoid: Semigroup {
    fn id() -> Self::Set;
}
