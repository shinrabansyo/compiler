use std::task::Context;
use std::sync::{Arc, Mutex};

pub struct SharedState {
    now_deadlock: bool,
    any_stepped: bool,
}

// for 'block_on'
impl SharedState {
    pub fn new() -> Arc<Mutex<SharedState>> {
        let shared_state = SharedState {
            now_deadlock: false,
            any_stepped: false,
        };
        Arc::new(Mutex::new(shared_state))
    }

    pub fn check_any_stepped(&self) -> bool {
        self.any_stepped
    }
}

// for 'join_all'
impl SharedState {
    pub fn notify_stepped(ctx: &mut Context<'_>) {
        let ext_data = ctx.ext();
        let shared_state = ext_data.downcast_mut::<Arc<Mutex<SharedState>>>().unwrap();
        shared_state.lock().unwrap().any_stepped = true;
    }
}

// for #[failable_as_async]
impl SharedState {
    pub fn check_deadlock(ctx: &mut Context<'_>) -> bool {
        let ext_data = ctx.ext();
        let shared_state = ext_data.downcast_mut::<Arc<Mutex<SharedState>>>().unwrap();
        shared_state.lock().unwrap().now_deadlock
    }
}

pub(crate) struct SharedStateBuilder {
    now_deadlock: bool,
}

impl SharedStateBuilder {
    pub fn new() -> Self {
        SharedStateBuilder {
            now_deadlock: false,
        }
    }

    pub fn build(self) -> Arc<Mutex<SharedState>> {
        Arc::new(Mutex::new(SharedState {
            now_deadlock: self.now_deadlock,
            any_stepped: false,
        }))
    }

    pub fn set_notify_deadlock(mut self, value: bool) -> Self {
        self.now_deadlock = value;
        self
    }
}
