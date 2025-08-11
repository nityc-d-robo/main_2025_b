use controllers::p9n_interface;

#[allow(unused_imports)]
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

use crate::functions::retaining_arm::RetainingArmState;

const NODE_NAME: &str = "main_2025_b";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    let state = Arc::new(RwLock::new(retaining_arm::RetainingArmState::new()));

    let thread_state = state.clone();
    thread::spawn(move || {
        retaining_arm::entry(thread_state);
    });

    selector.add_subscriber(
        joy,
        Box::new(move |msg: TakenMsg<Joy>| proseed(msg, (state.clone(), 0))),
    );

    loop {
        selector.wait()?;
    }
}

fn proseed(msg: TakenMsg<Joy>, states: (Arc<RwLock<RetainingArmState>>, usize)) {
    let _logger = Logger::new(NODE_NAME);
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy = p9n_interface::DualShock4Interface::new(&binding);
    joy.set_joy_msg(&msg);

    let mut tmp = states.0.write().unwrap();
    if joy.pressed_circle() {
        tmp.test = 1;
    }
    if !joy.pressed_circle() {
        tmp.test = 0;
    }
}
