#[doc = "Reader of register EVENTS_STARTED"]
pub type R = crate::pac::generic::R<u32, super::EVENTS_STARTED>;
#[doc = "Writer for register EVENTS_STARTED"]
pub type W = crate::pac::generic::W<u32, super::EVENTS_STARTED>;
#[doc = "Register EVENTS_STARTED `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EVENTS_STARTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_STARTED`"]
pub type EVENTS_STARTED_R = crate::pac::generic::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_STARTED`"]
pub struct EVENTS_STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_STARTED_W<'a> {
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
    pub fn events_started(&self) -> EVENTS_STARTED_R {
        EVENTS_STARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_started(&mut self) -> EVENTS_STARTED_W {
        EVENTS_STARTED_W { w: self }
    }
}
