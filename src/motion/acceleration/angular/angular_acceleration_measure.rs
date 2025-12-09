use crate::internal::measure::Measure;
use crate::internal::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    motion::acceleration::angular::angular_acceleration_unit::AngularAccelerationUnit,
};

pub struct AngularAcceleration<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(AngularAcceleration, AngularAccelerationUnit);
jayutil_unit_generate_measure_traits!(AngularAcceleration);
