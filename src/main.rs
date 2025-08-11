#[allow(unused_imports)]
use controllers::p9n_interface;

use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{sensor_msgs, sensor_msgs::msg::Joy},
    pr_info,
    topic::publisher::Publisher,
    topic::subscriber::TakenMsg,
};
use std::sync::{Arc, RwLock};
use std::thread;
mod functions;
use functions::*;

fn main() -> Result<(), DynError> {
    let _logger = Logger::new("director_2024_a");
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node("director_2024_a", None, Default::default())?;
    let joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy0", None)?;

    let state = Arc::new(RwLock::new(retaining_arm::RetainingArmState::new()));

    let thread_state = state.clone();
    thread::spawn(move || {
        retaining_arm::entry(thread_state);
    });

    selector.add_subscriber(joy, Box::new(move |msg: TakenMsg<Joy>| proseed(msg)));

    loop {
        selector.wait()?;
    }
}

fn proseed(msg: TakenMsg<Joy>) {
    let _logger = Logger::new("director_2024_a");
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy = p9n_interface::DualShock4Interface::new(&binding);
    joy.set_joy_msg(&msg);

    if joy.pressed_dpad_up() {
        pr_info!(_logger, "hoge");
    }
}
