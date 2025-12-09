use crate::{jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits};

pub struct DistanceUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_traits!(DistanceUnit);
jayunits_unit_factory_build_impl!(DistanceUnit);
