use crate::geom::distance::distance_unit::DistanceUnit;
use crate::internal::unit::Unit;
use crate::time::time_unit::TimeUnit;
use crate::{
    jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct LinearVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(LinearVelocityUnit);
jayunits_unit_factory_build_traits!(LinearVelocityUnit);

jayutil_unit_motion_generate_impl!(LinearVelocityUnit, DistanceUnit, TimeUnit);
