#[doc = "Reader of register INTEN"]
pub type R = crate::pac::generic::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::pac::generic::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable interrupt for USBRESET event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBRESET_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBRESET_A> for bool {
    #[inline(always)]
    fn from(variant: USBRESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBRESET`"]
pub type USBRESET_R = crate::pac::generic::R<bool, USBRESET_A>;
impl USBRESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRESET_A {
        match self.bits {
            false => USBRESET_A::DISABLED,
            true => USBRESET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBRESET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBRESET_A::ENABLED
    }
}
#[doc = "Write proxy for field `USBRESET`"]
pub struct USBRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> USBRESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBRESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBRESET_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBRESET_A::ENABLED)
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
#[doc = "Enable or disable interrupt for STARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STARTED`"]
pub type STARTED_R = crate::pac::generic::R<bool, STARTED_A>;
impl STARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STARTED_A::ENABLED
    }
}
#[doc = "Write proxy for field `STARTED`"]
pub struct STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STARTED_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STARTED_A::ENABLED)
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
#[doc = "Enable or disable interrupt for ENDEPIN\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN0`"]
pub type ENDEPIN0_R = crate::pac::generic::R<bool, ENDEPIN0_A>;
impl ENDEPIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN0_A {
        match self.bits {
            false => ENDEPIN0_A::DISABLED,
            true => ENDEPIN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN0_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN0`"]
pub struct ENDEPIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN0_A::ENABLED)
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
#[doc = "Enable or disable interrupt for ENDEPIN\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN1`"]
pub type ENDEPIN1_R = crate::pac::generic::R<bool, ENDEPIN1_A>;
impl ENDEPIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN1_A {
        match self.bits {
            false => ENDEPIN1_A::DISABLED,
            true => ENDEPIN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN1_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN1`"]
pub struct ENDEPIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN1_A::ENABLED)
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
#[doc = "Enable or disable interrupt for ENDEPIN\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN2`"]
pub type ENDEPIN2_R = crate::pac::generic::R<bool, ENDEPIN2_A>;
impl ENDEPIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN2_A {
        match self.bits {
            false => ENDEPIN2_A::DISABLED,
            true => ENDEPIN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN2_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN2`"]
pub struct ENDEPIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN2_A::ENABLED)
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
#[doc = "Enable or disable interrupt for ENDEPIN\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN3`"]
pub type ENDEPIN3_R = crate::pac::generic::R<bool, ENDEPIN3_A>;
impl ENDEPIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN3_A {
        match self.bits {
            false => ENDEPIN3_A::DISABLED,
            true => ENDEPIN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN3_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN3`"]
pub struct ENDEPIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPIN\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN4`"]
pub type ENDEPIN4_R = crate::pac::generic::R<bool, ENDEPIN4_A>;
impl ENDEPIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN4_A {
        match self.bits {
            false => ENDEPIN4_A::DISABLED,
            true => ENDEPIN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN4_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN4`"]
pub struct ENDEPIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPIN\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN5`"]
pub type ENDEPIN5_R = crate::pac::generic::R<bool, ENDEPIN5_A>;
impl ENDEPIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN5_A {
        match self.bits {
            false => ENDEPIN5_A::DISABLED,
            true => ENDEPIN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN5_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN5`"]
pub struct ENDEPIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN5_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPIN\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN6`"]
pub type ENDEPIN6_R = crate::pac::generic::R<bool, ENDEPIN6_A>;
impl ENDEPIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN6_A {
        match self.bits {
            false => ENDEPIN6_A::DISABLED,
            true => ENDEPIN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN6_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN6`"]
pub struct ENDEPIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN6_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPIN\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPIN7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPIN7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPIN7`"]
pub type ENDEPIN7_R = crate::pac::generic::R<bool, ENDEPIN7_A>;
impl ENDEPIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPIN7_A {
        match self.bits {
            false => ENDEPIN7_A::DISABLED,
            true => ENDEPIN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPIN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPIN7_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPIN7`"]
pub struct ENDEPIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPIN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPIN7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPIN7_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable or disable interrupt for EP0DATADONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EP0DATADONE_A> for bool {
    #[inline(always)]
    fn from(variant: EP0DATADONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP0DATADONE`"]
pub type EP0DATADONE_R = crate::pac::generic::R<bool, EP0DATADONE_A>;
impl EP0DATADONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0DATADONE_A {
        match self.bits {
            false => EP0DATADONE_A::DISABLED,
            true => EP0DATADONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_A::ENABLED
    }
}
#[doc = "Write proxy for field `EP0DATADONE`"]
pub struct EP0DATADONE_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0DATADONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0DATADONE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDISOIN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOIN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDISOIN`"]
pub type ENDISOIN_R = crate::pac::generic::R<bool, ENDISOIN_A>;
impl ENDISOIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOIN_A {
        match self.bits {
            false => ENDISOIN_A::DISABLED,
            true => ENDISOIN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOIN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOIN_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDISOIN`"]
pub struct ENDISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDISOIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDISOIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDISOIN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDISOIN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT0`"]
pub type ENDEPOUT0_R = crate::pac::generic::R<bool, ENDEPOUT0_A>;
impl ENDEPOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT0_A {
        match self.bits {
            false => ENDEPOUT0_A::DISABLED,
            true => ENDEPOUT0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT0`"]
pub struct ENDEPOUT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT1`"]
pub type ENDEPOUT1_R = crate::pac::generic::R<bool, ENDEPOUT1_A>;
impl ENDEPOUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT1_A {
        match self.bits {
            false => ENDEPOUT1_A::DISABLED,
            true => ENDEPOUT1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT1_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT1`"]
pub struct ENDEPOUT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT2`"]
pub type ENDEPOUT2_R = crate::pac::generic::R<bool, ENDEPOUT2_A>;
impl ENDEPOUT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT2_A {
        match self.bits {
            false => ENDEPOUT2_A::DISABLED,
            true => ENDEPOUT2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT2_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT2`"]
pub struct ENDEPOUT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT3_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT3`"]
pub type ENDEPOUT3_R = crate::pac::generic::R<bool, ENDEPOUT3_A>;
impl ENDEPOUT3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT3_A {
        match self.bits {
            false => ENDEPOUT3_A::DISABLED,
            true => ENDEPOUT3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT3_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT3`"]
pub struct ENDEPOUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT4_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT4`"]
pub type ENDEPOUT4_R = crate::pac::generic::R<bool, ENDEPOUT4_A>;
impl ENDEPOUT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT4_A {
        match self.bits {
            false => ENDEPOUT4_A::DISABLED,
            true => ENDEPOUT4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT4_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT4`"]
pub struct ENDEPOUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT5_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT5`"]
pub type ENDEPOUT5_R = crate::pac::generic::R<bool, ENDEPOUT5_A>;
impl ENDEPOUT5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT5_A {
        match self.bits {
            false => ENDEPOUT5_A::DISABLED,
            true => ENDEPOUT5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT5_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT5`"]
pub struct ENDEPOUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT5_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT6_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT6`"]
pub type ENDEPOUT6_R = crate::pac::generic::R<bool, ENDEPOUT6_A>;
impl ENDEPOUT6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT6_A {
        match self.bits {
            false => ENDEPOUT6_A::DISABLED,
            true => ENDEPOUT6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT6_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT6`"]
pub struct ENDEPOUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT6_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDEPOUT\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDEPOUT7_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEPOUT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDEPOUT7`"]
pub type ENDEPOUT7_R = crate::pac::generic::R<bool, ENDEPOUT7_A>;
impl ENDEPOUT7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEPOUT7_A {
        match self.bits {
            false => ENDEPOUT7_A::DISABLED,
            true => ENDEPOUT7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT7_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDEPOUT7`"]
pub struct ENDEPOUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDEPOUT7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDEPOUT7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT7_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Enable or disable interrupt for ENDISOOUT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDISOOUT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ENDISOOUT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDISOOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDISOOUT`"]
pub type ENDISOOUT_R = crate::pac::generic::R<bool, ENDISOOUT_A>;
impl ENDISOOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDISOOUT_A {
        match self.bits {
            false => ENDISOOUT_A::DISABLED,
            true => ENDISOOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDISOOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDISOOUT_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDISOOUT`"]
pub struct ENDISOOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDISOOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDISOOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDISOOUT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDISOOUT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Enable or disable interrupt for SOF event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOF_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<SOF_A> for bool {
    #[inline(always)]
    fn from(variant: SOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::pac::generic::R<bool, SOF_A>;
impl SOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOF_A {
        match self.bits {
            false => SOF_A::DISABLED,
            true => SOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SOF_A::ENABLED
    }
}
#[doc = "Write proxy for field `SOF`"]
pub struct SOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SOF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SOF_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Enable or disable interrupt for USBEVENT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBEVENT_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<USBEVENT_A> for bool {
    #[inline(always)]
    fn from(variant: USBEVENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `USBEVENT`"]
pub type USBEVENT_R = crate::pac::generic::R<bool, USBEVENT_A>;
impl USBEVENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBEVENT_A {
        match self.bits {
            false => USBEVENT_A::DISABLED,
            true => USBEVENT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == USBEVENT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == USBEVENT_A::ENABLED
    }
}
#[doc = "Write proxy for field `USBEVENT`"]
pub struct USBEVENT_W<'a> {
    w: &'a mut W,
}
impl<'a> USBEVENT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBEVENT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(USBEVENT_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(USBEVENT_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Enable or disable interrupt for EP0SETUP event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0SETUP_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EP0SETUP_A> for bool {
    #[inline(always)]
    fn from(variant: EP0SETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EP0SETUP`"]
pub type EP0SETUP_R = crate::pac::generic::R<bool, EP0SETUP_A>;
impl EP0SETUP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EP0SETUP_A {
        match self.bits {
            false => EP0SETUP_A::DISABLED,
            true => EP0SETUP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EP0SETUP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EP0SETUP_A::ENABLED
    }
}
#[doc = "Write proxy for field `EP0SETUP`"]
pub struct EP0SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0SETUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EP0SETUP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0SETUP_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0SETUP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Enable or disable interrupt for EPDATA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPDATA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<EPDATA_A> for bool {
    #[inline(always)]
    fn from(variant: EPDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EPDATA`"]
pub type EPDATA_R = crate::pac::generic::R<bool, EPDATA_A>;
impl EPDATA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPDATA_A {
        match self.bits {
            false => EPDATA_A::DISABLED,
            true => EPDATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPDATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPDATA_A::ENABLED
    }
}
#[doc = "Write proxy for field `EPDATA`"]
pub struct EPDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPDATA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPDATA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EPDATA_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for USBRESET event"]
    #[inline(always)]
    pub fn usbreset(&self) -> USBRESET_R {
        USBRESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for ENDEPIN\\[0\\]
event"]
    #[inline(always)]
    pub fn endepin0(&self) -> ENDEPIN0_R {
        ENDEPIN0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for ENDEPIN\\[1\\]
event"]
    #[inline(always)]
    pub fn endepin1(&self) -> ENDEPIN1_R {
        ENDEPIN1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for ENDEPIN\\[2\\]
event"]
    #[inline(always)]
    pub fn endepin2(&self) -> ENDEPIN2_R {
        ENDEPIN2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for ENDEPIN\\[3\\]
event"]
    #[inline(always)]
    pub fn endepin3(&self) -> ENDEPIN3_R {
        ENDEPIN3_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for ENDEPIN\\[4\\]
event"]
    #[inline(always)]
    pub fn endepin4(&self) -> ENDEPIN4_R {
        ENDEPIN4_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for ENDEPIN\\[5\\]
event"]
    #[inline(always)]
    pub fn endepin5(&self) -> ENDEPIN5_R {
        ENDEPIN5_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for ENDEPIN\\[6\\]
event"]
    #[inline(always)]
    pub fn endepin6(&self) -> ENDEPIN6_R {
        ENDEPIN6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ENDEPIN\\[7\\]
event"]
    #[inline(always)]
    pub fn endepin7(&self) -> ENDEPIN7_R {
        ENDEPIN7_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for EP0DATADONE event"]
    #[inline(always)]
    pub fn ep0datadone(&self) -> EP0DATADONE_R {
        EP0DATADONE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for ENDISOIN event"]
    #[inline(always)]
    pub fn endisoin(&self) -> ENDISOIN_R {
        ENDISOIN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for ENDEPOUT\\[0\\]
event"]
    #[inline(always)]
    pub fn endepout0(&self) -> ENDEPOUT0_R {
        ENDEPOUT0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for ENDEPOUT\\[1\\]
event"]
    #[inline(always)]
    pub fn endepout1(&self) -> ENDEPOUT1_R {
        ENDEPOUT1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for ENDEPOUT\\[2\\]
event"]
    #[inline(always)]
    pub fn endepout2(&self) -> ENDEPOUT2_R {
        ENDEPOUT2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for ENDEPOUT\\[3\\]
event"]
    #[inline(always)]
    pub fn endepout3(&self) -> ENDEPOUT3_R {
        ENDEPOUT3_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable or disable interrupt for ENDEPOUT\\[4\\]
event"]
    #[inline(always)]
    pub fn endepout4(&self) -> ENDEPOUT4_R {
        ENDEPOUT4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable or disable interrupt for ENDEPOUT\\[5\\]
event"]
    #[inline(always)]
    pub fn endepout5(&self) -> ENDEPOUT5_R {
        ENDEPOUT5_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable or disable interrupt for ENDEPOUT\\[6\\]
event"]
    #[inline(always)]
    pub fn endepout6(&self) -> ENDEPOUT6_R {
        ENDEPOUT6_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable or disable interrupt for ENDEPOUT\\[7\\]
event"]
    #[inline(always)]
    pub fn endepout7(&self) -> ENDEPOUT7_R {
        ENDEPOUT7_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable or disable interrupt for ENDISOOUT event"]
    #[inline(always)]
    pub fn endisoout(&self) -> ENDISOOUT_R {
        ENDISOOUT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable or disable interrupt for SOF event"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable or disable interrupt for USBEVENT event"]
    #[inline(always)]
    pub fn usbevent(&self) -> USBEVENT_R {
        USBEVENT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable or disable interrupt for EP0SETUP event"]
    #[inline(always)]
    pub fn ep0setup(&self) -> EP0SETUP_R {
        EP0SETUP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for EPDATA event"]
    #[inline(always)]
    pub fn epdata(&self) -> EPDATA_R {
        EPDATA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for USBRESET event"]
    #[inline(always)]
    pub fn usbreset(&mut self) -> USBRESET_W<'_> {
        USBRESET_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W<'_> {
        STARTED_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for ENDEPIN\\[0\\]
event"]
    #[inline(always)]
    pub fn endepin0(&mut self) -> ENDEPIN0_W<'_> {
        ENDEPIN0_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for ENDEPIN\\[1\\]
event"]
    #[inline(always)]
    pub fn endepin1(&mut self) -> ENDEPIN1_W<'_> {
        ENDEPIN1_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for ENDEPIN\\[2\\]
event"]
    #[inline(always)]
    pub fn endepin2(&mut self) -> ENDEPIN2_W<'_> {
        ENDEPIN2_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for ENDEPIN\\[3\\]
event"]
    #[inline(always)]
    pub fn endepin3(&mut self) -> ENDEPIN3_W<'_> {
        ENDEPIN3_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for ENDEPIN\\[4\\]
event"]
    #[inline(always)]
    pub fn endepin4(&mut self) -> ENDEPIN4_W<'_> {
        ENDEPIN4_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for ENDEPIN\\[5\\]
event"]
    #[inline(always)]
    pub fn endepin5(&mut self) -> ENDEPIN5_W<'_> {
        ENDEPIN5_W { w: self }
    }
    #[doc = "Bit 8 - Enable or disable interrupt for ENDEPIN\\[6\\]
event"]
    #[inline(always)]
    pub fn endepin6(&mut self) -> ENDEPIN6_W<'_> {
        ENDEPIN6_W { w: self }
    }
    #[doc = "Bit 9 - Enable or disable interrupt for ENDEPIN\\[7\\]
event"]
    #[inline(always)]
    pub fn endepin7(&mut self) -> ENDEPIN7_W<'_> {
        ENDEPIN7_W { w: self }
    }
    #[doc = "Bit 10 - Enable or disable interrupt for EP0DATADONE event"]
    #[inline(always)]
    pub fn ep0datadone(&mut self) -> EP0DATADONE_W<'_> {
        EP0DATADONE_W { w: self }
    }
    #[doc = "Bit 11 - Enable or disable interrupt for ENDISOIN event"]
    #[inline(always)]
    pub fn endisoin(&mut self) -> ENDISOIN_W<'_> {
        ENDISOIN_W { w: self }
    }
    #[doc = "Bit 12 - Enable or disable interrupt for ENDEPOUT\\[0\\]
event"]
    #[inline(always)]
    pub fn endepout0(&mut self) -> ENDEPOUT0_W<'_> {
        ENDEPOUT0_W { w: self }
    }
    #[doc = "Bit 13 - Enable or disable interrupt for ENDEPOUT\\[1\\]
event"]
    #[inline(always)]
    pub fn endepout1(&mut self) -> ENDEPOUT1_W<'_> {
        ENDEPOUT1_W { w: self }
    }
    #[doc = "Bit 14 - Enable or disable interrupt for ENDEPOUT\\[2\\]
event"]
    #[inline(always)]
    pub fn endepout2(&mut self) -> ENDEPOUT2_W<'_> {
        ENDEPOUT2_W { w: self }
    }
    #[doc = "Bit 15 - Enable or disable interrupt for ENDEPOUT\\[3\\]
event"]
    #[inline(always)]
    pub fn endepout3(&mut self) -> ENDEPOUT3_W<'_> {
        ENDEPOUT3_W { w: self }
    }
    #[doc = "Bit 16 - Enable or disable interrupt for ENDEPOUT\\[4\\]
event"]
    #[inline(always)]
    pub fn endepout4(&mut self) -> ENDEPOUT4_W<'_> {
        ENDEPOUT4_W { w: self }
    }
    #[doc = "Bit 17 - Enable or disable interrupt for ENDEPOUT\\[5\\]
event"]
    #[inline(always)]
    pub fn endepout5(&mut self) -> ENDEPOUT5_W<'_> {
        ENDEPOUT5_W { w: self }
    }
    #[doc = "Bit 18 - Enable or disable interrupt for ENDEPOUT\\[6\\]
event"]
    #[inline(always)]
    pub fn endepout6(&mut self) -> ENDEPOUT6_W<'_> {
        ENDEPOUT6_W { w: self }
    }
    #[doc = "Bit 19 - Enable or disable interrupt for ENDEPOUT\\[7\\]
event"]
    #[inline(always)]
    pub fn endepout7(&mut self) -> ENDEPOUT7_W<'_> {
        ENDEPOUT7_W { w: self }
    }
    #[doc = "Bit 20 - Enable or disable interrupt for ENDISOOUT event"]
    #[inline(always)]
    pub fn endisoout(&mut self) -> ENDISOOUT_W<'_> {
        ENDISOOUT_W { w: self }
    }
    #[doc = "Bit 21 - Enable or disable interrupt for SOF event"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W<'_> {
        SOF_W { w: self }
    }
    #[doc = "Bit 22 - Enable or disable interrupt for USBEVENT event"]
    #[inline(always)]
    pub fn usbevent(&mut self) -> USBEVENT_W<'_> {
        USBEVENT_W { w: self }
    }
    #[doc = "Bit 23 - Enable or disable interrupt for EP0SETUP event"]
    #[inline(always)]
    pub fn ep0setup(&mut self) -> EP0SETUP_W<'_> {
        EP0SETUP_W { w: self }
    }
    #[doc = "Bit 24 - Enable or disable interrupt for EPDATA event"]
    #[inline(always)]
    pub fn epdata(&mut self) -> EPDATA_W<'_> {
        EPDATA_W { w: self }
    }
}
