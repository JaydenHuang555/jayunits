use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit,
};

use crate::builder::measure::Measure;
use crate::builder::unit::Unit;

pub struct AngularVelocity<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(AngularVelocity, AngularVelocityUnit);
jayutil_unit_generate_measure_traits!(AngularVelocity);
