use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits};
use crate::unit::Unit;


pub struct AngularAccelerationUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str
}

jayutil_unit_generate_unit_impl!(AngularAccelerationUnit);
jayutil_unit_generate_unit_traits!(AngularAccelerationUnit);