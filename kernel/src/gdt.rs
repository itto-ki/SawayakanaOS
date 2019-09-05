// Reference: https://wiki.osdev.org/Global_Descriptor_Table
//            http://softwaretechnique.jp/OS_Development/kernel_development02.html
#[repr(packed)]
pub struct GdtEntry {
    pub segment_limit_low: u16,
    pub base_address_low:  u16,
    pub base_address_mid:  u8,
    pub access:            u8,
    pub flags_limit:       u8,
    pub base_address_high: u8,
}

impl GdtEntry {
    pub const fn new(offset: u32, limit: u32, access: u8, flags: u8) -> Self {
        GdtEntry {
            segment_limit_low: limit as u16,
            base_address_low:  offset as u16,
            base_address_mid:  (offset >> 16) as u8,
            access:            access,
            flags_limit:       flags & 0xF0 | ((limit >> 16) as u8) & 0x0F,
            base_address_high: (offset >> 24) as u8,
        }
    }
}
