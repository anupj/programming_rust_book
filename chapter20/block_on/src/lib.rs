use crossbeam::sync::Parker;
use futures_lite::pin;
use std::future::Future;
use std::task::{Context, Poll};
use waker_fn::waker_fn;

fn block_on<F: Future>(future: F) -> F::Output {
    // Parker type is a simple blocking primitive
    // `parker.park()` blocks the thread until someone
    // else calls `.unpark()` on the corresponding `Unparker`
    let parker = Parker::new();
    let unparker = parker.unparker().clone();

    // `waker_fn` creates a `Waker` from a given closure
    let waker = waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);

    // The `pin!` macro takes ownership of
    // the future and declares a new variable
    // of the same name whose type is `Pin<&mut F>`
    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => parker.park(),
        }
    }
}
