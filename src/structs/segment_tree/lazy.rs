use crate::math::num::Monoid;

pub mod lst_monoid;
pub use lst_monoid::LSTMonoid;

#[codesnip::entry("LazySegTree")]
pub use lazy_segtree::LazySegTree;

#[codesnip::entry("LazySegTree", include("LSTMonoid", "Monoid"))]
mod lazy_segtree {
    use super::{LSTMonoid, Monoid};
    use core::{
        convert::TryInto,
        fmt::{Debug, Display, Formatter, Result},
        marker::PhantomData,
        ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    };

    pub struct LazySegTree<M: Monoid, LM: LSTMonoid<M>> {
        len: usize,
        depth: usize,
        tree: Vec<M::Set>,
        lazy: Vec<Option<M::Set>>,
        lst_monoid: PhantomData<LM>,
    }

    /// if i == 0 { 0 } else { log2(i) }
    fn bit(i: usize) -> usize {
        (64 - (i as u64).leading_zeros()) as usize
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LazySegTree<M, LM> {
        pub fn new(min_len: usize) -> Self {
            Self::init(min_len, &vec![])
        }

        fn init(min_len: usize, v: &[M::Set]) -> Self {
            let min_len = min_len.max(v.len());
            let depth: u64 = min_len.saturating_sub(1).try_into().ok().unwrap();
            let depth = 64 - depth.leading_zeros();
            let depth: usize = depth.try_into().ok().unwrap();
            // 2^{depth}
            let len = 1 << depth;
            let size = 2 * len - 1;
            let mut tree = vec![M::id(); size];
            let lazy = vec![LM::id_act(); size];

            // clone
            tree[len - 1..len - 1 + v.len()].clone_from_slice(v);

            // update
            for i in (0..len - 1).rev() {
                let lhs = &tree[i * 2 + 1];
                let rhs = &tree[i * 2 + 2];
                tree[i] = M::operate(lhs, rhs);
            }

            Self {
                len,
                depth,
                tree,
                lazy,
                lst_monoid: PhantomData,
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn depth(&self) -> usize {
            self.depth
        }

        pub fn range(&self, index: usize) -> usize {
            let log2: u64 = (index + 1).try_into().ok().unwrap();
            let log2 = 64 - log2.leading_zeros() - 1;
            let log2: usize = log2.try_into().ok().unwrap();
            1 << (self.depth() - log2)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LazySegTree<M, LM> {
        fn eval(&mut self, i: usize) {
            if !LM::is_id_act(&self.lazy[i]) {
                let acter = &LM::drain(&mut self.lazy[i]);

                self.tree[i] = LM::act(&self.tree[i], acter, self.range(i));

                if i < self.len() - 1 {
                    let left = &mut self.lazy[i * 2 + 1];
                    *left = LM::merge_act(left, acter);

                    let right = &mut self.lazy[i * 2 + 2];
                    *right = LM::merge_act(right, acter);
                }
            }
        }

        pub fn get<I: LSTIndex<M, LM>>(&mut self, index: I) -> Option<M::Set> {
            LSTIndex::get(index, self)
        }

        pub unsafe fn get_unchecked<I: LSTIndex<M, LM>>(&mut self, index: I) -> M::Set {
            LSTIndex::get_unchecked(index, self)
        }

        pub fn operate<I: LSTIndex<M, LM>, T: Into<Option<M::Set>>>(&mut self, index: I, value: T) {
            LSTIndex::operate(index, self, &value.into())
        }

        pub fn tree_index(&self) -> impl Iterator<Item = usize> {
            fn wshr(lhs: usize, rhs: u32) -> usize {
                lhs.checked_shr(rhs).unwrap_or(0)
            }
            let depth = self.depth();
            (0..(1 << depth))
                .map(move |i: usize| {
                    let tz = i.trailing_zeros();
                    wshr(1_usize << depth >> 1, tz) + wshr(i >> 1, tz)
                })
                .map(move |i| {
                    core::iter::successors(Some(i * 2), |k| Some(k * 2 + 1))
                        .take(depth + 1 - bit(i))
                })
                .flatten()
        }
    }

    pub trait LSTIndex<M: Monoid, LM: LSTMonoid<M>> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set>;
        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set;
        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>);
    }

    fn get_unchecked_inner<M: Monoid, LM: LSTMonoid<M>>(
        lst: &mut LazySegTree<M, LM>,
        range: &Range<usize>,
        i: usize,
        l: usize,
        r: usize,
    ) -> M::Set {
        if range.contains(&l) && (range.start..=range.end).contains(&r) {
            // if `l..r` in `range`
            lst.eval(i);
            lst.tree[i].clone()
        } else if (..range.end).contains(&l) && (..r).contains(&range.start) {
            // if `range` and `l..r` have a common range
            lst.eval(i);
            let mid = l + (r - l) / 2;
            let left = get_unchecked_inner(lst, range, i * 2 + 1, l, mid);
            let right = get_unchecked_inner(lst, range, i * 2 + 2, mid, r);
            M::operate(&left, &right)
        } else {
            // if `l..r` out of `range`
            M::id()
        }
    }

    #[codesnip::skip]
    #[allow(dead_code)]
    fn get_non_recursive<M: Monoid, LM: LSTMonoid<M>>(
        lst: &mut LazySegTree<M, LM>,
        range: Range<usize>,
    ) -> M::Set {
        let mut lacc = M::id();
        let mut racc = M::id();
        let mut left = range.start + lst.len() - 1;
        let mut right = range.end + lst.len() - 1;

        while left < right {
            if left & 1 == 0 {
                lst.eval(left);
                lacc = M::operate(&lacc, &lst.tree[left]);
            }
            if right & 1 == 0 {
                right -= 1;
                lst.eval(right);
                racc = M::operate(&lst.tree[right], &racc);
            }
            left >>= 1;
            right >>= 1;
        }
        M::operate(&lacc, &racc)
    }

    fn operate_inner<M: Monoid, LM: LSTMonoid<M>>(
        lst: &mut LazySegTree<M, LM>,
        range: &Range<usize>,
        val: &Option<M::Set>,
        i: usize,
        l: usize,
        r: usize,
    ) {
        if range.contains(&l) && (range.start..=range.end).contains(&r) {
            // if `l..r` in `range`
            lst.lazy[i] = LM::merge_act(&lst.lazy[i], &val);
            lst.eval(i);
        } else if (..range.end).contains(&l) && (..r).contains(&range.start) {
            // if `range` and `l..r` have a common range
            lst.eval(i);
            let mid = l + (r - l) / 2;
            let li = i * 2 + 1;
            let ri = i * 2 + 2;
            operate_inner(lst, range, val, li, l, mid);
            operate_inner(lst, range, val, ri, mid, r);
            lst.tree[i] = M::operate(&lst.tree[li], &lst.tree[ri]);
        }
    }

    #[codesnip::skip]
    #[allow(dead_code)]
    fn operate_non_recursive<M: Monoid, LM: LSTMonoid<M>>(
        lst: &mut LazySegTree<M, LM>,
        range: Range<usize>,
        value: &Option<M::Set>,
    ) {
        let mut left = range.start + lst.len() - 1;
        let mut right = range.end + lst.len() - 1;

        while left < right {
            if left & 1 == 0 {
                lst.eval(left);
                lst.lazy[left] = LM::merge_act(&lst.lazy[left], value);
            }
            if right & 1 == 0 {
                right -= 1;
                lst.eval(right);
                lst.lazy[right] = LM::merge_act(&lst.lazy[right], value);
            }

            left >>= 1;
            right >>= 1;
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for Range<usize> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            if self.end <= lst.len() {
                unsafe { Some(self.get_unchecked(lst)) }
            } else {
                None
            }
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            get_unchecked_inner(lst, &self, 0, 0, lst.len())
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            assert!(self.end <= lst.len());
            operate_inner(lst, &self, value, 0, 0, lst.len())
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for RangeInclusive<usize> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(*self.start()..self.end() + 1, lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(*self.start()..self.end() + 1, lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(*self.start()..self.end() + 1, lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for RangeFull {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(0..lst.len(), lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(0..lst.len(), lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(0..lst.len(), lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for RangeFrom<usize> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(self.start..lst.len(), lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(self.start..lst.len(), lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(self.start..lst.len(), lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for RangeTo<usize> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(0..self.end, lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(0..self.end, lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(0..self.end, lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for RangeToInclusive<usize> {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(0..=self.end, lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(0..=self.end, lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(0..=self.end, lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> LSTIndex<M, LM> for usize {
        fn get(self, lst: &mut LazySegTree<M, LM>) -> Option<M::Set> {
            LSTIndex::get(self..=self, lst)
        }

        unsafe fn get_unchecked(self, lst: &mut LazySegTree<M, LM>) -> M::Set {
            LSTIndex::get_unchecked(self..=self, lst)
        }

        fn operate(self, lst: &mut LazySegTree<M, LM>, value: &Option<M::Set>) {
            LSTIndex::operate(self..=self, lst, value)
        }
    }

    impl<M: Monoid, LM: LSTMonoid<M>> From<Vec<M::Set>> for LazySegTree<M, LM> {
        fn from(v: Vec<M::Set>) -> Self {
            Self::init(v.len(), &v)
        }
    }

    impl<M, LM> Debug for LazySegTree<M, LM>
    where
        M: Monoid,
        M::Set: Debug,
        LM: LSTMonoid<M>,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for i in self.tree_index() {
                write!(f, "{:3}: ", i)?;

                let depth = bit(i + 1) - 1;
                for k in (1..depth).rev() {
                    if ((i + 1) >> k) & 1 == 0 {
                        f.write_str("│ ")?;
                    } else {
                        f.write_str("  ")?;
                    }
                }
                if depth > 0 {
                    if (i + 1) & 1 == 0 {
                        f.write_str("├─")?;
                    } else {
                        f.write_str("└─")?;
                    }
                }

                write!(f, "{:?}", self.tree[i])?;
                if let Some(lazy) = &self.lazy[i] {
                    write!(f, "[{:?}]", lazy)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    impl<M, LM> Display for LazySegTree<M, LM>
    where
        M: Monoid,
        M::Set: Display,
        LM: LSTMonoid<M>,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for i in self.tree_index() {
                write!(f, "{:3}: ", i)?;

                let depth = bit(i + 1) - 1;
                for k in (1..depth).rev() {
                    if ((i + 1) >> k) & 1 == 0 {
                        f.write_str("│ ")?;
                    } else {
                        f.write_str("  ")?;
                    }
                }
                if depth > 0 {
                    if (i + 1) & 1 == 0 {
                        f.write_str("├─")?;
                    } else {
                        f.write_str("└─")?;
                    }
                }

                write!(f, "{}", self.tree[i])?;
                if let Some(lazy) = &self.lazy[i] {
                    write!(f, "[{}]", lazy)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        lst_monoid::{LMAdd, LMRep},
        LSTMonoid, LazySegTree, Monoid,
    };
    use crate::math::num::alge_struct::monoid::{
        AddMonoid, BitAndMonoid, BitOrMonoid, BitXorMonoid, MaxMonoid, MinMonoid, MulMonoid,
    };
    use core::fmt::Debug;

    type LST = LazySegTree<AddMonoid<usize>, LMRep<AddMonoid<usize>>>;

    macro_rules! test_segtree {
        ($($name:ident: $lst_monoid:ident, $monoid:ident);* $(;)?) => {$(
            mod $name {
                use super::*;
                type Mono = $monoid<isize>;
                type LSTMono = $lst_monoid<Mono>;
                type Seg = LazySegTree<Mono, LSTMono>;

                #[test]
                fn from() {
                    let v = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 6];
                    let mut seg = Seg::from(v.clone());
                    check_segtree(&v, &mut seg);
                }

                #[test]
                fn operate() {
                    let len = 14;
                    let mut ans = vec![Mono::id(); len];
                    let mut segtree = Seg::new(len);
                    for i in 0..len {
                        for k in i..=len {
                            let val = Some(((k - i) % 7) as isize);
                            for m in i..k {
                                ans[m] = LSTMono::act(&ans[m], &val, 1);
                            }
                            segtree.operate(i..k, val);
                            check_segtree(&ans, &mut segtree);
                        }
                    }
                }
            }
        )*};
    }

    test_segtree! {
        replace_sum:  LMRep, AddMonoid;
        replace_prod: LMRep, MulMonoid;
        replace_max:  LMRep, MaxMonoid;
        replace_min:  LMRep, MinMonoid;
        replace_xor:  LMRep, BitXorMonoid;
        replace_or:   LMRep, BitOrMonoid;
        replace_and:  LMRep, BitAndMonoid;
        add_sum:  LMAdd, AddMonoid;
        add_prod: LMAdd, MulMonoid;
        add_max:  LMAdd, MaxMonoid;
        add_min:  LMAdd, MinMonoid;
        add_xor:  LMAdd, BitXorMonoid;
        add_or:   LMAdd, BitOrMonoid;
        add_and:  LMAdd, BitAndMonoid;
    }

    #[allow(dead_code)]
    fn check_segtree<M, LM>(ans: &[M::Set], segtree: &mut LazySegTree<M, LM>)
    where
        M: Monoid,
        M::Set: PartialEq + Debug,
        LM: LSTMonoid<M>,
    {
        let n = ans.len();
        // get for each
        for i in 0..n {
            assert_eq!(
                segtree.get(i).as_ref(),
                Some(&ans[i]),
                "i={}\n{:?}",
                i,
                segtree
            );
        }

        // get for each range
        for i in 0..n {
            for k in i..=n {
                let ans = ans[i..k].iter().fold(M::id(), |a, b| M::operate(&a, b));
                let r = segtree.get(i..k);
                assert_eq!(r.unwrap(), ans, "range: `{}..{}`\n{:?}", i, k, segtree);
            }
        }
    }

    #[test]
    fn tree_index() {
        fn stack(min_len: usize) -> Vec<usize> {
            let depth = (min_len as f64).log2().ceil() as usize;
            let len = 1 << depth;
            let mut que = std::collections::VecDeque::with_capacity(depth * 2);
            que.push_front(0);
            let mut ret = Vec::with_capacity(len << 1);
            while let Some(i) = que.pop_front() {
                ret.push(i);

                if i < len - 1 {
                    que.push_front(i * 2 + 2);
                    que.push_front(i * 2 + 1);
                }
            }
            ret
        }

        macro_rules! test {
            ($min_len:expr) => {
                assert_eq!(
                    LST::new($min_len).tree_index().collect::<Vec<_>>(),
                    stack($min_len)
                );
            };
        }

        test!(16);
        test!(20);
    }
}
