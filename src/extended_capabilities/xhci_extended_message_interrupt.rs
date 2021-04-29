//! xHCI Extended Message Interrupt Capability.

use super::ExtendedCapability;
use accessor::Mapper;
use accessor::Single;

/// xHCI Extended Message Interrupt Capability.
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct XhciExtendedMessageInterrupt {
    _id: u8,
    _next: u8,
    /// Message Control.
    pub control: MessageControl,
    /// Message Upper Address.
    pub upper_address: u32,
    /// Table Offset and BIR.
    pub table_offset: TableOffset,
}
impl<M> From<Single<XhciExtendedMessageInterrupt, M>> for ExtendedCapability<M>
where
    M: Mapper + Clone,
{
    fn from(x: Single<XhciExtendedMessageInterrupt, M>) -> Self {
        ExtendedCapability::XhciExtendedMessageInterrupt(x)
    }
}

/// Message Control.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MessageControl(u16);
impl MessageControl {
    rw_bit!(15, msi_x_enable, "MSI-X Enable");
    ro_field!(0..=10, table_size, "Table Size", u16);
}
impl_debug_from_methods! {
    MessageControl {
        msi_x_enable,
        table_size,
    }
}

/// Table Offset and BIR.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TableOffset(u32);
impl TableOffset {
    /// Returns the 8-byte aligned offset.
    #[must_use]
    pub fn offset(self) -> u32 {
        self.0 & !0b111
    }

    ro_field!(0..=2, bir, "BIR", u8);
}
impl_debug_from_methods! {
    TableOffset {
        offset,
        bir,
    }
}
