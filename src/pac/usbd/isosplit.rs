#[doc = "Reader of register ISOSPLIT"]
pub type R = crate::pac::generic::R<u32, super::ISOSPLIT>;
#[doc = "Writer for register ISOSPLIT"]
pub type W = crate::pac::generic::W<u32, super::ISOSPLIT>;
#[doc = "Register ISOSPLIT `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::ISOSPLIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Controls the split of ISO buffers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SPLIT_A {
    #[doc = "0: Full buffer dedicated to either iso IN or OUT"]
    ONEDIR = 0,
    #[doc = "128: Lower half for IN, upper half for OUT"]
    HALFIN = 128,
}
impl From<SPLIT_A> for u16 {
    #[inline(always)]
    fn from(variant: SPLIT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPLIT`"]
pub type SPLIT_R = crate::pac::generic::R<u16, SPLIT_A>;
impl SPLIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u16, SPLIT_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            0 => Val(SPLIT_A::ONEDIR),
            128 => Val(SPLIT_A::HALFIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEDIR`"]
    #[inline(always)]
    pub fn is_one_dir(&self) -> bool {
        *self == SPLIT_A::ONEDIR
    }
    #[doc = "Checks if the value of the field is `HALFIN`"]
    #[inline(always)]
    pub fn is_half_in(&self) -> bool {
        *self == SPLIT_A::HALFIN
    }
}
#[doc = "Write proxy for field `SPLIT`"]
pub struct SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLIT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    #[inline(always)]
    pub fn one_dir(self) -> &'a mut W {
        self.variant(SPLIT_A::ONEDIR)
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline(always)]
    pub fn half_in(self) -> &'a mut W {
        self.variant(SPLIT_A::HALFIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&self) -> SPLIT_R {
        SPLIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline(always)]
    pub fn split(&mut self) -> SPLIT_W {
        SPLIT_W { w: self }
    }
}
