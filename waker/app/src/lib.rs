#![no_std]

use sails_rs::{gstd::msg, prelude::*};

struct WakerService(());

#[sails_rs::service]
impl WakerService {
    pub fn new() -> Self {
        Self(())
    }

    // Service's method (command)
    pub fn wake_me(&mut self) {
        let payload = ["Sender".encode(), "Wake".encode()].concat();
        sails_rs::gstd::debug!("WAKER");
        msg::send_bytes(msg::source(), payload, 0).expect("Error during sending");
    }
}

pub struct WakerProgram(());

#[sails_rs::program]
impl WakerProgram {
    // Program's constructor
    pub fn new() -> Self {
        Self(())
    }

    // Exposed service
    pub fn waker(&self) -> WakerService {
        WakerService::new()
    }
}
