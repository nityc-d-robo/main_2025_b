use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};
use std::sync::{Arc, RwLock};
use std::{thread, time};

use crate::NODE_NAME;

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
    logger: Logger,
}

impl RetainingArm {
    pub const FUNCTION_NAME: &'static str = "retaining_arm";

    pub fn new<U, N>(url: U, node_name: N) -> Self
    where
        U: AsRef<str>,
        N: AsRef<str>,
    {
        Self {
            status: Status::new(),
            handle: GrpcHandle::new(url.as_ref()),
            logger: Logger::new(&format!("{}/{}", node_name.as_ref(), Self::FUNCTION_NAME)),
        }
    }

    pub fn update(&mut self) {
        md::send_pwm(&self.handle, 0 as u8, (1000 * self.status.left) as i16);
        md::send_pwm(&self.handle, 1 as u8, (1000 * self.status.right) as i16);
    }
}
// entryはArcでラップしたRwLockを受け取る設計にする
