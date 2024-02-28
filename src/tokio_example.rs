// pub async fn tokio_spawn_example() -> Result<String, String> {
//     let c: Result<String, String> = tokio::spawn(async move {
//         println!("I am a tokio async");
//         Ok(String::from("success"))
//     }).await?;
//     c
// }


// https://tokio.rs/tokio/tutorial/hello-tokio
async fn say_world() {
    println!("world");
}

pub async fn tokio_say_world() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    let op = say_world();

    // This println! comes first
    println!("hello");

    // Calling `.await` on `op` starts executing `say_world`.
    op.await;
}