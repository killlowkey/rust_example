#[cfg(test)]
mod tests {
    // #[test]
    // fn panic() {
    //     let names = vec!["1", "2"];
    //     // panic("called `Option::unwrap()` on a `None` value"),
    //     names.get(10).unwrap();
    // }

    #[test]
    fn unwrap() {
        let names = vec!["1", "2"];
        assert_eq!(names.get(0).unwrap().to_owned(), "1");
    }
}