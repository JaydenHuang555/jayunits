use crate::builder::measure::Measure;
use crate::builder::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    motion::velocity::linear::linear_velocity_unit::LinearVelocityUnit,
};

pub struct LinearVelocity<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(LinearVelocity, LinearVelocityUnit);
jayutil_unit_generate_measure_traits!(LinearVelocity);
