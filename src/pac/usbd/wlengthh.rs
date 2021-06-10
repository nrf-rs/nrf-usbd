#[doc = "Reader of register WLENGTHH"]
pub type R = crate::pac::generic::R<u32, super::WLENGTHH>;
#[doc = "Reader of field `WLENGTHH`"]
pub type WLENGTHH_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 7, MSB of wLength"]
    #[inline(always)]
    pub fn wlengthh(&self) -> WLENGTHH_R {
        WLENGTHH_R::new((self.bits & 0xff) as u8)
    }
}
