#![no_std]
#![feature(generic_associated_types)]
#![feature(asm)]
#![feature(type_alias_impl_trait)]
#![feature(min_type_alias_impl_trait)]
#![allow(incomplete_features)]

#[cfg(not(any(feature = "stm32l0x1", feature = "stm32l0x2", feature = "stm32l0x3",)))]
compile_error!(
    "No chip feature activated. You must activate exactly one of the following features: "
);

#[cfg(any(
    all(feature = "stm32l0x1", feature = "stm32l0x2"),
    all(feature = "stm32l0x1", feature = "stm32l0x3"),
    all(feature = "stm32l0x2", feature = "stm32l0x3"),
))]
compile_error!(
    "Multile chip features activated. You must activate exactly one of the following features: "
);

pub use stm32l0xx_hal as hal;
pub use stm32l0xx_hal::pac;

// This mod MUST go first, so that the others see its macros.
pub(crate) mod fmt;

pub mod exti;
pub mod interrupt;
