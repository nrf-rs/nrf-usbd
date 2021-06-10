#[doc = "Reader of register MAXCNT"]
pub type R = crate::pac::generic::R<u32, super::MAXCNT>;
#[doc = "Writer for register MAXCNT"]
pub type W = crate::pac::generic::W<u32, super::MAXCNT>;
#[doc = "Register MAXCNT `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::MAXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXCNT`"]
pub type MAXCNT_R = crate::pac::generic::R<u8, u8>;
#[doc = "Write proxy for field `MAXCNT`"]
pub struct MAXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum number of bytes to transfer"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W {
        MAXCNT_W { w: self }
    }
}
