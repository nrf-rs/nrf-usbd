#[doc = "Reader of register EVENTS_ENDEPIN[%s]"]
pub type R = crate::pac::generic::R<u32, super::EVENTS_ENDEPIN>;
#[doc = "Writer for register EVENTS_ENDEPIN[%s]"]
pub type W = crate::pac::generic::W<u32, super::EVENTS_ENDEPIN>;
#[doc = "Register EVENTS_ENDEPIN[%s]
`reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EVENTS_ENDEPIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EVENTS_ENDEPIN`"]
pub type EVENTS_ENDEPIN_R = crate::pac::generic::R<bool, bool>;
#[doc = "Write proxy for field `EVENTS_ENDEPIN`"]
pub struct EVENTS_ENDEPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ENDEPIN_W<'a> {
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
    pub fn events_endepin(&self) -> EVENTS_ENDEPIN_R {
        EVENTS_ENDEPIN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_endepin(&mut self) -> EVENTS_ENDEPIN_W {
        EVENTS_ENDEPIN_W { w: self }
    }
}
