#[doc = "Reader of register BMREQUESTTYPE"]
pub type R = crate::pac::generic::R<u32, super::BMREQUESTTYPE>;
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RECIPIENT_A {
    #[doc = "0: Device"]
    DEVICE = 0,
    #[doc = "1: Interface"]
    INTERFACE = 1,
    #[doc = "2: Endpoint"]
    ENDPOINT = 2,
    #[doc = "3: Other"]
    OTHER = 3,
}
impl From<RECIPIENT_A> for u8 {
    #[inline(always)]
    fn from(variant: RECIPIENT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RECIPIENT`"]
pub type RECIPIENT_R = crate::pac::generic::R<u8, RECIPIENT_A>;
impl RECIPIENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u8, RECIPIENT_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            0 => Val(RECIPIENT_A::DEVICE),
            1 => Val(RECIPIENT_A::INTERFACE),
            2 => Val(RECIPIENT_A::ENDPOINT),
            3 => Val(RECIPIENT_A::OTHER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DEVICE`"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == RECIPIENT_A::DEVICE
    }
    #[doc = "Checks if the value of the field is `INTERFACE`"]
    #[inline(always)]
    pub fn is_interface(&self) -> bool {
        *self == RECIPIENT_A::INTERFACE
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == RECIPIENT_A::ENDPOINT
    }
    #[doc = "Checks if the value of the field is `OTHER`"]
    #[inline(always)]
    pub fn is_other(&self) -> bool {
        *self == RECIPIENT_A::OTHER
    }
}
#[doc = "Data transfer type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TYPE_A {
    #[doc = "0: Standard"]
    STANDARD = 0,
    #[doc = "1: Class"]
    CLASS = 1,
    #[doc = "2: Vendor"]
    VENDOR = 2,
}
impl From<TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TYPE`"]
pub type TYPE_R = crate::pac::generic::R<u8, TYPE_A>;
impl TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::pac::generic::Variant<u8, TYPE_A> {
        use crate::pac::generic::Variant::*;
        match self.bits {
            0 => Val(TYPE_A::STANDARD),
            1 => Val(TYPE_A::CLASS),
            2 => Val(TYPE_A::VENDOR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TYPE_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `CLASS`"]
    #[inline(always)]
    pub fn is_class(&self) -> bool {
        *self == TYPE_A::CLASS
    }
    #[doc = "Checks if the value of the field is `VENDOR`"]
    #[inline(always)]
    pub fn is_vendor(&self) -> bool {
        *self == TYPE_A::VENDOR
    }
}
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRECTION_A {
    #[doc = "0: Host-to-device"]
    HOSTTODEVICE = 0,
    #[doc = "1: Device-to-host"]
    DEVICETOHOST = 1,
}
impl From<DIRECTION_A> for bool {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIRECTION`"]
pub type DIRECTION_R = crate::pac::generic::R<bool, DIRECTION_A>;
impl DIRECTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRECTION_A {
        match self.bits {
            false => DIRECTION_A::HOSTTODEVICE,
            true => DIRECTION_A::DEVICETOHOST,
        }
    }
    #[doc = "Checks if the value of the field is `HOSTTODEVICE`"]
    #[inline(always)]
    pub fn is_host_to_device(&self) -> bool {
        *self == DIRECTION_A::HOSTTODEVICE
    }
    #[doc = "Checks if the value of the field is `DEVICETOHOST`"]
    #[inline(always)]
    pub fn is_device_to_host(&self) -> bool {
        *self == DIRECTION_A::DEVICETOHOST
    }
}
impl R {
    #[doc = "Bits 0:4 - Data transfer type"]
    #[inline(always)]
    pub fn recipient(&self) -> RECIPIENT_R {
        RECIPIENT_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Data transfer type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
