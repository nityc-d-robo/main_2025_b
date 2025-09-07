use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    left: isize, // 正転　1 停止　0 反転　-1
    right: isize,
    center: isize,
}
impl Status {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            center: 0,
        }
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

    pub fn right_fold(&mut self) {
        self.status.right = 1;
    }

    pub fn right_unfold(&mut self) {
        self.status.right = -1;
    }

    pub fn right_stop(&mut self) {
        self.status.right = 0;
    }

    pub fn left_fold(&mut self) {
        self.status.left = 1;
    }

    pub fn left_unfold(&mut self) {
        self.status.left = -1;
    }

    pub fn left_stop(&mut self) {
        self.status.left = 0;
    }

    pub fn center_fold(&mut self) {
        self.status.center = 1;
    }

    pub fn center_unfold(&mut self) {
        self.status.center = -1;
    }

    pub fn center_stop(&mut self) {
        self.status.center = 0;
    }

    pub fn update(&mut self) {
        pr_info!(self._logger, "{}", self.status.left);
        md::send_pwm(
            &self.handle,
            Adress::RetainingArmLeft as u8,
            (300 * self.status.left) as i16,
        );
        md::send_pwm(
            &self.handle,
            Adress::RetainingArmRight as u8,
            (300 * self.status.right) as i16,
        );

        md::send_pwm(
            &self.handle,
            Adress::RetainingCenter as u8,
            (600 * self.status.center) as i16,
        );
    }
}
