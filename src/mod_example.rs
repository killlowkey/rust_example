pub use crate::mod2::back_of_house;

mod mod1 {
    pub mod sub_mod1 {
        pub fn sub_mod1_method() {
            println!("I am sub_mod1_method")
        }

        // `pub(crate)` 使得函数只在当前包中可见
        pub(crate) fn current_package_access() {
            println!("current_package_access")
        }

        // `pub(crate)` 使得函数可在父包中可见
        pub(super) fn super_package_access() {
            println!("super_package_access")
        }

        fn sub_mod1_inner_method() {
            println!("I am sub_mod1_inner_method")
        }

        fn using_parent_parent_method() {
            // super -> 父亲 mod1
            // super::super 父亲父亲 mod1 外部
            super::super::parent_method()
        }
    }

    mod sub_mod2 {
        pub fn sub_mod2_method() {
            println!("I am sub_mod2_method")
        }
    }
}

fn parent_method() {
    println!("I am parent_method")
}

fn using_mod_method() {
    // 必须要 public 才可以访问
    // self: 表示当前文件
    // mod1: mod 名字
    // sub_mod1: mod1 中的 mod
    // sub_mod1_method: sub_mod1 中 public 方法
    self::mod1::sub_mod1::sub_mod1_method();

    // self::mod1::sub_mod1::current_package_access();
}

fn using_outside_mod_method() {
    // 直接使用
    // crate: 表示当前包
    // mod2：文件名（mod2.rs）
    // back_of_house：mod 名
    // cook_order：back_of_house 中 public 方法
    // crate::mod2::back_of_house::cook_order();

    // 导入使用
    // use crate::mod2::back_of_house;
    back_of_house::cook_order()
}