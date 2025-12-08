use crate::motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, jayutil_unit_motion_generate_impl};
use crate::unit::Unit;

pub struct LinearAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str
}

jayutil_unit_generate_unit_impl!(LinearAccelerationUnit);
jayutil_unit_generate_unit_traits!(LinearAccelerationUnit);

jayutil_unit_motion_generate_impl!(LinearAccelerationUnit, LinearVelocityUnit, TimeUnit);

pub const METERS_PER_SECOND_PER_SECOND: LinearAccelerationUnit = LinearAccelerationUnit::from(1.0, "Meter per Second^2", "m/(s^2)");