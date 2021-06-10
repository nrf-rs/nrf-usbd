#[doc = "Reader of register AMOUNT"]
pub type R = crate::pac::generic::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::pac::generic::R<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - Number of bytes transferred in the last transaction"]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0x03ff) as u16)
    }
}
