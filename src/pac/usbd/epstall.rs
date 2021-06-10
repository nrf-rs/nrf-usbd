#[doc = "Writer for register EPSTALL"]
pub type W = crate::pac::generic::W<u32, super::EPSTALL>;
#[doc = "Register EPSTALL `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::EPSTALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
pub enum IO_AW {
    #[doc = "0: Selects OUT endpoint"]
    OUT = 0,
    #[doc = "1: Selects IN endpoint"]
    IN = 1,
}
impl From<IO_AW> for bool {
    #[inline(always)]
    fn from(variant: IO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IO`"]
pub struct IO_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Selects OUT endpoint"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(IO_AW::OUT)
    }
    #[doc = "Selects IN endpoint"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(IO_AW::IN)
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
#[doc = "Stall selected endpoint\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STALL_AW {
    #[doc = "0: Don't stall selected endpoint"]
    UNSTALL = 0,
    #[doc = "1: Stall selected endpoint"]
    STALL = 1,
}
impl From<STALL_AW> for bool {
    #[inline(always)]
    fn from(variant: STALL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STALL`"]
pub struct STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> STALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STALL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Don't stall selected endpoint"]
    #[inline(always)]
    pub fn un_stall(self) -> &'a mut W {
        self.variant(STALL_AW::UNSTALL)
    }
    #[doc = "Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut W {
        self.variant(STALL_AW::STALL)
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
impl W {
    #[doc = "Bits 0:2 - Select endpoint number"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W {
        EP_W { w: self }
    }
    #[doc = "Bit 7 - Selects IN or OUT endpoint"]
    #[inline(always)]
    pub fn io(&mut self) -> IO_W {
        IO_W { w: self }
    }
    #[doc = "Bit 8 - Stall selected endpoint"]
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W {
        STALL_W { w: self }
    }
}
