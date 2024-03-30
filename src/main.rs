extern crate core;
mod greet_world;
mod struct_example;
mod flow_example;
mod complex_type_example;
mod option;
mod match_example;
mod generics_example;
mod trait_example;
mod lifetime_example;
mod tokio_example;
mod result_example;
mod closure_example;
mod mod_example;
mod mod2;
mod r#use;
mod box_example;
mod abi;


// 值无法离开类型单独讨论，类型一般分为原生类型和组合类型。指针和引用都指向值的内存地址，只不过二者在解引用时的行为不一样。引用只能解引用到原来的数据类型，而指针没有这个限制，然而，不受约束的指针解引用，会带来内存安全方面的问题。
// 函数是代码中重复行为的抽象，方法是对象内部定义的函数，而闭包是一种特殊的函数，它会捕获函数体内使用到的上下文中的自由变量，作为闭包成员的一部分。
// 而接口将调用者和实现者隔离开，大大促进了代码的复用和扩展。面向接口编程可以让系统变得灵活，当使用接口去引用具体的类型时，我们就需要虚表来辅助运行时代码的执行。有了虚表，我们可以很方便地进行动态分派，它是运行时多态的基础。
// 在代码的运行方式中，并发是并行的基础，是同时与多个任务打交道的能力；并行是并发的体现，是同时处理多个任务的手段。同步阻塞后续操作，异步允许后续操作。被广泛用于异步操作的 Promise 代表未来某个时刻会得到的结果，async/await 是 Promise 的封装，一般用状态机来实现。
// 泛型编程通过参数化让数据结构像函数一样延迟绑定，提升其通用性，类型的参数可以用接口约束，使类型满足一定的行为，同时，在使用泛型结构时，我们的代码也需要更高的抽象度。
// 这些基础概念，这对于后续理解 Rust 的很多概念至关重要。
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