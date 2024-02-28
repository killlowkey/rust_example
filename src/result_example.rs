use std::error::Error;
use std::fs::File;


// arr 无法使用 arr: [i32] 这种，传入的参数必须是固定大小的
// 比如引用类型&[i32]、或者固定大小的数组[i32; 4]、或者 Box<[i32]>
fn result_fn(arr: &[i32; 4]) -> Option<&i32> {
    // 无法这样使用 ? 必须使用一个变量来承载 ？运算符结果
    // arr.get(0)?
    let res = arr.get(0)?;
    println!("{}", res);
    Some(res)
}

pub fn result_fn_example() {
    let arr = [1, 3, 4, 5];
    println!("{:?}", result_fn(&arr))
}

fn result_error() -> Result<(), Box<dyn Error>> {
    // let f = File::open("hello.txt")?; // 如果打开失败，使用了 ? 运算符，会直接 panic
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("open file successfully");
            Ok(())
        }
        Err(err) => {
            println!("open file failed with error: {}", err.to_string());
            Err(Box::new(err))
        }
    }
}

pub fn result_error_example() {
    // 等价于：result_error()?
    result_error().unwrap()
}

fn result_example() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}