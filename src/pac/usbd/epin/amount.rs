#[doc = "Reader of register AMOUNT"]
pub type R = crate::pac::generic::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0x7f) as u8)
    }
}
