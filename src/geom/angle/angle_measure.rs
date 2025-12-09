use crate::internal::unit::Unit;
use crate::{
    jayunits_measure_factory_build_impl, jayunits_measure_factory_build_traits,
    num::NumLike,
    {geom::angle::angle_unit::AngleUnit, internal::measure::Measure},
};

pub struct Angle<Num>
where
    Num: NumLike,
{
    base: Num,
}

jayunits_measure_factory_build_impl!(Angle, AngleUnit);
jayunits_measure_factory_build_traits!(Angle);
