use crate::motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::internal::unit::Unit;
use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct LinearAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(LinearAccelerationUnit);
jayutil_unit_generate_unit_traits!(LinearAccelerationUnit);

jayutil_unit_motion_generate_impl!(LinearAccelerationUnit, LinearVelocityUnit, TimeUnit);