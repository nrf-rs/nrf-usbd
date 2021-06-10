#[doc = "Reader of register PTR"]
pub type R = crate::pac::generic::R<u32, super::PTR>;
#[doc = "Writer for register PTR"]
pub type W = crate::pac::generic::W<u32, super::PTR>;
#[doc = "Register PTR `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::PTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PTR`"]
pub type PTR_R = crate::pac::generic::R<u32, u32>;
#[doc = "Write proxy for field `PTR`"]
pub struct PTR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data pointer. Accepts any address in Data RAM."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data pointer. Accepts any address in Data RAM."]
    #[inline(always)]
    pub fn ptr(&mut self) -> PTR_W {
        PTR_W { w: self }
    }
}
