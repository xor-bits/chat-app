use std::{future::IntoFuture, sync::Arc};
use surrealdb::{Datastore, Session};
use util::{GetLazyAwait, LazyAwait};

//

pub struct Database {
    inner: Arc<LazyAwait<DatabaseInner>>,
}

#[derive(Debug, Clone)]
struct DatabaseInner {
    ds: Arc<Datastore>,
    ses: Arc<Session>,
}

//

impl<'a> IntoFuture for &'a Database {
    type Output = &'a DatabaseInner;

    type IntoFuture = <LazyAwait<DatabaseInner> as IntoFuture>::IntoFuture;

    fn into_future(self) -> Self::IntoFuture {
        self.inner.as_ref().into_future()
    }
}
