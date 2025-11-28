#[doc = "Reader of register SHORTS"]
pub type R = crate::pac::generic::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::pac::generic::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between EP0DATADONE event and STARTEPIN\\[0\\]
task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPIN0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_STARTEPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_STARTEPIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP0DATADONE_STARTEPIN0`"]
pub type EP0DATADONE_STARTEPIN0_R = crate::pac::generic::R<bool, EP0DATADONE_STARTEPIN0_A>;
impl EP0DATADONE_STARTEPIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_STARTEPIN0_A {
        match self.bits {
            false => EP0DATADONE_STARTEPIN0_A::DISABLED,
            true => EP0DATADONE_STARTEPIN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPIN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPIN0_A::ENABLED
    }
}
#[doc = "Write proxy for field `EP0DATADONE_STARTEPIN0`"]
pub struct EP0DATADONE_STARTEPIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_STARTEPIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_STARTEPIN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0_A::ENABLED)
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
#[doc = "Shortcut between EP0DATADONE event and STARTEPOUT\\[0\\]
task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPOUT0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_STARTEPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_STARTEPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP0DATADONE_STARTEPOUT0`"]
pub type EP0DATADONE_STARTEPOUT0_R = crate::pac::generic::R<bool, EP0DATADONE_STARTEPOUT0_A>;
impl EP0DATADONE_STARTEPOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_STARTEPOUT0_A {
        match self.bits {
            false => EP0DATADONE_STARTEPOUT0_A::DISABLED,
            true => EP0DATADONE_STARTEPOUT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPOUT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPOUT0_A::ENABLED
    }
}
#[doc = "Write proxy for field `EP0DATADONE_STARTEPOUT0`"]
pub struct EP0DATADONE_STARTEPOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_STARTEPOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_STARTEPOUT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Shortcut between EP0DATADONE event and EP0STATUS task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_EP0STATUS_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<EP0DATADONE_EP0STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_EP0STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP0DATADONE_EP0STATUS`"]
pub type EP0DATADONE_EP0STATUS_R = crate::pac::generic::R<bool, EP0DATADONE_EP0STATUS_A>;
impl EP0DATADONE_EP0STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_EP0STATUS_A {
        match self.bits {
            false => EP0DATADONE_EP0STATUS_A::DISABLED,
            true => EP0DATADONE_EP0STATUS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_EP0STATUS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_EP0STATUS_A::ENABLED
    }
}
#[doc = "Write proxy for field `EP0DATADONE_EP0STATUS`"]
pub struct EP0DATADONE_EP0STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_EP0STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_EP0STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUS_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Shortcut between ENDEPOUT\\[0\\]
event and EP0STATUS task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0STATUS_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_EP0STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_EP0STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT0_EP0STATUS`"]
pub type ENDEPOUT0_EP0STATUS_R = crate::pac::generic::R<bool, ENDEPOUT0_EP0STATUS_A>;
impl ENDEPOUT0_EP0STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_EP0STATUS_A {
        match self.bits {
            false => ENDEPOUT0_EP0STATUS_A::DISABLED,
            true => ENDEPOUT0_EP0STATUS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_EP0STATUS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_EP0STATUS_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT0_EP0STATUS`"]
pub struct ENDEPOUT0_EP0STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_EP0STATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_EP0STATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUS_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUS_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Shortcut between ENDEPOUT\\[0\\]
event and EP0RCVOUT task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0RCVOUT_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_EP0RCVOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_EP0RCVOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT0_EP0RCVOUT`"]
pub type ENDEPOUT0_EP0RCVOUT_R = crate::pac::generic::R<bool, ENDEPOUT0_EP0RCVOUT_A>;
impl ENDEPOUT0_EP0RCVOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_EP0RCVOUT_A {
        match self.bits {
            false => ENDEPOUT0_EP0RCVOUT_A::DISABLED,
            true => ENDEPOUT0_EP0RCVOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_EP0RCVOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_EP0RCVOUT_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT0_EP0RCVOUT`"]
pub struct ENDEPOUT0_EP0RCVOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_EP0RCVOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_EP0RCVOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUT_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between EP0DATADONE event and STARTEPIN\\[0\\]
task"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&self) -> EP0DATADONE_STARTEPIN0_R {
        EP0DATADONE_STARTEPIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between EP0DATADONE event and STARTEPOUT\\[0\\]
task"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&self) -> EP0DATADONE_STARTEPOUT0_R {
        EP0DATADONE_STARTEPOUT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between EP0DATADONE event and EP0STATUS task"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&self) -> EP0DATADONE_EP0STATUS_R {
        EP0DATADONE_EP0STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between ENDEPOUT\\[0\\]
event and EP0STATUS task"]
    #[inline(always)]
    pub fn endepout0_ep0status(&self) -> ENDEPOUT0_EP0STATUS_R {
        ENDEPOUT0_EP0STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between ENDEPOUT\\[0\\]
event and EP0RCVOUT task"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&self) -> ENDEPOUT0_EP0RCVOUT_R {
        ENDEPOUT0_EP0RCVOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between EP0DATADONE event and STARTEPIN\\[0\\]
task"]
    #[inline(always)]
    pub fn ep0datadone_startepin0(&mut self) -> EP0DATADONE_STARTEPIN0_W<'_> {
        EP0DATADONE_STARTEPIN0_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between EP0DATADONE event and STARTEPOUT\\[0\\]
task"]
    #[inline(always)]
    pub fn ep0datadone_startepout0(&mut self) -> EP0DATADONE_STARTEPOUT0_W<'_> {
        EP0DATADONE_STARTEPOUT0_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between EP0DATADONE event and EP0STATUS task"]
    #[inline(always)]
    pub fn ep0datadone_ep0status(&mut self) -> EP0DATADONE_EP0STATUS_W<'_> {
        EP0DATADONE_EP0STATUS_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between ENDEPOUT\\[0\\]
event and EP0STATUS task"]
    #[inline(always)]
    pub fn endepout0_ep0status(&mut self) -> ENDEPOUT0_EP0STATUS_W<'_> {
        ENDEPOUT0_EP0STATUS_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between ENDEPOUT\\[0\\]
event and EP0RCVOUT task"]
    #[inline(always)]
    pub fn endepout0_ep0rcvout(&mut self) -> ENDEPOUT0_EP0RCVOUT_W<'_> {
        ENDEPOUT0_EP0RCVOUT_W { w: self }
    }
}
