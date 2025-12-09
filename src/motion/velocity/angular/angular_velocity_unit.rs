use crate::geom::angle::angle_unit::AngleUnit;
use crate::internal::unit::Unit;
use crate::time::time_unit::TimeUnit;
use crate::{
    jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct AngularVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(AngularVelocityUnit);
jayunits_unit_factory_build_traits!(AngularVelocityUnit);

jayutil_unit_motion_generate_impl!(AngularVelocityUnit, AngleUnit, TimeUnit);
