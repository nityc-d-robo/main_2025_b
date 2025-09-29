use super::*;
use motor_lib::{md, sd, GrpcHandle};
#[allow(unused_imports)]
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    ud: isize, // 正転　1 停止　0 反転　-1
    right: isize,
    bq: isize,
    roof: isize,
}
impl Status {
    pub fn new() -> Self {
        Self {
            ud: 0,
            right: 0,
            bq: 0,
            roof: 0,
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
    pub fn roof_right(&mut self) {
        self.status.roof = 1;
    }

    pub fn roof_left(&mut self) {
        self.status.roof = -1;
    }

    pub fn roof_stop(&mut self) {
        self.status.roof = 0;
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
        // pr_info!(self._logger, "ud:{}", self.status.ud);
        md::send_limsw(
            &self.handle,
            MdAdress::RoofArmUd as u8,
            if self.status.ud == 1 { 1 } else { 0 },
            (400 * self.status.ud) as i16,
            0,
        );

        // pr_info!(self._logger, "right:{}", self.status.right);
        md::send_pwm(
            &self.handle,
            MdAdress::RoofArmRight as u8,
            (-600 * self.status.right) as i16,
        );

        // pr_info!(self._logger, "bq:{}", self.status.bq);
        sd::send_power(
            &self.handle,
            SdAdress::HeadBq as u8,
            0,
            self.status.bq as i16 * 200,
        );

        // pr_info!(self._logger, "roof:{}", self.status.roof);
        md::send_pwm(
            &self.handle,
            MdAdress::Roof as u8,
            (300 * self.status.roof) as i16,
        );
    }
}
