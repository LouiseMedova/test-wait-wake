#![no_std]

use sails_rs::{
    gstd::{debug, exec, msg},
    prelude::*,
};

static mut WAIT: bool = false;
static mut MSG_ID: MessageId = MessageId::zero();

struct SenderService(());

#[sails_rs::service]
impl SenderService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn send_msg_and_go_to_wait(&mut self, waker: ActorId) {
        unsafe {
            if WAIT == false {
                let payload = ["Waker".encode(), "WakeMe".encode()].concat();
                msg::send_bytes(waker, payload, 0).expect("Error during sending");
                WAIT = true;
                MSG_ID = msg::id();
                debug!("GO TO WAIT");
                exec::wait_for(10);
            } else {
                debug!("WAKED!");
                exec::wait_for(1000);
            }
        }
    }

    pub fn wake(&mut self) {
        unsafe {
            exec::wake(MSG_ID).expect("Unable to wake");
        }
    }
}

pub struct SenderProgram(());

#[sails_rs::program]
impl SenderProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn sender(&self) -> SenderService {
        SenderService::new()
    }
}
