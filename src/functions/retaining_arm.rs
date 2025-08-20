use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    pub left: isize, // 正転　1 停止　0 反転　-1
    pub right: isize,
}
impl Status {
    pub fn new() -> Self {
        Self { left: 0, right: 0 }
    }
}
pub struct RetainingArm {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl RetainingArm {
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

    pub fn update(&mut self) {
        md::send_pwm(
            &self.handle,
            Adress::RetainingArmLeft as u8,
            (1000 * self.status.left) as i16,
        );
        md::send_pwm(
            &self.handle,
            Adress::RetainingArmRight as u8,
            (1000 * self.status.right) as i16,
        );
    }
}
