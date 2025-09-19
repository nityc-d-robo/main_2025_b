pub mod ei;
pub mod elevator;
pub mod omni;
pub mod retaining_arm;
pub mod roof_arm;

use crate::Logger;
pub trait HasFunctionName {
    const FUNCTION_NAME: &'static str;

    fn logger_new<N>(node_name: N) -> Logger
    where
        N: AsRef<str>,
    {
        Logger::new(&format!("{}/{}", node_name.as_ref(), Self::FUNCTION_NAME))
    }
}

impl HasFunctionName for retaining_arm::RetainingArm {
    const FUNCTION_NAME: &'static str = "retaining_arm";
}

impl HasFunctionName for roof_arm::RoofArm {
    const FUNCTION_NAME: &'static str = "roof_arm";
}

impl HasFunctionName for elevator::Elevator {
    const FUNCTION_NAME: &'static str = "elevator";
}

impl HasFunctionName for ei::Ei {
    const FUNCTION_NAME: &'static str = "ei";
}
impl HasFunctionName for omni::Omni {
    const FUNCTION_NAME: &'static str = "omni";
}

pub enum MdAdress {
    OmniFR = 0,
    OmniBL = 1,
    OmniFL = 2,
    OmniBR = 3,
    ElevatorFirst = 4,
    EiUd,
    ElevatorSecond = 6,
    RetainingArmRight = 11,
    RetainingArmLeft = 8,
    RetainingCenter = 9,
    RoofArmRight = 7,
    RoofArmUd = 12,
    EiRoller,
    EiFin,
}

pub enum SmdAdress {
    EiRollerUd = 0,
}

pub enum SdAdress {
    HeadBq = 0,
    EiBq = 1,
}
