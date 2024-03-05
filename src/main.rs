extern crate core;

use crate::option_example::option_method_example;

mod greet_world;
mod struct_example;
mod flow_example;
mod complex_type_example;
mod option_example;
mod match_example;
mod generics_example;
mod trait_example;
mod lifetime_example;
mod tokio_example;
mod result_example;
mod closure_example;
mod mod_example;
mod mod2;
mod use_example;
mod box_example;
mod abi;


// #[tokio::main]
fn main()  {
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

    // option_example::option_example()
    // option_example::option_method_example();

    // generics_example::add_example();
    // generics_example::result_example();
    // generics_example::display_array_new_example();
    // generics_example::hashmap_example();
    generics_example::add_example2();

    // trait_example::trait_example();
    // trait_example::trait_notify_restraint_example();
    // trait_example::trait_notify_with_display_restraint_example();
    // trait_example::largest_example();

    // println!("{}", tokio_example::tokio_spawn_example().await?)
    // tokio_example::tokio_say_world().await

    // result_example::result_fn_example();
    // result_example::result_error_example();

    // closure_example::closure_example();

    // lifetime_example::return_owner_example();
}