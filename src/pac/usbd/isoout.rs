#[doc = "Data pointer\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptr](ptr) module"]
pub type PTR = crate::pac::generic::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::pac::generic::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::pac::generic::Writable for PTR {}
#[doc = "Data pointer"]
pub mod ptr;
#[doc = "Maximum number of bytes to transfer\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxcnt](maxcnt) module"]
pub type MAXCNT = crate::pac::generic::Reg<u32, _MAXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXCNT;
#[doc = "`read()` method returns [maxcnt::R](maxcnt::R) reader structure"]
impl crate::pac::generic::Readable for MAXCNT {}
#[doc = "`write(|w| ..)` method takes [maxcnt::W](maxcnt::W) writer structure"]
impl crate::pac::generic::Writable for MAXCNT {}
#[doc = "Maximum number of bytes to transfer"]
pub mod maxcnt;
#[doc = "Number of bytes transferred in the last transaction\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](amount) module"]
pub type AMOUNT = crate::pac::generic::Reg<u32, _AMOUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMOUNT;
#[doc = "`read()` method returns [amount::R](amount::R) reader structure"]
impl crate::pac::generic::Readable for AMOUNT {}
#[doc = "Number of bytes transferred in the last transaction"]
pub mod amount;
