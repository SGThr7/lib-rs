use super::Monoid;

pub trait LSTMonoid<M: Monoid> {
    fn id_act() -> Option<M::Set> {
        None
    }

    fn is_id_act(act: &Option<M::Set>) -> bool {
        act.is_none()
    }

    fn drain(acter: &mut Option<M::Set>) -> Option<M::Set> {
        let mut y = Self::id_act();
        core::mem::swap(acter, &mut y);
        y
    }

    fn act(set: &M::Set, acter: &Option<M::Set>, range: usize) -> M::Set;

    fn merge_act(lhs: &Option<M::Set>, rhs: &Option<M::Set>) -> Option<M::Set>;
}
