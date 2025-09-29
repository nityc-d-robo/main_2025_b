use std::collections::HashMap;

use super::MdAdress;
use super::*;
use motor_lib::{md, sd, GrpcHandle};
use omni_control::{Chassis, OmniSetting, Tire};
#[allow(unused_imports)]
use safe_drive::{logger::Logger, pr_info};

pub struct Status {
    direction: isize, // 正転　-1 停止　0 反転　1 回路が悪い
    pub prev_motor_power: HashMap<usize, f64>,
    omni_setting: OmniSetting,
    alpha: f64,
}
impl Status {
    pub fn new() -> Self {
        Self {
            direction: -1,
            prev_motor_power: HashMap::new(),
            omni_setting: OmniSetting {
                chassis: Chassis {
                    fl: Tire {
                        id: MdAdress::OmniFL as usize,
                        raito: 1.,
                    },
                    fr: Tire {
                        id: MdAdress::OmniFR as usize,
                        raito: 1.,
                    },
                    br: Tire {
                        id: MdAdress::OmniBR as usize,
                        raito: 1.,
                    },
                    bl: Tire {
                        id: MdAdress::OmniBL as usize,
                        raito: 1.,
                    },
                },

                max_pawer_input: MAX_PAWER_INPUT,
                max_pawer_output: MAX_PAWER_OUTPUT_NORMAL,
                max_revolution: MAX_REVOLUTION,
            },
            alpha: 1.0,
        }
    }
}
pub struct Omni {
    pub status: Status,
    handle: GrpcHandle,
    _logger: Logger,
}

impl Omni {
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

    pub fn omni_setting(&mut self) -> &mut OmniSetting {
        &mut self.status.omni_setting
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

    pub fn alpha_set(&mut self, alpha: f64) {
        self.status.alpha = alpha;
    }

    pub fn alpha(&self) -> f64 {
        self.status.alpha
    }

    pub fn max_pawer_output_set(&mut self) {
        self.status.omni_setting.max_pawer_output = MAX_PAWER_OUTPUT_BOOST;
    }

    pub fn max_pawer_output_reset(&mut self) {
        self.status.omni_setting.max_pawer_output = MAX_PAWER_OUTPUT_NORMAL;
    }

    pub fn direcion_update(&self) {
        if self.direction() == 1 {
            sd::send_power(
                &self.handle,
                SdAdress::HeadBq as u8,
                1,
                (999 as f32 * (40. / 100.)) as i16,
            );
            sd::send_power(&self.handle, SdAdress::EiBq as u8, 1, 0);
        } else {
            sd::send_power(
                &self.handle,
                SdAdress::EiBq as u8,
                1,
                (999 as f32 * (40. / 100.)) as i16,
            );
            sd::send_power(&self.handle, SdAdress::HeadBq as u8, 1, 0);
        }
    }

    pub fn update(&self) {
        // pr_info!(self._logger, "direction:{}", self.direction());

        let powers: &HashMap<usize, f64> = &self.status.prev_motor_power;
        for i in powers.keys() {
            {
                // pr_info!(self._logger, "power_{}:{}", i, -powers[i]);

                md::send_speed(
                    &self.handle,
                    *i as u8,
                    -powers[i] as i16 * self.status.direction as i16,
                );
            }
        }
    }
}

pub const MAX_PAWER_INPUT: f64 = 1.;
pub const MAX_REVOLUTION: f64 = 1.;

pub const MAX_PAWER_OUTPUT_NORMAL: f64 = 80.;
pub const MAX_PAWER_OUTPUT_BOOST: f64 = 999. / 4.;
