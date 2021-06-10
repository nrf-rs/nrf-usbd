#[doc = "Reader of register BREQUEST"]
pub type R = crate::pac::generic::R<u32, super::BREQUEST>;
#[doc = "SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BREQUEST_A {
    #[doc = "0: Standard request GET_STATUS"]
    STD_GET_STATUS = 0,
    #[doc = "1: Standard request CLEAR_FEATURE"]
    STD_CLEAR_FEATURE = 1,
    #[doc = "3: Standard request SET_FEATURE"]
    STD_SET_FEATURE = 3,
    #[doc = "5: Standard request SET_ADDRESS"]
    STD_SET_ADDRESS = 5,
    #[doc = "6: Standard request GET_DESCRIPTOR"]
    STD_GET_DESCRIPTOR = 6,
    #[doc = "7: Standard request SET_DESCRIPTOR"]
    STD_SET_DESCRIPTOR = 7,
    #[doc = "8: Standard request GET_CONFIGURATION"]
    STD_GET_CONFIGURATION = 8,
    #[doc = "9: Standard request SET_CONFIGURATION"]
    STD_SET_CONFIGURATION = 9,
    #[doc = "10: Standard request GET_INTERFACE"]
    STD_GET_INTERFACE = 10,
    #[doc = "11: Standard request SET_INTERFACE"]
    STD_SET_INTERFACE = 11,
    #[doc = "12: Standard request SYNCH_FRAME"]
    STD_SYNCH_FRAME = 12,
}
impl From<BREQUEST_A> for u8 {
    #[inline(always)]
    fn from(variant: BREQUEST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BREQUEST`"]
pub type BREQUEST_R = crate::pac::generic::R<u8, BREQUEST_A>;
impl BREQUEST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u8, BREQUEST_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            0 => Val(BREQUEST_A::STD_GET_STATUS),
            1 => Val(BREQUEST_A::STD_CLEAR_FEATURE),
            3 => Val(BREQUEST_A::STD_SET_FEATURE),
            5 => Val(BREQUEST_A::STD_SET_ADDRESS),
            6 => Val(BREQUEST_A::STD_GET_DESCRIPTOR),
            7 => Val(BREQUEST_A::STD_SET_DESCRIPTOR),
            8 => Val(BREQUEST_A::STD_GET_CONFIGURATION),
            9 => Val(BREQUEST_A::STD_SET_CONFIGURATION),
            10 => Val(BREQUEST_A::STD_GET_INTERFACE),
            11 => Val(BREQUEST_A::STD_SET_INTERFACE),
            12 => Val(BREQUEST_A::STD_SYNCH_FRAME),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STD_GET_STATUS`"]
    #[inline(always)]
    pub fn is_std_get_status(&self) -> bool {
        *self == BREQUEST_A::STD_GET_STATUS
    }
    #[doc = "Checks if the value of the field is `STD_CLEAR_FEATURE`"]
    #[inline(always)]
    pub fn is_std_clear_feature(&self) -> bool {
        *self == BREQUEST_A::STD_CLEAR_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_FEATURE`"]
    #[inline(always)]
    pub fn is_std_set_feature(&self) -> bool {
        *self == BREQUEST_A::STD_SET_FEATURE
    }
    #[doc = "Checks if the value of the field is `STD_SET_ADDRESS`"]
    #[inline(always)]
    pub fn is_std_set_address(&self) -> bool {
        *self == BREQUEST_A::STD_SET_ADDRESS
    }
    #[doc = "Checks if the value of the field is `STD_GET_DESCRIPTOR`"]
    #[inline(always)]
    pub fn is_std_get_descriptor(&self) -> bool {
        *self == BREQUEST_A::STD_GET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_SET_DESCRIPTOR`"]
    #[inline(always)]
    pub fn is_std_set_descriptor(&self) -> bool {
        *self == BREQUEST_A::STD_SET_DESCRIPTOR
    }
    #[doc = "Checks if the value of the field is `STD_GET_CONFIGURATION`"]
    #[inline(always)]
    pub fn is_std_get_configuration(&self) -> bool {
        *self == BREQUEST_A::STD_GET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_SET_CONFIGURATION`"]
    #[inline(always)]
    pub fn is_std_set_configuration(&self) -> bool {
        *self == BREQUEST_A::STD_SET_CONFIGURATION
    }
    #[doc = "Checks if the value of the field is `STD_GET_INTERFACE`"]
    #[inline(always)]
    pub fn is_std_get_interface(&self) -> bool {
        *self == BREQUEST_A::STD_GET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SET_INTERFACE`"]
    #[inline(always)]
    pub fn is_std_set_interface(&self) -> bool {
        *self == BREQUEST_A::STD_SET_INTERFACE
    }
    #[doc = "Checks if the value of the field is `STD_SYNCH_FRAME`"]
    #[inline(always)]
    pub fn is_std_synch_frame(&self) -> bool {
        *self == BREQUEST_A::STD_SYNCH_FRAME
    }
}
impl R {
    #[doc = "Bits 0:7 - SETUP data, byte 1, bRequest. Values provided for standard requests only, user must implement class and vendor values."]
    #[inline(always)]
    pub fn brequest(&self) -> BREQUEST_R {
        BREQUEST_R::new((self.bits & 0xff) as u8)
    }
}
