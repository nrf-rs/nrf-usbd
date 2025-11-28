#[doc = "Reader of register ISOINCONFIG"]
pub type R = crate::pac::generic::R<u32, super::ISOINCONFIG>;
#[doc = "Writer for register ISOINCONFIG"]
pub type W = crate::pac::generic::W<u32, super::ISOINCONFIG>;
#[doc = "Register ISOINCONFIG `reset()`'s with value 0"]
impl crate::pac::generic::ResetValue for super::ISOINCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPONSE_A {
    #[doc = "0: Endpoint does not respond in that case"]
    NORESP = 0,
    #[doc = "1: Endpoint responds with a zero-length data packet in that case"]
    ZERODATA = 1,
}
impl From<RESPONSE_A> for bool {
    #[inline(always)]
    fn from(variant: RESPONSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESPONSE`"]
pub type RESPONSE_R = crate::pac::generic::R<bool, RESPONSE_A>;
impl RESPONSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESPONSE_A {
        match self.bits {
            false => RESPONSE_A::NORESP,
            true => RESPONSE_A::ZERODATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORESP`"]
    #[inline(always)]
    pub fn is_no_resp(&self) -> bool {
        *self == RESPONSE_A::NORESP
    }
    #[doc = "Checks if the value of the field is `ZERODATA`"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == RESPONSE_A::ZERODATA
    }
}
#[doc = "Write proxy for field `RESPONSE`"]
pub struct RESPONSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPONSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESPONSE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Endpoint does not respond in that case"]
    #[inline(always)]
    pub fn no_resp(self) -> &'a mut W {
        self.variant(RESPONSE_A::NORESP)
    }
    #[doc = "Endpoint responds with a zero-length data packet in that case"]
    #[inline(always)]
    pub fn zero_data(self) -> &'a mut W {
        self.variant(RESPONSE_A::ZERODATA)
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
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&self) -> RESPONSE_R {
        RESPONSE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    #[inline(always)]
    pub fn response(&mut self) -> RESPONSE_W<'_> {
        RESPONSE_W { w: self }
    }
}
