use super::*;
use motor_lib::{md, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    roller: isize,
    flag: isize,
}
impl Status {
    pub fn new() -> Self {
        Self { roller: 0, flag: 0 }
    }
}
pub struct Ei {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl Ei {
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

    pub fn roller_toggle(&mut self) {
        self.status.roller = (self.status.roller + 1) % 2;
    }

    pub fn flag_fold(&mut self) {
        self.status.flag = 1;
    }

    pub fn flag_unfold(&mut self) {
        self.status.flag = -1;
    }

    pub fn flag_stop(&mut self) {
        self.status.flag = 0;
    }

    pub fn update(&mut self) {
        md::send_pwm(
            &self.handle,
            Adress::EiRoller as u8,
            (-800 * self.status.roller) as i16,
        );
        md::send_pwm(
            &self.handle,
            Adress::EiFlag as u8,
            (400 * self.status.flag) as i16,
        );
    }
}
