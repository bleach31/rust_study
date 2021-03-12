use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

fn mult(x: u32) -> u32 {
    x * 2
}

impl Future for CountDown {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<u32> {
        if self.0 % 5 == 0 {
            Poll::Ready(mult(self.0))
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let countdown_future1 = CountDown(14);
    let countdown_future2 = CountDown(24);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    println!("finish!");
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}
