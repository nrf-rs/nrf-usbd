#[doc = "Reader of register EVENTS_USBRESET"]
pub type R = crate::pac::generic::R<u32, super::EVENTS_USBRESET>;
#[doc = "Writer for register EVENTS_USBRESET"]
pub type W = crate::pac::generic::W<u32, super::EVENTS_USBRESET>;
#[doc = "Register EVENTS_USBRESET `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EVENTS_USBRESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_USBRESET`"]
pub type EVENTS_USBRESET_R = crate::pac::generic::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_USBRESET`"]
pub struct EVENTS_USBRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_USBRESET_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_usbreset(&self) -> EVENTS_USBRESET_R {
        EVENTS_USBRESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_usbreset(&mut self) -> EVENTS_USBRESET_W<'_> {
        EVENTS_USBRESET_W { w: self }
    }
}
