use crate::{jayutil_unit_generate_unit_impl, jayutil_unit_generate_unit_traits, unit::Unit};

pub struct AngleUnit {
    scale_to_base: f64,
    name: &'static str,
    symbol: &'static str,
}

jayutil_unit_generate_unit_impl!(AngleUnit);
jayutil_unit_generate_unit_traits!(AngleUnit);