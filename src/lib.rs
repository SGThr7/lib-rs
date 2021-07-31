#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[codesnip::entry]
pub fn testfn() -> bool {
    println!("Hello world");
    true
}
