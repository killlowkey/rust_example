use std::fmt::{Debug, Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String;

    // 可以不用实现该方法，因为有默认实现
    fn default(&self) -> String {
        String::from("default implementation")
    }
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

// 限定都是 Summary 类型
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// 多重约束：Summary 和 Display 类型
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

// where 约束，等价于下面的约束，本质上就是优化 <> 里面东西，使得看起来更简洁
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    1
}

// PartialOrd 比较顺序
// Copy 用于拷贝数据
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

    let age_list = [1, 2, 400, 3, 8, 5];
    let result = largest(&age_list);
    println!("The largest number is {}", result);
}

fn returns_summary(switch: bool) -> Box<dyn Summary> {
    if switch {
        Box::new(Post {
            title: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        })
    } else {
        Box::new(Weibo {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
        })
    }
}

// https://time.geekbang.org/column/article/410038
// 而接口将调用者和实现者隔离开，大大促进了代码的复用和扩展。
// 面向接口编程可以让系统变得灵活，当使用接口去引用具体的类型时，我们就需要虚表来辅助运行时代码的执行。
// 有了虚表，我们可以很方便地进行动态分派，它是运行时多态的基础。
pub fn virtual_table() {
    let mut post = Post {
        title: String::from(
            "Penguins win the Stanley Cup Championship!",
        ),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
        )
    };

    // 类型擦除，转为具体的 trait，此时需要虚表的支持
    let summary: &mut dyn Summary = &mut post;
    println!("{}", summary.summarize());
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn virtual_table_test() {
        virtual_table();
    }
}