use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    {internal::measure::Measure, internal::unit::Unit, time::time_unit::TimeUnit},
};

pub struct Time<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(Time, TimeUnit);
jayunits_measure_factory_build_traits!(Time);
