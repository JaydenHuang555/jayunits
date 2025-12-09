use crate::internal::measure::Measure;
use crate::internal::unit::Unit;
use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    motion::acceleration::angular::angular_acceleration_unit::AngularAccelerationUnit,
};

pub struct AngularAcceleration<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(AngularAcceleration, AngularAccelerationUnit);
jayunits_measure_factory_build_traits!(AngularAcceleration);
