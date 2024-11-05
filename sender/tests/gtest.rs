use sails_rs::{
    calls::*,
    gtest::{calls::*, System},
};

use sender_client::traits::*;
use waker_client::traits::*;
const ACTOR_ID: u64 = 42;

#[tokio::test]
async fn wait_wake_test() {
    let system = System::new();
    system.init_logger();
    system.mint_to(ACTOR_ID, 100_000_000_000_000);

    let remoting = GTestRemoting::new(system, ACTOR_ID.into());
    remoting.system().init_logger();

    // Submit program code into the system
    let program_code_id = remoting.system().submit_code(sender::WASM_BINARY);

    let waker_code_id = remoting.system().submit_code(waker::WASM_BINARY);

    let waker_factory = waker_client::WakerFactory::new(remoting.clone());

    let waker_id = waker_factory
        .new() 
        .send_recv(waker_code_id, b"salt")
        .await
        .unwrap();


    let program_factory = sender_client::SenderFactory::new(remoting.clone());

    let program_id = program_factory
        .new() 
        .send_recv(program_code_id, b"salt")
        .await
        .unwrap();

    let mut service_client = sender_client::Sender::new(remoting.clone());

    service_client
        .send_msg_and_go_to_wait(waker_id)
        .send_recv(program_id)
        .await;
    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();

    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();
    remoting.system().run_next_block();

}
