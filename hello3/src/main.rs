use async_trait::async_trait;
use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[tokio::main]
async fn main() {
    ftr();
    ais();
    let ans = add(2, 3).await;
    println!("{}", ans);
    println!("--------------------");
}

#[async_trait]
pub trait AsyncTrait {
    async fn f(&self);
}

struct Runner {}

#[async_trait]
impl AsyncTrait for Runner {
    async fn f(&self) {
        println!("hello async trait")
    }
}

async fn add(left: i32, right: i32) -> i32 {
    left + right
}
fn ais() {
    executor::block_on(something_great_async_function());
}
async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}
fn something_great_async_function() -> impl Future<Output = i32> {
    async {
        let ans = async_add(2, 3).await;
        println!("{}", ans);
        ans
    }
}

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
fn ftr() {
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);

    for (i, s) in res.iter().enumerate() {
        println!("{}:{}", i, s);
    }
}
