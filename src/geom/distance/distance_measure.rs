use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    {geom::distance::distance_unit::DistanceUnit, internal::measure::Measure, internal::unit::Unit},
};

pub struct Distance<Num> {
    base: Num,
}

jayutil_unit_generate_measure_impl!(Distance, DistanceUnit);
jayutil_unit_generate_measure_traits!(Distance);
