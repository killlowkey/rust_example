pub fn box_example() {
    let a = Box::new(10);
    let b = *a + 10;
    assert_eq!(*a, 10);
    assert_eq!(b, 20);
}

#[cfg(test)]
mod tests {
    use crate::box_example::box_example;

    #[test]
    fn box_test() {
        box_example()
    }
}