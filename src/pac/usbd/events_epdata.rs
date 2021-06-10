#[doc = "Reader of register EVENTS_EPDATA"]
pub type R = crate::pac::generic::R<u32, super::EVENTS_EPDATA>;
#[doc = "Writer for register EVENTS_EPDATA"]
pub type W = crate::pac::generic::W<u32, super::EVENTS_EPDATA>;
#[doc = "Register EVENTS_EPDATA `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EVENTS_EPDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_EPDATA`"]
pub type EVENTS_EPDATA_R = crate::pac::generic::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_EPDATA`"]
pub struct EVENTS_EPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_EPDATA_W<'a> {
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
    pub fn events_epdata(&self) -> EVENTS_EPDATA_R {
        EVENTS_EPDATA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_epdata(&mut self) -> EVENTS_EPDATA_W {
        EVENTS_EPDATA_W { w: self }
    }
}
