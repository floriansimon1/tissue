use std::sync;

use antidote;
use futures::executor;

use crate::base::lazy_value;

#[test]
fn check_that_value_gets_computed_on_request() {
    struct Global;

    let global = sync::Arc::new(antidote::Mutex::new(Global));

    let value = lazy_value::make_lazy(|_| async { 3 });

    assert_eq!(*executor::block_on(value.get(global)), 3);
}

#[test]
fn function_is_not_computed_if_not_awaited() {
    let global = sync::Arc::new(antidote::Mutex::new(false));

    lazy_value::make_lazy(|called: sync::Arc<antidote::Mutex<bool>>| async move {
        *called.lock() = true;
    });

    assert_eq!(*global.lock(), false);
}

#[test]
fn result_can_be_queried_multiple_times_but_are_computed_once() {
    let global = sync::Arc::new(antidote::Mutex::new(0u8));

    let value = lazy_value::make_lazy(|calls: sync::Arc<antidote::Mutex<u8>>| async move {
        *calls.lock() += 1;
    });

    for _ in 0 ..= 3 {
        executor::block_on(value.get(global.clone()));
    }

    assert_eq!(*global.lock(), 1);
}
