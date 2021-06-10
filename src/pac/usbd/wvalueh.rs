#[doc = "Reader of register WVALUEH"]
pub type R = crate::pac::generic::R<u32, super::WVALUEH>;
#[doc = "Reader of field `WVALUEH`"]
pub type WVALUEH_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 3, MSB of wValue"]
    #[inline(always)]
    pub fn wvalueh(&self) -> WVALUEH_R {
        WVALUEH_R::new((self.bits & 0xff) as u8)
    }
}
