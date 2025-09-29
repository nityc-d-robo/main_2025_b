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
use crate::functions::{ei::Ei, elevator::Elevator};
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

const NODE_NAME: &str = "main_2025_b";
const URL: &str = "http://192.168.0.4:50051";
fn main() -> Result<(), DynError> {
    let _logger = Logger::new(NODE_NAME);
    let ctx = Context::new()?;
    let mut selector = ctx.create_selector()?;
    let node = ctx.create_node(NODE_NAME, None, Default::default())?;
    let subscriber_joy = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;

    let mut mechanisms = Mechanisms {
        re_arm: RetainingArm::new(URL, NODE_NAME),
        ro_arm: RoofArm::new(URL, NODE_NAME),
        el: Elevator::new(URL, NODE_NAME),
        omni: Omni::new(URL, NODE_NAME),
        ei: Ei::new(URL, NODE_NAME),
    };

    selector.add_subscriber(
        subscriber_joy,
        Box::new({
            let mut controller = controllers::Gamepad::new(controllers::DualSenseLayout);
            move |msg: TakenMsg<Joy>| proseed(msg, &mut controller, &mut mechanisms)
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
    use controllers::combination::ButtonCombination as BC;
    use controllers::combination::State::*;
    use controllers::Button::*;

    let _logger = Logger::new(NODE_NAME);
    {
        if contoller.pressed_edge(&msg, Select) {
            mechanisms.omni.reverse_direction();
            mechanisms.re_arm.reverse_direction();
        }

        if BC::new()
            .add(L1.state(Pressed))
            .add(L2.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.omni.max_pawer_output_set();
            mechanisms.omni.alpha_set(0.1);
        } else {
            mechanisms.omni.max_pawer_output_reset();
            mechanisms.omni.alpha_set(1.0);
        }
    }
    //roof
    {
        if BC::new()
            .add(L2.state(Pressed))
            .add(Cross.state(PressedEdge))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.bq_toggle();
        }

        if BC::new()
            .add(L2.state(Pressed))
            .add(Circle.state(PressedEdge))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.right_toggle();
        }

        if BC::new()
            .add(L2.state(Pressed))
            .add(DpadLeft.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.roof_right();
        } else if BC::new()
            .add(L2.state(Pressed))
            .add(DpadRight.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.roof_left();
        } else {
            mechanisms.ro_arm.roof_stop();
        }

        if BC::new()
            .add(L2.state(Pressed))
            .add(DpadUp.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.ud_up();
        } else if BC::new()
            .add(L2.state(Pressed))
            .add(DpadDown.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.ud_down();
        } else {
            mechanisms.ro_arm.ud_stop();
        }

        if BC::new()
            .add(L2.state(Pressed))
            .add(Cross.state(PressedEdge))
            .evalute(contoller, &msg)
        {
            mechanisms.ro_arm.bq_toggle();
        }
    }

    // el
    {
        if BC::new().add(Cross.state(Pressed)).evalute(contoller, &msg) {
            mechanisms.el.first_up();
            mechanisms.el.second_up();
        } else if BC::new()
            .add(DpadDown.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.el.first_down();
            mechanisms.el.second_down();
        } else if BC::new()
            .add(Cross.state(Pressed))
            .add(Circle.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.el.second_up();
            mechanisms.el.first_stop();
        } else if BC::new()
            .add(DpadLeft.state(Pressed))
            .add(DpadDown.state(Pressed))
            .evalute(contoller, &msg)
        {
            mechanisms.el.second_down();
            mechanisms.el.first_stop();
        } else {
            mechanisms.el.second_stop();
            mechanisms.el.first_stop();
        }
    }

    //re
    {
        let allow_re_mode = vec![DpadLeft, DpadRight, Circle, Square];

        {
            //left
            let re_l_mode = BC::new()
                .add(L1.state(Pressed))
                .add(R1.state(Ignore))
                .ignores(&allow_re_mode);
            if re_l_mode
                .add(DpadLeft.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.left_unfold();
            } else if re_l_mode
                .add(DpadRight.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.left_fold();
            } else {
                mechanisms.re_arm.left_stop();
            }
        }

        {
            //right
            let re_r_mode = BC::new()
                .add(R1.state(Pressed))
                .add(L1.state(Ignore))
                .ignores(&allow_re_mode);

            if re_r_mode
                .add(Circle.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.right_unfold();
            } else if re_r_mode
                .add(Square.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.right_fold();
            } else {
                mechanisms.re_arm.right_stop();
            }
        }

        {
            //center
            let re_c_mode = BC::new()
                .add(R1.state(Pressed))
                .add(L1.state(Pressed))
                .ignores(&[DpadUp, DpadDown]);

            if re_c_mode
                .add(DpadUp.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.center_unfold();
            } else if re_c_mode
                .add(DpadDown.state(Pressed))
                .evalute(contoller, &msg)
            {
                mechanisms.re_arm.center_fold();
            } else {
                mechanisms.re_arm.center_stop();
            }
        }
    }
    //ei

    {
        let ei_mode = BC::new().add(R2.state(Pressed));
        let allow_ei_mode = vec![
            DpadLeft, DpadRight, Circle, Square, Triangle, DpadUp, DpadDown, Cross,
        ];
        if ei_mode.ignores(&allow_ei_mode).evalute(contoller, &msg) {
            {
                if ei_mode.add(DpadUp.state(Pressed)).evalute(contoller, &msg) {
                    mechanisms.ei.roller_ud_up();
                } else if ei_mode
                    .add(DpadDown.state(Pressed))
                    .evalute(contoller, &msg)
                {
                    mechanisms.ei.roller_ud_down();
                } else {
                    mechanisms.ei.roller_ud_stop();
                }
            }

            if ei_mode
                .add(Circle.state(PressedEdge))
                .evalute(contoller, &msg)
            {
                mechanisms.ei.roller_toggle();
            }

            {
                if ei_mode.add(Square.state(Pressed)).evalute(contoller, &msg) {
                    mechanisms.ei.fin_unfold();
                } else if ei_mode
                    .add(Triangle.state(Pressed))
                    .evalute(contoller, &msg)
                {
                    mechanisms.ei.fin_fold();
                } else {
                    mechanisms.ei.fin_stop();
                }
            }

            {
                if ei_mode
                    .add(DpadLeft.state(Pressed))
                    .evalute(contoller, &msg)
                {
                    mechanisms.ei.ud_up();
                } else if ei_mode
                    .add(DpadRight.state(Pressed))
                    .evalute(contoller, &msg)
                {
                    mechanisms.ei.ud_down();
                } else {
                    mechanisms.ei.ud_stop();
                }
            }

            if ei_mode
                .add(Cross.state(PressedEdge))
                .evalute(contoller, &msg)
            {
                mechanisms.ei.bq_toggle();
            }
        } else {
            mechanisms.ei.fin_stop();
        }
    }

    mechanisms.re_arm.update();
    mechanisms.ro_arm.update();
    mechanisms.el.update();
    mechanisms.ei.update();
    mechanisms.omni.direcion_update();

    omni_fn(&msg, contoller, &mut mechanisms.omni);
}

fn omni_fn(msg: &TakenMsg<Joy>, contoller: &mut Gamepad<DualSenseLayout>, omni: &mut Omni) {
    let direcion = omni.direction();
    let alpha = omni.alpha();
    use controllers::Axes::*;

    let target_power = omni.omni_setting().move_chassis(
        contoller.axis(&msg, StickLX) as f64,
        contoller.axis(&msg, StickLY) as f64,
        direcion as f64 * -contoller.axis(&msg, StickRX) as f64,
    );

    let threshold = 10.;
    for (i, &tp) in target_power.iter() {
        omni.status
            .prev_motor_power
            .entry(*i)
            .and_modify(|prev| {
                let delta: f64 = alpha * (tp - *prev);
                if delta.abs() < threshold {
                    *prev = tp; // 小さい変化は直接目標値に
                } else {
                    *prev += delta;
                }
            })
            .or_insert(tp);
    }
    omni.update();
}
