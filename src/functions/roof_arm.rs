use super::*;
use motor_lib::{md, sd, smd, GrpcHandle};
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    ud: isize, // 正転　1 停止　0 反転　-1
    right: isize,
    bq: isize,
    roof: f64,
}
impl Status {
    pub fn new() -> Self {
        Self {
            ud: 0,
            right: 0,
            bq: 0,
            roof: 0.0,
        }
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

    //
    pub fn roof_ud_right(&mut self, dx: f64) {
        self.status.roof += dx.max(0.);
    }

    pub fn roller_ud_left(&mut self, dx: f64) {
        self.status.roof -= dx.max(0.);
    }

    // ud
    // ======================
    pub fn ud_up(&mut self) {
        self.status.ud = -1;
    }

    pub fn ud_down(&mut self) {
        self.status.ud = 1;
    }

    pub fn ud_stop(&mut self) {
        self.status.ud = 0;
    }

    // right
    // ======================
    pub fn right_toggle(&mut self) {
        self.status.right = (self.status.right + 1) % 2;
    }

    pub fn bq_toggle(&mut self) {
        self.status.bq = (self.status.bq + 1) % 2;
    }

    pub fn right_start(&mut self) {
        self.status.right = 1;
    }
    pub fn right_stop(&mut self) {
        self.status.right = 0;
    }

    pub fn update(&mut self) {
        md::send_limsw(
            &self.handle,
            MdAdress::RoofArmUd as u8,
            if self.status.ud == 1 { 1 } else { 0 },
            (400 * self.status.ud) as i16,
            0,
        );

        md::send_pwm(
            &self.handle,
            MdAdress::RoofArmRight as u8,
            (-800 * self.status.right) as i16,
        );

        // let _ = sd::send_power(
        //     &self.handle,
        //     SdAdress::EiBq as u8,
        //     0,
        //     self.status.bq as i16 * 999,
        // );
        // let _ = smd::send_angle(
        //     &self.handle,
        //     SmdAdress::Roof as u8,
        //     0,
        //     self.status.roof.min(360.).max(0.) as i16,
        // );
    }
}
