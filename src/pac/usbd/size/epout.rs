#[doc = "Reader of register EPOUT[%s]"]
pub type R = crate::pac::generic::R<u32, super::EPOUT>;
#[doc = "Writer for register EPOUT[%s]"]
pub type W = crate::pac::generic::W<u32, super::EPOUT>;
#[doc = "Register EPOUT[%s]
`reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EPOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::pac::generic::R<u8, u8>;
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number of bytes received last in the data stage of this OUT endpoint"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W<'_> {
        SIZE_W { w: self }
    }
}
