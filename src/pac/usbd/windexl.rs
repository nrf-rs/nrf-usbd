#[doc = "Reader of register WINDEXL"]
pub type R = crate::pac::generic::R<u32, super::WINDEXL>;
#[doc = "Reader of field `WINDEXL`"]
pub type WINDEXL_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 4, LSB of wIndex"]
    #[inline(always)]
    pub fn windexl(&self) -> WINDEXL_R {
        WINDEXL_R::new((self.bits & 0xff) as u8)
    }
}
