// 执行 cargo test 运行
#[cfg(test)]
mod tests {
    // rust_example 为 Cargo.toml 中 package 的 name
    use rust_example::abi::pb::PbPerson;
    use rust_example::abi::business::PersonService;

    #[test]
    fn abi_test() {
        let person = PbPerson::new("ray".into(), 20);
        println!("{:?}", person)
    }

    #[test]
    fn person_service_test() {
        let mut service = PersonService::new();

        let person = PbPerson::new("ray".into(), 20);
        assert_eq!(service.add_person(person).unwrap(), ());

        let person = PbPerson::new("najj".into(), 234);
        let result = service.add_person(person);
        println!("{:?}", result);
        assert_eq!(result.is_err(), true);

        let p = service.get_person("ray").unwrap();
        println!("{:?}", p);
        assert_eq!(*p, PbPerson::new("ray".into(), 20));

        let vec = service.get_all_person();
        assert_eq!(vec.is_some(), true);
        // 如果执行 vec.unwrap()，原先的 vec 所有权会被移除，在 println! 无法使用
        assert_eq!(vec.as_ref().unwrap().len() > 0, true);
        println!("{:?}", vec);

        let result1 = service.remove_person("sjsjsj");
        assert_eq!(result1.is_err(), true);
        println!("{:?}", result1.err().unwrap());
    }
}