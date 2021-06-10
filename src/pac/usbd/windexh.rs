#[doc = "Reader of register WINDEXH"]
pub type R = crate::pac::generic::R<u32, super::WINDEXH>;
#[doc = "Reader of field `WINDEXH`"]
pub type WINDEXH_R = crate::pac::generic::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 5, MSB of wIndex"]
    #[inline(always)]
    pub fn windexh(&self) -> WINDEXH_R {
        WINDEXH_R::new((self.bits & 0xff) as u8)
    }
}
