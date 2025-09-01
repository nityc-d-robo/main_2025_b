use controllers::ButtonState;
use motor_lib::{md, GrpcHandle};
use omni_control::OmniSetting;
use safe_drive::msg::common_interfaces::geometry_msgs::msg;

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
use functions::omni::*;
use functions::retaining_arm::RetainingArm;
use functions::roof_arm::RoofArm;

use crate::functions::elevator::Elevator;

struct Mechanisms {
    re_arm: RetainingArm,
    ro_arm: RoofArm,
    el: Elevator,
}

const NODE_NAME: &str = "main_2025_b";
const URL: &str = "http://192.168.0.206:50051";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let subscriber_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;
    let subscriber_cmd = node.create_subscriber::<msg::Twist>("cmd_vel", None)?;

    let mechanisms = Mechanisms {
        re_arm: RetainingArm::new(URL, NODE_NAME),
        ro_arm: RoofArm::new(URL, NODE_NAME),
        el: Elevator::new(URL, NODE_NAME),
    };

    selector.add_subscriber(
        subscriber_joy,
        Box::new({
            let mut mechanisms = mechanisms;
            let mut prev_buttons: controllers::ButtonState = [false; 13];
            move |msg: TakenMsg<Joy>| proseed(msg, &mut prev_buttons, &mut mechanisms)
        }),
    );

    selector.add_subscriber(
        subscriber_cmd,
        Box::new(move |msg| {
            let omni_setting = OmniSetting {
                chassis: CHASSIS,
                max_pawer_input: MAX_PAWER_INPUT,
                max_pawer_output: MAX_PAWER_OUTPUT,
                max_revolution: MAX_REVOLUTION,
            };

            let motor_power = omni_setting.move_chassis(msg.linear.x, msg.linear.y, msg.angular.z);
            for i in motor_power.keys() {
                md::send_pwm(
                    &GrpcHandle::new(URL.as_ref()),
                    *i as u8,
                    -motor_power[i] as i16,
                );
            }
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
        mechanisms.el.up();
    }
    if joy.pressed_cross() {
        // mechanisms.re_arm.status.left = -1;
        mechanisms.el.down()
    }
    if !joy.pressed_circle() && !joy.pressed_cross() {
        // mechanisms.re_arm.status.left = 0;
        mechanisms.el.stop()
    }

    if joy.pressed_triangle_edge(prev_buttons) {
        mechanisms.ro_arm.right_toggle();
    }

    mechanisms.re_arm.update();
    mechanisms.el.update();
}
