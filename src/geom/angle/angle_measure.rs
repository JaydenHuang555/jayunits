use crate::builder::unit::Unit;
use crate::{
    jayutil_unit_generate_measure_impl, jayutil_unit_generate_measure_traits,
    num::NumLike,
    {geom::angle::angle_unit::AngleUnit, builder::measure::Measure},
};

pub struct Angle<Num>
where
    Num: NumLike,
{
    base: Num,
}

jayutil_unit_generate_measure_impl!(Angle, AngleUnit);
jayutil_unit_generate_measure_traits!(Angle);
