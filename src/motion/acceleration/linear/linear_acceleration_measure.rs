use crate::internal::measure::Measure;
use crate::internal::unit::Unit;
use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    motion::acceleration::linear::linear_acceleration_unit::LinearAccelerationUnit,
};

pub struct LinearAcceleration<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(LinearAcceleration, LinearAccelerationUnit);
jayunits_measure_factory_build_traits!(LinearAcceleration);
