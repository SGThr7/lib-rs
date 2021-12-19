#[cfg_attr(nightly, codesnip::entry)]
#[macro_export]
macro_rules! yes_no {
    ($bool:expr, $yes:literal, $no:literal) => {
        if $bool {
            println!($yes)
        } else {
            println!($no)
        }
    };
    ($bool:expr) => {
        $crate::yes_no! { $bool, "Yes", "No" }
    };
}

#[cfg_attr(nightly, codesnip::entry)]
#[macro_export]
macro_rules! println_vec {
    ($vec:expr) => {{
        for a in $vec {
            println!("{}", a);
        }
    }};
}
