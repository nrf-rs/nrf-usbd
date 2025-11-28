#[doc = "Reader of register EPINEN"]
pub type R = crate::pac::generic::R<u32, super::EPINEN>;
#[doc = "Writer for register EPINEN"]
pub type W = crate::pac::generic::W<u32, super::EPINEN>;
#[doc = "Register EPINEN `reset()`'s with value 0x01"]
impl crate::pac::generic::ResetValue for super::EPINEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enable IN endpoint 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Disable endpoint IN 0 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 0 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::pac::generic::R<bool, IN0_A>;
impl IN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0_A {
        match self.bits {
            false => IN0_A::DISABLE,
            true => IN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN0_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 0 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN0_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 0 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN0_A::ENABLE)
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
#[doc = "Enable IN endpoint 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Disable endpoint IN 1 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 1 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::pac::generic::R<bool, IN1_A>;
impl IN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1_A {
        match self.bits {
            false => IN1_A::DISABLE,
            true => IN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN1_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 1 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN1_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 1 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN1_A::ENABLE)
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
#[doc = "Enable IN endpoint 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Disable endpoint IN 2 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 2 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::pac::generic::R<bool, IN2_A>;
impl IN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2_A {
        match self.bits {
            false => IN2_A::DISABLE,
            true => IN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN2_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 2 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN2_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 2 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN2_A::ENABLE)
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
#[doc = "Enable IN endpoint 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Disable endpoint IN 3 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 3 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::pac::generic::R<bool, IN3_A>;
impl IN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3_A {
        match self.bits {
            false => IN3_A::DISABLE,
            true => IN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN3_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 3 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN3_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 3 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN3_A::ENABLE)
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
#[doc = "Enable IN endpoint 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Disable endpoint IN 4 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 4 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN4_A> for bool {
    #[inline(always)]
    fn from(variant: IN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::pac::generic::R<bool, IN4_A>;
impl IN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4_A {
        match self.bits {
            false => IN4_A::DISABLE,
            true => IN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN4_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN4`"]
pub struct IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 4 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN4_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 4 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN4_A::ENABLE)
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
#[doc = "Enable IN endpoint 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Disable endpoint IN 5 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 5 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN5_A> for bool {
    #[inline(always)]
    fn from(variant: IN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::pac::generic::R<bool, IN5_A>;
impl IN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN5_A {
        match self.bits {
            false => IN5_A::DISABLE,
            true => IN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN5_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN5`"]
pub struct IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 5 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN5_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 5 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN5_A::ENABLE)
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
#[doc = "Enable IN endpoint 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Disable endpoint IN 6 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 6 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN6_A> for bool {
    #[inline(always)]
    fn from(variant: IN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::pac::generic::R<bool, IN6_A>;
impl IN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN6_A {
        match self.bits {
            false => IN6_A::DISABLE,
            true => IN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN6_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN6`"]
pub struct IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 6 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN6_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 6 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN6_A::ENABLE)
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
#[doc = "Enable IN endpoint 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Disable endpoint IN 7 (no response to IN tokens)"]
    DISABLE = 0,
    #[doc = "1: Enable endpoint IN 7 (response to IN tokens)"]
    ENABLE = 1,
}
impl From<IN7_A> for bool {
    #[inline(always)]
    fn from(variant: IN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::pac::generic::R<bool, IN7_A>;
impl IN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN7_A {
        match self.bits {
            false => IN7_A::DISABLE,
            true => IN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IN7_A::ENABLE
    }
}
#[doc = "Write proxy for field `IN7`"]
pub struct IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable endpoint IN 7 (no response to IN tokens)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IN7_A::DISABLE)
    }
    #[doc = "Enable endpoint IN 7 (response to IN tokens)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IN7_A::ENABLE)
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
#[doc = "Enable ISO IN endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISOIN_A {
    #[doc = "0: Disable ISO IN endpoint 8"]
    DISABLE = 0,
    #[doc = "1: Enable ISO IN endpoint 8"]
    ENABLE = 1,
}
impl From<ISOIN_A> for bool {
    #[inline(always)]
    fn from(variant: ISOIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISOIN`"]
pub type ISOIN_R = crate::pac::generic::R<bool, ISOIN_A>;
impl ISOIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOIN_A {
        match self.bits {
            false => ISOIN_A::DISABLE,
            true => ISOIN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISOIN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISOIN_A::ENABLE
    }
}
#[doc = "Write proxy for field `ISOIN`"]
pub struct ISOIN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISOIN_A::DISABLE)
    }
    #[doc = "Enable ISO IN endpoint 8"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISOIN_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&self) -> ISOIN_R {
        ISOIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IN endpoint 0"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W<'_> {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable IN endpoint 1"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W<'_> {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable IN endpoint 2"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W<'_> {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable IN endpoint 3"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W<'_> {
        IN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable IN endpoint 4"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W<'_> {
        IN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable IN endpoint 5"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W<'_> {
        IN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable IN endpoint 6"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W<'_> {
        IN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable IN endpoint 7"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W<'_> {
        IN7_W { w: self }
    }
    #[doc = "Bit 8 - Enable ISO IN endpoint"]
    #[inline(always)]
    pub fn isoin(&mut self) -> ISOIN_W<'_> {
        ISOIN_W { w: self }
    }
}
