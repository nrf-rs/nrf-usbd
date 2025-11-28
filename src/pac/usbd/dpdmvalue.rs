#[doc = "Reader of register DPDMVALUE"]
pub type R = crate::pac::generic::R<u32, super::DPDMVALUE>;
#[doc = "Writer for register DPDMVALUE"]
pub type W = crate::pac::generic::W<u32, super::DPDMVALUE>;
#[doc = "Register DPDMVALUE `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::DPDMVALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "1: D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    RESUME = 1,
    #[doc = "2: D+ forced high, D- forced low (J state)"]
    J = 2,
    #[doc = "4: D+ forced low, D- forced high (K state)"]
    K = 4,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STATE`"]
pub type STATE_R = crate::pac::generic::R<u8, STATE_A>;
impl STATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u8, STATE_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            1 => Val(STATE_A::RESUME),
            2 => Val(STATE_A::J),
            4 => Val(STATE_A::K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == STATE_A::RESUME
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == STATE_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == STATE_A::K
    }
}
#[doc = "Write proxy for field `STATE`"]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "D+ forced low, D- forced high (K state) for a timing preset in hardware (50 us or 5 ms, depending on bus state)"]
    #[inline(always)]
    pub fn resume(self) -> &'a mut W {
        self.variant(STATE_A::RESUME)
    }
    #[doc = "D+ forced high, D- forced low (J state)"]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(STATE_A::J)
    }
    #[doc = "D+ forced low, D- forced high (K state)"]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(STATE_A::K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - State D+ and D- lines will be forced into by the DPDMDRIVE task"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W<'_> {
        STATE_W { w: self }
    }
}
