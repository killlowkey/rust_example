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
        assert_eq!(service.add(person).unwrap(), ());

        let person = PbPerson::new("najj".into(), 234);
        let result = service.add(person);
        println!("{:?}", result);
        assert_eq!(result.is_err(), true);

        let p = service.get("ray").unwrap().to_owned();
        println!("{:?}", p);
        assert_eq!(p, PbPerson::new("ray".into(), 20));

        // 转为所有权在进行比较
        let x = service.get_mut("ray").unwrap();
        assert_eq!(x.to_owned(), PbPerson::new("ray".into(), 20));
        println!("x={:?}", x);

        let vec = service.list();
        assert_eq!(vec.is_some(), true);
        // 如果执行 vec.unwrap()，原先的 vec 所有权会被移除，在 println! 无法使用
        assert_eq!(vec.as_ref().unwrap().len() > 0, true);
        println!("{:?}", vec);

        let result1 = service.remove("sjsjsj");
        assert_eq!(result1.is_err(), true);
        // println!("{:?}", result1.err().unwrap());

        assert_eq!(service.is_empty(), false);
    }
}