#[doc = "Reader of register LOWPOWER"]
pub type R = crate::pac::generic::R<u32, super::LOWPOWER>;
#[doc = "Writer for register LOWPOWER"]
pub type W = crate::pac::generic::W<u32, super::LOWPOWER>;
#[doc = "Register LOWPOWER `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::LOWPOWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Controls USBD peripheral low-power mode during USB suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWER_A {
    #[doc = "0: Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCENORMAL = 0,
    #[doc = "1: Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOWPOWER = 1,
}
impl From<LOWPOWER_A> for bool {
    #[inline(always)]
    fn from(variant: LOWPOWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOWPOWER`"]
pub type LOWPOWER_R = crate::pac::generic::R<bool, LOWPOWER_A>;
impl LOWPOWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWPOWER_A {
        match self.bits {
            false => LOWPOWER_A::FORCENORMAL,
            true => LOWPOWER_A::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `FORCENORMAL`"]
    #[inline(always)]
    pub fn is_force_normal(&self) -> bool {
        *self == LOWPOWER_A::FORCENORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == LOWPOWER_A::LOWPOWER
    }
}
#[doc = "Write proxy for field `LOWPOWER`"]
pub struct LOWPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOWPOWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline(always)]
    pub fn force_normal(self) -> &'a mut W {
        self.variant(LOWPOWER_A::FORCENORMAL)
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LOWPOWER_A::LOWPOWER)
    }
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
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&self) -> LOWPOWER_R {
        LOWPOWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline(always)]
    pub fn lowpower(&mut self) -> LOWPOWER_W {
        LOWPOWER_W { w: self }
    }
}
