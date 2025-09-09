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

pub enum Adress {
    OmniBL,
    OmniBR,
    OmniFR,
    OmniFL,
    RoofArmRight = 5,
    RoofArmUd = 6,
    ElevatorFirst = 7,
    RetainingArmLeft = 8,
    RetainingArmRight = 9,
    ElevatorSecond = 10,
    RetainingCenter,
}
