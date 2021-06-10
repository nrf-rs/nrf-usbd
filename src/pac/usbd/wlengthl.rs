#[doc = "Reader of register WLENGTHL"]
pub type R = crate::pac::generic::R<u32, super::WLENGTHL>;
#[doc = "Reader of field `WLENGTHL`"]
pub type WLENGTHL_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 6, LSB of wLength"]
    #[inline(always)]
    pub fn wlengthl(&self) -> WLENGTHL_R {
        WLENGTHL_R::new((self.bits & 0xff) as u8)
    }
}
