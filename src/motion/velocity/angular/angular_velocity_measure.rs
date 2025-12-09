use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    motion::velocity::angular::angular_velocity_unit::AngularVelocityUnit,
};

use crate::internal::measure::Measure;
use crate::internal::unit::Unit;

pub struct AngularVelocity<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(AngularVelocity, AngularVelocityUnit);
jayunits_measure_factory_build_traits!(AngularVelocity);
