use crate::{jayunits_unit_factory_build_impl, jayunits_unit_factory_build_traits};

pub struct TimeUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayunits_unit_factory_build_impl!(TimeUnit);
jayunits_unit_factory_build_traits!(TimeUnit);
