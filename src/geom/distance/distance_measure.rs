use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    {
        geom::distance::distance_unit::DistanceUnit, internal::measure::Measure,
        internal::unit::Unit,
    },
};

pub struct Distance<Num> {
    base: Num,
}

jayunits_measure_factory_build_impl!(Distance, DistanceUnit);
jayunits_measure_factory_build_traits!(Distance);
