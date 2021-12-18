use super::Monoid;

/// [Group (mathematics)](https://en.wikipedia.org/wiki/Group_(mathematics)) is an monoid with inverse element.
///
/// # Inverse element
///
/// When `e` is identity element,
///
/// ~~~text
/// ∀ a ∈ Set, ∃ b ∈ Set, a ◦ b = b ◦ a = e
/// ~~~
#[codesnip::entry(include("Monoid"))]
pub trait Group: Monoid {
    fn inv(x: &Self::Set) -> Self::Set;
    fn inv_operate(lhs: &Self::Set, rhs: &Self::Set) -> Self::Set {
        Self::operate(lhs, &Self::inv(rhs))
    }
}
