use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    ud: isize, // 正転　1 停止　0 反転　-1
    right: isize,
}
impl Status {
    pub fn new() -> Self {
        Self { ud: 0, right: 0 }
    }
}
pub struct RoofArm {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl RoofArm {
    pub fn new<U, N>(url: U, node_name: N) -> Self
    where
        U: AsRef<str>,
        N: AsRef<str>,
    {
        Self {
            status: Status::new(),
            handle: GrpcHandle::new(url.as_ref()),
            _logger: Self::logger_new(node_name),
        }
    }

    // ud
    // ======================
    pub fn ud_up(&mut self) {
        self.status.ud = -1;
    }

    pub fn ud_down(&mut self) {
        self.status.ud = 1;
    }

    pub fn ud_stop(&mut self) {
        self.status.ud = 0;
    }

    // right
    // ======================
    pub fn right_toggle(&mut self) {
        self.status.right = (self.status.right + 1) % 2;
    }

    pub fn update(&mut self) {
        md::send_pwm(
            &self.handle,
            Adress::RoofArmUd as u8,
            (400 * self.status.ud) as i16,
        );

        md::send_pwm(
            &self.handle,
            Adress::RoofArmRight as u8,
            (-600 * self.status.right) as i16,
        );
    }
}
