#[doc = "Reader of register WVALUEL"]
pub type R = crate::pac::generic::R<u32, super::WVALUEL>;
#[doc = "Reader of field `WVALUEL`"]
pub type WVALUEL_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 2, LSB of wValue"]
    #[inline(always)]
    pub fn wvaluel(&self) -> WVALUEL_R {
        WVALUEL_R::new((self.bits & 0xff) as u8)
    }
}
