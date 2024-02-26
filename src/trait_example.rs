use std::fmt::{Debug, Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    // 标题
    pub title: String,
    // 作者
    pub author: String,
    // 内容
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn trait_example() {
    let post = Post { title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content: "Rust棒极了!".to_string() };
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多重约束
// pub fn notify_with_display(item: &(impl Summary + Display)) {}
pub fn notify_with_display<T: Summary + Display>(item: &T) {
    println!("{}", item)
}

pub fn trait_notify_restraint_example() {
    let post = Post { title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content: "Rust棒极了!".to_string() };
    let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };

    notify(&post);
    notify(&weibo);
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "author={} title={} content={}", &self.author, &self.title, &self.content)
    }
}

pub fn trait_notify_with_display_restraint_example() {
    let post = Post { title: "Rust语言简介".to_string(), author: "Sunface".to_string(), content: "Rust棒极了!".to_string() };
    // let weibo = Weibo { username: "sunface".to_string(), content: "好像微博没Tweet好用".to_string() };

    notify_with_display(&post);
    // notify_with_display(&weibo); // error
}

// where 约束
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_with_where<T>(list: &[T]) -> T
    where T: PartialOrd + Copy
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_example() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}