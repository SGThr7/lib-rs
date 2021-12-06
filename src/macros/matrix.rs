#[cfg_attr(nightly, codesnip::entry)]
#[macro_export]
macro_rules! mat {
    () => {
        vec![]
    };
    ($e:expr; $n:expr $(,)?) => {
        vec![$e; $n]
    };
    ($e:expr; $nhead:expr, $($ntail:expr),* $(,)?) => {
        vec![$crate::mat![$e; $($ntail),*]; $nhead]
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn mat() {
        let t = mat![3; 3, 3];
        assert_eq!(t.len(), 3);
        assert!(t.iter().all(|v| v.len() == 3));
        assert!(t.iter().all(|v| v.iter().all(|x| *x == 3)));
    }
}
