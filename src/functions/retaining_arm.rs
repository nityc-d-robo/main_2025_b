use super::*;
use motor_lib::{md, GrpcHandle};
#[allow(unused_imports)]
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    left: isize, // 正転　1 停止　0 反転　-1
    right: isize,
    center: isize,
    direction: isize, // 正転　1 反転　-1
}
impl Status {
    pub fn new() -> Self {
        Self {
            left: 0,
            right: 0,
            center: 0,
            direction: 1,
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

    pub fn direction(&self) -> isize {
        self.status.direction
    }

    pub fn reverse_direction(&mut self) {
        self.status.direction *= -1;
    }
    pub fn reset_direction(&mut self) {
        self.status.direction = -1;
    }

    pub fn update(&mut self) {
        // direction によって左右を入れ替える
        if self.status.direction == -1 {
            let temp = self.status.left;
            self.status.left = self.status.right;
            self.status.right = temp;
        }

        // pr_info!(self._logger, "left:{}", self.status.left);
        md::send_limsw(
            &self.handle,
            MdAdress::RetainingArmLeft as u8,
            if self.status.left == -1 { 1 } else { 0 },
            (500 * self.status.left) as i16,
            0,
        );

        // pr_info!(self._logger, "right:{}", self.status.right);
        md::send_limsw(
            &self.handle,
            MdAdress::RetainingArmRight as u8,
            if self.status.right == -1 { 0 } else { 1 },
            (500 * self.status.right) as i16,
            0,
        );

        // pr_info!(self._logger, "center:{}", self.status.center);
        md::send_pwm(
            &self.handle,
            MdAdress::RetainingCenter as u8,
            if self.status.center == 0 {
                -300
            } else {
                900 * self.status.center
            } as i16,
        );

        // direction　一応元に戻す
        if self.status.direction == -1 {
            let temp = self.status.left;
            self.status.left = self.status.right;
            self.status.right = temp;
        }
    }
}
