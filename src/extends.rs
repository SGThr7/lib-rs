#[cfg_attr(nightly, codesnip::entry("OrdEx"))]
pub mod ord;

#[cfg_attr(nightly, codesnip::entry("IteratorEx"))]
pub mod iterator;

mod for_codesnip {
    use super::*;

    #[codesnip::entry("OrdEx")]
    #[allow(unused_imports)]
    use ord::*;

    #[codesnip::entry("IteratorEx")]
    #[allow(unused_imports)]
    use iterator::*;
}
