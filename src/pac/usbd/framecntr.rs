#[doc = "Reader of register FRAMECNTR"]
pub type R = crate::pac::generic::R<u32, super::FRAMECNTR>;
#[doc = "Reader of field `FRAMECNTR`"]
pub type FRAMECNTR_R = crate::pac::generic::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Returns the current value of the start of frame counter"]
    #[inline(always)]
    pub fn framecntr(&self) -> FRAMECNTR_R {
        FRAMECNTR_R::new((self.bits & 0x07ff) as u16)
    }
}
