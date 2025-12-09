use crate::internal::unit::Unit;
use crate::motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::{
    jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct AngularAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(AngularAccelerationUnit);
jayunits_unit_factory_build_traits!(AngularAccelerationUnit);

jayutil_unit_motion_generate_impl!(AngularAccelerationUnit, AngularVelocityUnit, TimeUnit);
