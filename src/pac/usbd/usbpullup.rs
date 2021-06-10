#[doc = "Reader of register USBPULLUP"]
pub type R = crate::pac::generic::R<u32, super::USBPULLUP>;
#[doc = "Writer for register USBPULLUP"]
pub type W = crate::pac::generic::W<u32, super::USBPULLUP>;
#[doc = "Register USBPULLUP `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::USBPULLUP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control of the USB pull-up on the D+ line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECT_A {
    #[doc = "0: Pull-up is disconnected"]
    DISABLED = 0,
    #[doc = "1: Pull-up is connected to D+"]
    ENABLED = 1,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONNECT`"]
pub type CONNECT_R = crate::pac::generic::R<bool, CONNECT_A>;
impl CONNECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            false => CONNECT_A::DISABLED,
            true => CONNECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONNECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONNECT_A::ENABLED
    }
}
#[doc = "Write proxy for field `CONNECT`"]
pub struct CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONNECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pull-up is disconnected"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONNECT_A::DISABLED)
    }
    #[doc = "Pull-up is connected to D+"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONNECT_A::ENABLED)
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
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control of the USB pull-up on the D+ line"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W {
        CONNECT_W { w: self }
    }
}
