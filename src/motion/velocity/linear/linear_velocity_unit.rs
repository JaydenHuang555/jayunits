use crate::geom::distance::distance_unit::DistanceUnit;
use crate::internal::unit::Unit;
use crate::time::time_unit::TimeUnit;
use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct LinearVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(LinearVelocityUnit);
jayutil_unit_generate_unit_traits!(LinearVelocityUnit);

jayutil_unit_motion_generate_impl!(LinearVelocityUnit, DistanceUnit, TimeUnit);
