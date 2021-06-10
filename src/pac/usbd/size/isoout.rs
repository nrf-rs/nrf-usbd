#[doc = "Reader of register ISOOUT"]
pub type R = crate::pac::generic::R<u32, super::ISOOUT>;
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::pac::generic::R<u16, u16>;
#[doc = "Zero-length data packet received\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZERO_A {
    #[doc = "0: No zero-length data received, use value in SIZE"]
    NORMAL = 0,
    #[doc = "1: Zero-length data received, ignore value in SIZE"]
    ZERODATA = 1,
}
impl From<ZERO_A> for bool {
    #[inline(always)]
    fn from(variant: ZERO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ZERO`"]
pub type ZERO_R = crate::pac::generic::R<bool, ZERO_A>;
impl ZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ZERO_A {
        match self.bits {
            false => ZERO_A::NORMAL,
            true => ZERO_A::ZERODATA,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ZERO_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ZERODATA`"]
    #[inline(always)]
    pub fn is_zero_data(&self) -> bool {
        *self == ZERO_A::ZERODATA
    }
}
impl R {
    #[doc = "Bits 0:9 - Number of bytes received last on this ISO OUT data endpoint"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Zero-length data packet received"]
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
