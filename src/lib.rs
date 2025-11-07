#[cfg(feature = "math")]
pub mod math;

#[cfg(feature = "color")]
pub use palette;

#[macro_export]
macro_rules! define_spaces {
    ($p:path, $($name:ident),* $(,)?) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name;
            impl $p for $name {}
        )*
    };
    ($($name:ident),* $(,)?) => {
        define_spaces!(::stellare_types::math::Unit, $($name)*,);
    };
}
