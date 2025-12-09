use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    {builder::measure::Measure, time::time_unit::TimeUnit, builder::unit::Unit},
};

pub struct Time<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Time, TimeUnit);
jayutil_unit_generate_measure_traits!(Time);
