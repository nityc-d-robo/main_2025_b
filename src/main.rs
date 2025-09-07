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
use crate::functions::elevator::Elevator;
use controllers::*;
use functions::omni::*;
use functions::retaining_arm::RetainingArm;
use functions::roof_arm::RoofArm;

struct Mechanisms {
    re_arm: RetainingArm,
    ro_arm: RoofArm,
    el: Elevator,
}

const NODE_NAME: &str = "main_2025_b";
const URL: &str = "http://192.168.0.102:50051";
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
            let mut controller = controllers::Gamepad::new(controllers::DualSenseLayout);
            move |msg: TakenMsg<Joy>| proseed(msg, &mut controller, &mut mechanisms)
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

fn proseed(
    msg: TakenMsg<Joy>,
    contoller: &mut Gamepad<DualSenseLayout>,
    mechanisms: &mut Mechanisms,
) {
    let _logger = Logger::new(NODE_NAME);

    //roof
    {
        if contoller.pressed_edge(&msg, Button::Square) {
            mechanisms.ro_arm.right_toggle();
        }

        if contoller.pressed(&msg, Button::DpadLeft) {
            if contoller.pressed(&msg, Button::Triangle) {
                mechanisms.ro_arm.ud_up();
            }
            if contoller.pressed(&msg, Button::Cross) {
                mechanisms.ro_arm.ud_down();
            }
        }
        if !contoller.pressed(&msg, Button::Triangle) && !contoller.pressed(&msg, Button::Cross) {
            mechanisms.ro_arm.ud_stop();
        }
    }

    // el
    {
        if contoller.pressed(&msg, Button::Circle) {
            if contoller.pressed(&msg, Button::DpadUp) {
                mechanisms.el.second_up();
            }
            if contoller.pressed(&msg, Button::DpadDown) {
                mechanisms.el.second_down();
            }
            if !contoller.pressed(&msg, Button::DpadUp)
                && !contoller.pressed(&msg, Button::DpadDown)
            {
                mechanisms.el.second_stop();
            }
        } else {
            if contoller.pressed(&msg, Button::DpadUp) {
                mechanisms.el.first_up();
                mechanisms.el.second_up();
            }
            if contoller.pressed(&msg, Button::DpadDown) {
                mechanisms.el.first_down();
                mechanisms.el.second_down();
            }
            if !contoller.pressed(&msg, Button::DpadUp)
                && !contoller.pressed(&msg, Button::DpadDown)
            {
                mechanisms.el.first_stop();
                mechanisms.el.second_stop();
            }
        }
    }

    // re
    {
        {
            if contoller.pressed(&msg, Button::R2) {
                mechanisms.re_arm.right_fold();
            }
            if contoller.pressed(&msg, Button::R1) {
                mechanisms.re_arm.right_unfold();
            }
            if !contoller.pressed(&msg, Button::R2) && !contoller.pressed(&msg, Button::R1) {
                mechanisms.re_arm.right_stop();
            }
        }

        {
            if contoller.pressed(&msg, Button::L2) {
                mechanisms.re_arm.left_fold();
            }
            if contoller.pressed(&msg, Button::L1) {
                mechanisms.re_arm.left_unfold();
            }
            if !contoller.pressed(&msg, Button::L2) && !contoller.pressed(&msg, Button::L1) {
                mechanisms.re_arm.left_stop();
            }
        }

        {
            if contoller.pressed(&msg, Button::DpadRight) {
                if contoller.pressed(&msg, Button::Triangle) {
                    mechanisms.re_arm.center_fold();
                }
                if contoller.pressed(&msg, Button::Cross) {
                    mechanisms.re_arm.center_unfold();
                }
            }
            if !contoller.pressed(&msg, Button::Triangle) && !contoller.pressed(&msg, Button::Cross)
            {
                mechanisms.re_arm.center_stop();
            }
        }
    }
    mechanisms.re_arm.update();
    mechanisms.ro_arm.update();
    mechanisms.el.update();
}
