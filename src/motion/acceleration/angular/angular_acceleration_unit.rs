use crate::motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit;
use crate::time::time_unit::TimeUnit;
use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, jayutil_unit_motion_generate_impl};
use crate::unit::Unit;


pub struct AngularAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str
}

jayutil_unit_generate_unit_impl!(AngularAccelerationUnit);
jayutil_unit_generate_unit_traits!(AngularAccelerationUnit);

jayutil_unit_motion_generate_impl!(AngularAccelerationUnit, AngularVelocityUnit, TimeUnit);

pub const RADIANS_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(1.0, "Radians per Second per Second", "rad/s²");
pub const RADIANS_PER_MINUTE_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(60.0, "Radians per Minute per Second", "rad/min·s");
pub const DEGREES_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(std::f64::consts::PI / 180.0, "Degrees per Second per Second", "°/s²");
pub const GRADIANS_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(std::f64::consts::PI / 200.0, "Gradians per Second per Second", "gon/s²");
pub const ARCMINUTES_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(std::f64::consts::PI / 10800.0, "Arcminutes per Second per Second", "′/s²");
pub const ARCSECONDS_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(std::f64::consts::PI / 648_000.0, "Arcseconds per Second per Second", "″/s²");
pub const REVOLUTIONS_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(2.0 * std::f64::consts::PI, "Revolutions per Second per Second", "rev/s²");
pub const ROTATIONS_PER_SECOND_PER_SECOND: &AngularAccelerationUnit = &AngularAccelerationUnit::from(2.0 * std::f64::consts::PI, "Rotations per Second per Second", "rot/s²");
