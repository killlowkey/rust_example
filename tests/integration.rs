#[cfg(test)]
mod tests {
    // rust_example 为 Cargo.toml 中 package 的 name
    use rust_example::abi::pb::PbPerson;
    use rust_example::abi::business::PersonService;

    #[test]
    fn abi_test() {
        let person = PbPerson::new("ray".to_string(), 20);
        println!("{:?}", person)
    }

    #[test]
    fn person_service_test() {
        let mut service = PersonService::new();

        let person = PbPerson::new("ray".to_string(), 20);
        assert_eq!(service.add_person(person).unwrap(), ());

        let person = PbPerson::new("najj".to_string(), 234);
        let result = service.add_person(person);
        println!("{:?}", result);
        assert_eq!(result.is_err(), true);

        let p = service.get_person("ray").unwrap();
        println!("{:?}", p);
    }
}