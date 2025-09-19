use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    first_mode: isize,
    second_mode: isize,
    pub prev_first: i16,
    pub prev_second: i16,
}
impl Status {
    pub fn new() -> Self {
        Self {
            first_mode: 0,
            second_mode: 0,
            prev_first: 0,
            prev_second: 0,
        }
    }
}
pub struct Elevator {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl Elevator {
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
    }

    pub fn first_down(&mut self) {
        self.status.first_mode = -1;
    }

    pub fn first_stop(&mut self) {
        self.status.first_mode = 0;
    }

    pub fn second_up(&mut self) {
        self.status.second_mode = 1;
    }

    pub fn second_down(&mut self) {
        self.status.second_mode = -1;
    }

    pub fn second_stop(&mut self) {
        self.status.second_mode = 0;
    }

    pub fn update(&mut self) {
        md::send_limsw(
            &self.handle,
            MdAdress::ElevatorSecond as u8,
            if self.status.second_mode == 1 { 1 } else { 0 },
            (-500 * self.status.second_mode) as i16,
            0,
        );
        md::send_limsw(
            &self.handle,
            MdAdress::ElevatorFirst as u8,
            if self.status.first_mode == 1 { 0 } else { 1 },
            (500 * self.status.first_mode) as i16,
            0,
        );
    }
}
