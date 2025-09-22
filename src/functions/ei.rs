use super::*;
use motor_lib::{md, sd, GrpcHandle};
#[allow(unused_imports)]
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    roller: isize,
    roller_ud: isize,
    fin: isize,
    ud: isize,
    bq: isize,
}
impl Status {
    pub fn new() -> Self {
        Self {
            roller: 0,
            roller_ud: 0,
            fin: 0,
            ud: 0,
            bq: 0,
        }
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

    pub fn bq_toggle(&mut self) {
        self.status.bq = (self.status.bq + 1) % 2;
    }

    pub fn roller_toggle(&mut self) {
        self.status.roller = (self.status.roller + 1) % 2;
    }

    pub fn ud_up(&mut self) {
        self.status.ud = 1;
    }

    pub fn ud_down(&mut self) {
        self.status.ud = -1;
    }

    pub fn ud_stop(&mut self) {
        self.status.ud = 0;
    }

    pub fn fin_unfold(&mut self) {
        self.status.fin = 1;
    }

    pub fn fin_fold(&mut self) {
        self.status.fin = -1;
    }

    pub fn fin_stop(&mut self) {
        self.status.fin = 0;
    }

    pub fn roller_ud_up(&mut self) {
        self.status.roller_ud = 1;
    }

    pub fn roller_ud_down(&mut self) {
        self.status.roller_ud = -1;
    }

    pub fn roller_ud_stop(&mut self) {
        self.status.roller_ud = 0;
    }

    pub fn update(&mut self) {
        md::send_pwm(
            &self.handle,
            MdAdress::EiRoller as u8,
            (800 * self.status.roller) as i16,
        );
        md::send_pwm(
            &self.handle,
            MdAdress::EiFin as u8,
            (400 * self.status.fin) as i16,
        );

        md::send_limsw(
            &self.handle,
            MdAdress::EiUd as u8,
            if self.status.ud == 1 { 0 } else { 1 },
            (-500 * self.status.ud) as i16,
            0,
        );

        md::send_limsw(
            &self.handle,
            MdAdress::EiRollerUd as u8,
            if self.status.roller_ud == 1 { 0 } else { 1 }, //いい感じに変えてね
            (self.status.roller_ud * 400) as i16,
            0,
        );

        sd::send_power(
            &self.handle,
            SdAdress::EiBq as u8,
            0,
            self.status.bq as i16 * 200,
        );
    }
}
