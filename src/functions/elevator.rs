use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    pub mode: isize,
}
impl Status {
    pub fn new() -> Self {
        Self { mode: 0 }
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

    pub fn up(&mut self) {
        self.status.mode = -1;
    }

    pub fn down(&mut self) {
        self.status.mode = 1;
    }

    pub fn stop(&mut self) {
        self.status.mode = 0;
    }

    pub fn update(&mut self) {
        md::send_pwm(
            &self.handle,
            Adress::ElevatorFront as u8,
            (-500 * self.status.mode) as i16,
        );
        md::send_pwm(
            &self.handle,
            Adress::ElevatorBack as u8,
            (500 * self.status.mode) as i16,
        );
    }
}
