use crate::{jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits, unit::motion::acceleration::linear::linear_acceleration_unit::LinearAccelerationUnit};
use crate::unit::unit::Unit;
use crate::unit::measure::Measure;

pub struct LinearAcceleration<Num> {
    base: Num
}

jayutil_unit_generate_measure_impl!(LinearAcceleration, LinearAccelerationUnit);
jayutil_unit_generate_measure_traits!(LinearAcceleration);