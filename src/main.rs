use controllers::{p9n_interface, ButtonState};

use safe_drive::msg;
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
use functions::roof_arm::RoofArm;

struct Mechanisms {
    re_arm: RetainingArm,
    ro_arm: RoofArm,
}

const NODE_NAME: &str = "main_2025_b";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;
    let mut mechanisms = Mechanisms {
        re_arm: RetainingArm::new("http://192.168.0.206:50051", NODE_NAME),
        ro_arm: RoofArm::new("http://192.168.0.206:50051", NODE_NAME),
    };

    selector.add_subscriber(
        joy,
        Box::new({
            let mut mechanisms = mechanisms;
            let mut prev_buttons: controllers::ButtonState = [false; 13];
            move |msg: TakenMsg<Joy>| proseed(msg, &mut prev_buttons, &mut mechanisms)
        }),
    );

    loop {
        selector.wait()?;
    }
}

fn proseed(msg: TakenMsg<Joy>, prev_buttons: &mut ButtonState, mechanisms: &mut Mechanisms) {
    let _logger = Logger::new(NODE_NAME);
    let mut joy = controllers::Gamepad::new(&msg, controllers::DualSenseLayout);

    if joy.pressed_circle() {
        mechanisms.ro_arm.ud_up();
    }
    if joy.pressed_cross() {
        // mechanisms.re_arm.status.left = -1;
        mechanisms.ro_arm.ud_down()
    }
    if !joy.pressed_circle() && !joy.pressed_cross() {
        // mechanisms.re_arm.status.left = 0;
        mechanisms.ro_arm.ud_stop()
    }

    if joy.pressed_triangle_edge(prev_buttons) {
        mechanisms.ro_arm.right_toggle();
    }

    mechanisms.re_arm.update();
    mechanisms.ro_arm.update();
}
