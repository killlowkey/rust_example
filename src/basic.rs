#[cfg(test)]
mod tests {
    #[test]
    fn serialized_test() {
        let serialized = r#"{"name":"CoolBy","value":":("}"#;
        println!("{}", serialized);
    }
}