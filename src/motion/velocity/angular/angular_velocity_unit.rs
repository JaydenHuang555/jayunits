use crate::geom::angle::angle_unit::AngleUnit;
use crate::time::time_unit::TimeUnit;
use crate::internal::unit::Unit;
use crate::{
    jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits,
    jayutil_unit_motion_generate_impl,
};

pub struct AngularVelocityUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(AngularVelocityUnit);
jayutil_unit_generate_unit_traits!(AngularVelocityUnit);

jayutil_unit_motion_generate_impl!(AngularVelocityUnit, AngleUnit, TimeUnit);