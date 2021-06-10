#[doc = "Reader of register EVENTS_ENDISOIN"]
pub type R = crate::pac::generic::R<u32, super::EVENTS_ENDISOIN>;
#[doc = "Writer for register EVENTS_ENDISOIN"]
pub type W = crate::pac::generic::W<u32, super::EVENTS_ENDISOIN>;
#[doc = "Register EVENTS_ENDISOIN `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EVENTS_ENDISOIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_ENDISOIN`"]
pub type EVENTS_ENDISOIN_R = crate::pac::generic::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_ENDISOIN`"]
pub struct EVENTS_ENDISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ENDISOIN_W<'a> {
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
    pub fn events_endisoin(&self) -> EVENTS_ENDISOIN_R {
        EVENTS_ENDISOIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_endisoin(&mut self) -> EVENTS_ENDISOIN_W {
        EVENTS_ENDISOIN_W { w: self }
    }
}
