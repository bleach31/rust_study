use futures::executor;
use std::future::Future;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

// async fn something_great_async_function() -> impl Future<Output = i32> {
async fn something_great_async_function() -> i32 {
    let ans1: i32 = async_add(2, 3).await;
    let ans2: i32 = async_add(ans1, 3).await;
    println!("{}", ans2);
    ans2
}

fn main() {
    executor::block_on(something_great_async_function());
    //konohen,
}
