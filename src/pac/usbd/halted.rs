#[doc = "Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epin](epin) module"]
pub type EPIN = crate::pac::generic::Reg<u32, _EPIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPIN;
#[doc = "`read()` method returns [epin::R](epin::R) reader structure"]
impl crate::pac::generic::Readable for EPIN {}
#[doc = "Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epin;
#[doc = "Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epout](epout) module"]
pub type EPOUT = crate::pac::generic::Reg<u32, _EPOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPOUT;
#[doc = "`read()` method returns [epout::R](epout::R) reader structure"]
impl crate::pac::generic::Readable for EPOUT {}
#[doc = "Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
pub mod epout;
