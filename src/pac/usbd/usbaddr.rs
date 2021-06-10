#[doc = "Reader of register USBADDR"]
pub type R = crate::pac::generic::R<u32, super::USBADDR>;
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Device USB address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
