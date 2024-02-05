use std::{
    pin::Pin,
    task::{self, Poll},
    time::{Duration, Instant},
};

use futures_util::Future;
use pin_project_lite::pin_project;

pub enum MissedTickBehavior {
    Delay,
}

pub struct Interval;

impl Interval {
    pub async fn tick(&mut self) {}
    pub fn set_missed_tick_behavior(&self, behavior: MissedTickBehavior) {}
}

pin_project! {
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Timeout<T>{
    #[pin]
    value: T
}
}

impl<T> Future for Timeout<T>
where
    T: Future,
{
    type Output = Result<T::Output, std::time::Duration>;

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

pub async fn sleep(duration: Duration) {
    #[cfg(feature = "tokio_runtime")]
    tokio::time::sleep(duration).await;
    #[cfg(feature = "glommio_runtime")]
    glommio::timer::sleep(duration).await;
}

pub async fn sleep_until(time: Instant) {
    let duration = time - Instant::now();
    sleep(duration).await
}

pub fn timeout<F: Future>(duration: Duration, future: F) -> Timeout<F> {
    Timeout { value: future }
}

pub fn interval(duration: Duration) -> Interval {
    Interval {}
}
pub async fn spawn_blocking<F, R>(fut: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let res = if cfg!(feature = "tokio_runtime") {
        tokio_blocking(fut).await
    } else if cfg!(feature = "glommio_runtime") {
        glommio_blocking(fut).await
    } else {
        panic!("No runtime feature enabled")
    };

    res
}

#[cfg(feature = "tokio_runtime")]
async fn tokio_blocking<F, R>(fut: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let output = tokio::task::spawn_blocking(fut).await.unwrap();
    output
}

#[cfg(feature = "glommio_runtime")]
async fn glommio_blocking<F, R>(fut: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let output = glommio::executor().spawn_blocking(fut).await;
    output
}

pub async fn spawn<F: Future + Send + 'static>(future: F)
where
    F::Output: Send,
{
    #[cfg(feature = "tokio_runtime")]
    tokio::spawn(future);

    #[cfg(feature = "glommio_runtime")]
    glommio::spawn_local(future).detach();
}
