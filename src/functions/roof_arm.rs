use super::*;
use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    pub ud: isize, // 正転　1 停止　0 反転　-1
    pub right: isize,
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

    pub fn update(&mut self) {
        md::send_pwm(&self.handle, 2 as u8, (400 * self.status.ud) as i16);
    }
}
