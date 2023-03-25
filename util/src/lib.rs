use pin_project_lite::pin_project;
use std::{
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context, Poll},
};
use tokio::sync::{futures::Notified, Mutex, MutexGuard, Notify, OnceCell};

//

pub struct LazyAwait<F: Future> {
    value: OnceCell<F::Output>,
    ready: Notify,
    fut: Mutex<Pin<Box<F>>>,
}

pin_project! {
    #[project = GetLazyAwaitProj]
    pub enum GetLazyAwait<'a, F: Future> {
        Ready {
            value: &'a F::Output
        },
        Waiting {
            value: &'a OnceCell<F::Output>,
            #[pin]
            ready: Notified<'a>,
        },
        PrimaryWaiting {
            value: &'a OnceCell<F::Output>,
            ready: &'a Notify,
            #[pin]
            fut: MutexGuard<'a, Pin<Box<F>>>,
        },
    }
}

//

impl<F: Future> LazyAwait<F> {
    pub fn new(fut: F) -> Self {
        Self {
            value: <_>::default(),
            ready: <_>::default(),
            fut: Mutex::new(Box::pin(fut)),
        }
    }

    pub fn get(&self) -> GetLazyAwait<'_, F> {
        if let Some(value) = self.value.get() {
            return GetLazyAwait::Ready { value };
        }

        let notify = self.ready.notified();
        let Ok(future) = self.fut.try_lock() else {
            return GetLazyAwait::Waiting { value: &self.value, ready: notify }
        };

        GetLazyAwait::PrimaryWaiting {
            value: &self.value,
            ready: &self.ready,
            fut: future,
        }
    }
}

impl<'a, F: Future> Future for GetLazyAwait<'a, F> {
    type Output = &'a F::Output;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        match self.project() {
            GetLazyAwaitProj::Ready { value } => (*value).into(),
            GetLazyAwaitProj::Waiting { value, ready } => match ready.poll(ctx) {
                Poll::Ready(_) => value.get().unwrap().into(),
                Poll::Pending => Poll::Pending,
            },
            GetLazyAwaitProj::PrimaryWaiting { value, ready, fut } => {
                // let fut = fut.as_mut();
                let fut = fut.get_mut().as_mut();
                let val = match Future::poll(fut, ctx) {
                    Poll::Ready(val) => val,
                    Poll::Pending => return Poll::Pending,
                };

                value.set(val).map_err(|_| {}).unwrap();
                let poll = Poll::Ready(value.get().unwrap());
                ready.notify_waiters();

                poll
            }
        }
    }
}

impl<'a, F: Future> IntoFuture for &'a LazyAwait<F> {
    type Output = &'a F::Output;

    type IntoFuture = GetLazyAwait<'a, F>;

    fn into_future(self) -> Self::IntoFuture {
        self.get()
    }
}
