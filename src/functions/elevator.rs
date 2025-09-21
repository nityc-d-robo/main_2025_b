use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    first_mode: isize,
    second_mode: isize,
    pub prev_first: i16, // 実際に送る直前の速度
    pub prev_second: i16,
    pub target_first: i16, // 目標速度
    pub target_second: i16,
}

impl Status {
    pub fn new() -> Self {
        Self {
            first_mode: 0,
            second_mode: 0,
            prev_first: 0,
            prev_second: 0,
            target_first: 0,
            target_second: 0,
        }
    }
}
pub struct Elevator {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl Elevator {
    const MAX_SPEED: i16 = 900;
    const BASE_SPEED: i16 = 300;
    const ACC_STEP: i16 = 200; // 1回の update での速度変化幅
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

    pub fn first_up(&mut self) {
        self.status.first_mode = 1;
        self.status.target_first = Self::MAX_SPEED;
        self.status.prev_first = self.status.prev_first.max(Self::BASE_SPEED);
    }

    pub fn first_down(&mut self) {
        self.status.first_mode = -1;
        self.status.target_first = -Self::MAX_SPEED;
        self.status.prev_first = self.status.prev_first.min(-Self::BASE_SPEED);
    }

    pub fn first_stop(&mut self) {
        self.status.first_mode = 0;
        self.status.target_first = 0;
    }

    pub fn second_up(&mut self) {
        self.status.second_mode = 1;
        self.status.target_second = Self::MAX_SPEED;
        self.status.prev_second = self.status.prev_second.max(Self::BASE_SPEED);
    }

    pub fn second_down(&mut self) {
        self.status.second_mode = -1;
        self.status.target_second = -Self::MAX_SPEED;
        self.status.prev_second = self.status.prev_second.min(-Self::BASE_SPEED);
    }

    pub fn second_stop(&mut self) {
        self.status.second_mode = 0;
        self.status.target_second = 0;
    }

    fn approach(current: &mut i16, target: i16) {
        if *current < target {
            *current = (*current + Self::ACC_STEP).min(target);
        } else if *current > target {
            *current = (*current - Self::ACC_STEP).max(target);
        }
    }

    pub fn update(&mut self) {
        // スピードをターゲットに向けて徐々に変える
        Self::approach(&mut self.status.prev_first, self.status.target_first);
        Self::approach(&mut self.status.prev_second, self.status.target_second);

        md::send_limsw(
            &self.handle,
            MdAdress::ElevatorSecond as u8,
            if self.status.second_mode == 1 { 1 } else { 0 },
            -self.status.prev_second,
            0,
        );
        md::send_limsw(
            &self.handle,
            MdAdress::ElevatorFirst as u8,
            if self.status.first_mode == 1 { 0 } else { 1 },
            self.status.prev_first,
            0,
        );
    }
}
