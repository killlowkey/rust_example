#[cfg(test)]
mod tests {
    #[test]
    fn test_type_alias() {
        type Dog = Vec<String>;

        let mut d = Dog::new();
        d.push("1".into());
        d.push("2".into());
        d.push("3".into());

        println!("{:?}", d);
    }
}