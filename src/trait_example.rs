use std::fmt::{Display, Formatter};

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