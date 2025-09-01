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

pub enum Adress {
    OmniFR,
    OmniFL,
    OmniBL,
    OmniBR,
    RoofArmRight,
    RoofArmUd,
    RetainingArmLeft,
    RetainingArmRight,
}
