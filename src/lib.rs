pub mod prelude {
    pub use syntax_core::prelude::*;
    #[cfg(feature = "macros")]
    pub use syntax_macros::deserialize_by;
}

pub use syntax_core::*;
#[cfg(feature = "macros")]
pub use syntax_macros::*;
