#[doc = "Reader of register DTOGGLE"]
pub type R = crate::pac::generic::R<u32, super::DTOGGLE>;
#[doc = "Writer for register DTOGGLE"]
pub type W = crate::pac::generic::W<u32, super::DTOGGLE>;
#[doc = "Register DTOGGLE `reset()`'s with value 0x0100"]
impl crate::pac::generic::ResetValue for super::DTOGGLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::pac::generic::R<u8, u8>;
#[doc = "Write proxy for field `EP`"]
pub struct EP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Selects IN or OUT endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_A {
    #[doc = "0: Selects OUT endpoint"]
    OUT = 0,
    #[doc = "1: Selects IN endpoint"]
    IN = 1,
}
impl From<IO_A> for bool {
    #[inline(always)]
    fn from(variant: IO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IO`"]
pub type IO_R = crate::pac::generic::R<bool, IO_A>;
impl IO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_A {
        match self.bits {
            false => IO_A::OUT,
            true => IO_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == IO_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == IO_A::IN
    }
}
#[doc = "Write proxy for field `IO`"]
pub struct IO_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO_A::OUT)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO_A::IN)
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
#[doc = "Data toggle value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VALUE_A {
    #[doc = "0: No action on data toggle when writing the register with this value"]
    NOP = 0,
    #[doc = "1: Data toggle is DATA0 on endpoint set by EP and IO"]
    DATA0 = 1,
    #[doc = "2: Data toggle is DATA1 on endpoint set by EP and IO"]
    DATA1 = 2,
}
impl From<VALUE_A> for u8 {
    #[inline(always)]
    fn from(variant: VALUE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::pac::generic::R<u8, VALUE_A>;
impl VALUE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u8, VALUE_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            0 => Val(VALUE_A::NOP),
            1 => Val(VALUE_A::DATA0),
            2 => Val(VALUE_A::DATA1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == VALUE_A::NOP
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == VALUE_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == VALUE_A::DATA1
    }
}
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALUE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No action on data toggle when writing the register with this value"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(VALUE_A::NOP)
    }
    #[doc = "Data toggle is DATA0 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut W {
        self.variant(VALUE_A::DATA0)
    }
    #[doc = "Data toggle is DATA1 on endpoint set by EP and IO"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut W {
        self.variant(VALUE_A::DATA1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&self) -> IO_R {
        IO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select bulk endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IO_W {
        IO_W { w: self }
    }
    #[doc = "Bits 8:9 - Data toggle value"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
