use super::Adress;
use omni_control::{Chassis, Tire};
pub const CHASSIS: Chassis = Chassis {
    fl: Tire {
        id: Adress::OmniFL as usize,
        raito: 1.,
    },
    fr: Tire {
        id: Adress::OmniFR as usize,
        raito: 1.,
    },
    br: Tire {
        id: Adress::OmniBR as usize,
        raito: 1.,
    },
    bl: Tire {
        id: Adress::OmniBL as usize,
        raito: 1.,
    },
};

pub const MAX_PAWER_INPUT: f64 = 160.;
pub const MAX_PAWER_OUTPUT: f64 = 1000.;
pub const MAX_REVOLUTION: f64 = 5400.;
