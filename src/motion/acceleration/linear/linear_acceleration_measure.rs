use crate::measure::Measure;
use crate::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    motion::acceleration::linear::linear_acceleration_unit::LinearAccelerationUnit,
};

pub struct LinearAcceleration<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(LinearAcceleration, LinearAccelerationUnit);
jayutil_unit_generate_measure_traits!(LinearAcceleration);
