use crate::motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::unit::Unit;
use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct AngularAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(AngularAccelerationUnit);
jayutil_unit_generate_unit_traits!(AngularAccelerationUnit);

jayutil_unit_motion_generate_impl!(AngularAccelerationUnit, AngularVelocityUnit, TimeUnit);