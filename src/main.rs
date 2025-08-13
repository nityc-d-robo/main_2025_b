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
mod functions;
use functions::retaining_arm::RetainingArm;
struct Mechanisms {
    arm: RetainingArm,
}

const NODE_NAME: &str = "main_2025_b";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    let mut mechanisms = Mechanisms {
        arm: RetainingArm::new("http://192.168.0.206:50051", NODE_NAME),
    };
    selector.add_subscriber(
        joy,
        Box::new(move |msg: TakenMsg<Joy>| proseed(msg, &mut mechanisms)),
    );

    loop {
        selector.wait()?;
    }
}

fn proseed(msg: TakenMsg<Joy>, mechanisms: &mut Mechanisms) {
    let _logger = Logger::new(NODE_NAME);
    let binding = sensor_msgs::msg::Joy::new().unwrap();
    let mut joy = p9n_interface::DualShock4Interface::new(&binding);
    joy.set_joy_msg(&msg);

    if joy.pressed_circle() {
        mechanisms.arm.status.left = 1;
    }
    if joy.pressed_cross() {
        mechanisms.arm.status.left = -1;
    }
    if !joy.pressed_circle() && !joy.pressed_cross() {
        mechanisms.arm.status.left = 0;
    }

    mechanisms.arm.update();
}
