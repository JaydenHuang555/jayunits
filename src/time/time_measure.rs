use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    {internal::measure::Measure, time::time_unit::TimeUnit, internal::unit::Unit},
};

pub struct Time<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Time, TimeUnit);
jayutil_unit_generate_measure_traits!(Time);
