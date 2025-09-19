use std::{cell::RefCell, collections::HashMap, rc::Rc};

use motor_lib::{md, GrpcHandle};
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
use crate::functions::{ei::Ei, elevator::Elevator, MdAdress};
use controllers::*;
use functions::omni::*;
use functions::retaining_arm::RetainingArm;
use functions::roof_arm::RoofArm;

struct Mechanisms {
    re_arm: RetainingArm,
    ro_arm: RoofArm,
    el: Elevator,
    omni: Omni,
    ei: Ei,
}

struct MechanismsEi {
    ei: Ei,
}

const NODE_NAME: &str = "main_2025_b";
const URL: &str = "http://192.168.0.4:50051";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let subscriber_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    let subscriber_cmd = node.create_subscriber::<msg::Twist>("cmd_vel", None)?;

    let mechanisms = Rc::new(RefCell::new(Mechanisms {
        re_arm: RetainingArm::new(URL, NODE_NAME),
        ro_arm: RoofArm::new(URL, NODE_NAME),
        el: Elevator::new(URL, NODE_NAME),
        omni: Omni::new(URL, NODE_NAME),
        ei: Ei::new(URL, NODE_NAME),
    }));

    let joy_mechanisms = mechanisms.clone();
    selector.add_subscriber(
        subscriber_joy,
        Box::new({
            let mut controller = controllers::Gamepad::new(controllers::DualShock4Layout);
            move |msg: TakenMsg<Joy>| proseed(msg, &mut controller, joy_mechanisms.clone())
        }),
    );

    let mut prev_motor_power = HashMap::new();

    let alpha = 0.5; // 0.1ならゆっくり、0.5なら速く追従
    let cmd_mechanisms = mechanisms.clone();
    selector.add_subscriber(
        subscriber_cmd,
        Box::new(move |msg| {
            let omni = &mut cmd_mechanisms.borrow_mut().omni;
            let target_power =
                omni.omni_setting()
                    .move_chassis(msg.linear.x, msg.linear.y, msg.angular.z);

            for (i, &tp) in target_power.iter() {
                prev_motor_power
                    .entry(*i)
                    .and_modify(|prev| *prev += alpha * (tp - *prev))
                    .or_insert(tp);
            }

            omni.update(&prev_motor_power);
        }),
    );

    loop {
        selector.wait()?;
    }
}

fn proseed(
    msg: TakenMsg<Joy>,
    contoller: &mut Gamepad<DualShock4Layout>,
    _mechanisms: Rc<RefCell<Mechanisms>>,
) {
    let mechanisms = &mut *_mechanisms.borrow_mut();
    let _logger = Logger::new(NODE_NAME);

    //roof
    {
        if contoller.pressed(&msg, Button::L1)
            && contoller.pressed_edge(&msg, Button::Circle)
            && !contoller.pressed(&msg, Button::R1)
        {
            mechanisms.ro_arm.right_toggle();
        }

        if contoller.pressed(&msg, Button::L1) && contoller.pressed(&msg, Button::DpadUp) {
            mechanisms.ro_arm.ud_up();
        }
        if contoller.pressed(&msg, Button::L1) && contoller.pressed(&msg, Button::DpadDown) {
            mechanisms.ro_arm.ud_down();
        }

        if !contoller.pressed(&msg, Button::DpadUp) && !contoller.pressed(&msg, Button::DpadDown) {
            mechanisms.ro_arm.ud_stop();
        }

        if contoller.pressed(&msg, Button::L1) && contoller.pressed_edge(&msg, Button::Cross) {
            mechanisms.ro_arm.bq_toggle();
        }
    }

    // el
    {
        if contoller.pressed(&msg, Button::Cross) && !contoller.pressed(&msg, Button::Circle) {
            mechanisms.el.first_up();
            mechanisms.el.second_up();
        }
        if contoller.pressed(&msg, Button::DpadDown) {
            mechanisms.el.first_down();
            mechanisms.el.second_down();
        }

        if !contoller.pressed(&msg, Button::Cross) && !contoller.pressed(&msg, Button::DpadDown) {
            mechanisms.el.first_stop();
            mechanisms.el.second_stop();
        }

        if contoller.pressed(&msg, Button::Cross) && contoller.pressed(&msg, Button::Circle) {
            mechanisms.el.second_up();
        }
        if contoller.pressed(&msg, Button::DpadLeft) && contoller.pressed(&msg, Button::DpadDown) {
            mechanisms.el.second_down();
        }

        if !contoller.pressed(&msg, Button::Cross)
            && !contoller.pressed(&msg, Button::DpadLeft)
            && !contoller.pressed(&msg, Button::DpadDown)
        {
            mechanisms.el.second_stop();
        }
    }

    //re
    {
        if contoller.pressed(&msg, Button::L1) && contoller.pressed(&msg, Button::R1) {
            {
                if contoller.pressed(&msg, Button::DpadLeft) {
                    mechanisms.re_arm.left_unfold();
                    pr_info!(_logger, "left_unfold");
                }
                if contoller.pressed(&msg, Button::DpadRight) {
                    mechanisms.re_arm.left_fold();
                    pr_info!(_logger, "left_fold");
                }
                if !contoller.pressed(&msg, Button::DpadLeft)
                    && !contoller.pressed(&msg, Button::DpadRight)
                {
                    mechanisms.re_arm.left_stop();
                    pr_info!(_logger, "left_stop");
                }
            }

            {
                if contoller.pressed(&msg, Button::Circle) {
                    mechanisms.re_arm.right_unfold();
                }
                if contoller.pressed(&msg, Button::Square) {
                    mechanisms.re_arm.right_fold();
                }
                if !contoller.pressed(&msg, Button::Circle)
                    && !contoller.pressed(&msg, Button::Square)
                {
                    mechanisms.re_arm.right_stop();
                }
            }
        }
        if !contoller.pressed(&msg, Button::R1) && !contoller.pressed(&msg, Button::L1) {
            {
                mechanisms.re_arm.left_stop();
                mechanisms.re_arm.right_stop();
            }
        }
    }
    //ei

    {
        if contoller.pressed(&msg, Button::R1) {
            {
                if contoller.pressed(&msg, Button::DpadUp) {
                    mechanisms.ei.roller_ud_up(5.0);
                }
                if contoller.pressed(&msg, Button::DpadDown) {
                    mechanisms.ei.roller_ud_down(5.0);
                }
            }

            if contoller.pressed_edge(&msg, Button::Circle) {
                mechanisms.ei.roller_toggle();
            }

            {
                if contoller.pressed(&msg, Button::Square) {
                    mechanisms.ei.fin_unfold();
                }
                if contoller.pressed(&msg, Button::Triangle) {
                    mechanisms.ei.fin_fold();
                }
                if !contoller.pressed(&msg, Button::Square)
                    && !contoller.pressed(&msg, Button::Triangle)
                {
                    mechanisms.ei.fin_stop();
                }
            }
        }
        if !contoller.pressed(&msg, Button::R1) {
            mechanisms.ei.fin_stop();
        }

        if contoller.pressed(&msg, Button::L1) {
            mechanisms.ei.ud_up();
            pr_info!(_logger, "ei ud up");
        }
        if contoller.pressed(&msg, Button::L1) && contoller.pressed(&msg, Button::Triangle) {
            mechanisms.ei.ud_down();
            pr_info!(_logger, "ei ud down");
        }
        if !contoller.pressed(&msg, Button::L1) {
            mechanisms.ei.ud_stop();
            pr_info!(_logger, "ei ud stop");
        }

        if contoller.pressed(&msg, Button::R1) && contoller.pressed_edge(&msg, Button::Cross) {
            mechanisms.ei.bq_toggle();
        }
    }

    mechanisms.re_arm.update();
    mechanisms.ro_arm.update();
    mechanisms.el.update();
    mechanisms.ei.update();
}
