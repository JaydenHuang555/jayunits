use crate::internal::measure::Measure;
use crate::internal::unit::Unit;
use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit,
};

pub struct LinearVelocity<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(LinearVelocity, LinearVelocityUnit);
jayunits_measure_factory_build_traits!(LinearVelocity);
