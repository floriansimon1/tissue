use std::sync;

use antidote;
use futures::future;

pub fn make_lazy<Global, Value, Future, Λ>(λ: Λ) -> LazyValue<Global, Value>
where
      Global:        Send + 'static,
      Future:        future::Future<Output = Value> + Send,
      Λ:             (FnOnce(Global) -> Future) + Send + 'static
{
    use future::FutureExt;

    let λ = Box::new(move |global: Global| (async move {
        sync::Arc::new(λ(global).await)
    }).boxed());

    LazyValue { data: sync::Arc::new(antidote::Mutex::new(LazyValueData::AwaitingComputeOrder(Some(λ)))) }
}

pub enum LazyValueData<Global, Value> {
    AwaitingComputeOrder(Option<Box<dyn FnOnce(Global) -> future::BoxFuture<'static, sync::Arc<Value>> + Send>>),
    Computed(future::Shared<future::BoxFuture<'static, sync::Arc<Value>>>),
}

pub struct LazyValue<Global, Value> {
    data: sync::Arc<antidote::Mutex<LazyValueData<Global, Value>>>,
}

impl<Global, Value> LazyValue<Global, Value> where Value: Sync + Send {
    pub fn get(&self, global: Global) -> future::Shared<future::BoxFuture<'static, sync::Arc<Value>>> {
        let value = &mut *self.data.lock();

        if let LazyValueData::AwaitingComputeOrder(ref mut λ) = value {
            let future = futures::FutureExt::shared((λ.take().unwrap())(global));

            *value = LazyValueData::Computed(future.clone());

            future
        } else if let LazyValueData::Computed(value) = value  {
            value.clone()
        } else {
            panic!()
        }
    }
}
