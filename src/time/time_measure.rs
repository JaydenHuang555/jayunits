use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    {internal::measure::Measure, internal::unit::Unit, time::time_unit::TimeUnit},
};

pub struct Time<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Time, TimeUnit);
jayutil_unit_generate_measure_traits!(Time);
