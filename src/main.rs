mod greet_world;
mod struct_example;
mod flow_example;
mod complex_type_example;
mod option_example;
mod match_example;
mod generics_example;
mod trait_example;


fn main() {
    // greet_world::greet_world();

    // let p = struct_example::Person {
    //     name: Some(String::from("ray")),
    //     age: Some(10),
    // };
    //
    // println!("{}", p);
    // println!("{}", p.get_name());
    // p.other_example();

    // loop_example::loop_example()
    // complex_type_example::string_example()
    // complex_type_example::tuple_example()
    // complex_type_example::enum_example()
    // complex_type_example::array_example()
    // complex_type_example::vec_example()

    // option_example::option_example()

    // generics_example::add_example();

    // trait_example::trait_example();
    // trait_example::trait_notify_restraint_example();
    trait_example::trait_notify_with_display_restraint_example();
}