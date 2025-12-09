use crate::internal::unit::Unit;
use crate::motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::{
    jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct LinearAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(LinearAccelerationUnit);
jayunits_unit_factory_build_traits!(LinearAccelerationUnit);

jayutil_unit_motion_generate_impl!(LinearAccelerationUnit, LinearVelocityUnit, TimeUnit);
