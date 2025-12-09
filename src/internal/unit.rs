use std::fmt::Formatter;

pub trait Unit /*  Display */ {
    fn from_base<Num>(&self, base: Num) -> Num
    where
        Num: crate::num::NumLike;
    fn to_base<Num>(&self, value: Num) -> Num
    where
        Num: crate::num::NumLike;

    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.symbol())
    }
}

#[macro_export]
macro_rules! jayunits_unit_factory_build_traits {
    ($($t:ident), *) => {
        $(
        impl std::fmt::Display for $t {

            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::writeln!(f, "{}", <Self as crate::internal::unit::Unit>::symbol(self))
            }
        }
        )*
    };
}

#[macro_export]
macro_rules! jayunits_unit_factory_build_impl {
    ($($t: ident), *) => {
       $(
            impl $t {
                pub const fn from(scale_to_base: f64, name: &'static str, symbol: &'static str) -> Self {
                    Self {
                        scale_to_base,
                        name,
                        symbol
                    }
                }

                pub fn get_scale_to_base(&self) -> f64 {
                    self.scale_to_base.clone()
                }

            }

            impl crate::internal::unit::Unit for $t {

                fn from_base<Num>(&self, base: Num) -> Num
                    where
                        Num: crate::num::NumLike  {
                    base / Num::from_f64(self.scale_to_base)
                }

                fn to_base<Num>(&self, value: Num) -> Num
                where
                    Num: crate::num::NumLike {
                        value * Num::from_f64(self.scale_to_base)
                }

                fn name(&self) -> &'static str {
                    self.name
                }

                fn symbol(&self) -> &'static str {
                    self.symbol
                }
            }
       )*
    };
}
