use crate::{jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits};

pub struct AngleUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(AngleUnit);
jayunits_unit_factory_build_traits!(AngleUnit);
