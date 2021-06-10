#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
    pub tasks_startepin: [TASKS_STARTEPIN; 8],
    #[doc = "0x24 - Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
    pub tasks_startisoin: TASKS_STARTISOIN,
    #[doc = "0x28 - Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
    pub tasks_startepout: [TASKS_STARTEPOUT; 8],
    #[doc = "0x48 - Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
    pub tasks_startisoout: TASKS_STARTISOOUT,
    #[doc = "0x4c - Allows OUT data stage on control endpoint 0"]
    pub tasks_ep0rcvout: TASKS_EP0RCVOUT,
    #[doc = "0x50 - Allows status stage on control endpoint 0"]
    pub tasks_ep0status: TASKS_EP0STATUS,
    #[doc = "0x54 - Stalls data and status stage on control endpoint 0"]
    pub tasks_ep0stall: TASKS_EP0STALL,
    #[doc = "0x58 - Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
    pub tasks_dpdmdrive: TASKS_DPDMDRIVE,
    #[doc = "0x5c - Stops forcing D+ and D- lines into any state (USB engine takes control)"]
    pub tasks_dpdmnodrive: TASKS_DPDMNODRIVE,
    _reserved9: [u8; 160usize],
    #[doc = "0x100 - Signals that a USB reset condition has been detected on USB lines"]
    pub events_usbreset: EVENTS_USBRESET,
    #[doc = "0x104 - Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
    pub events_started: EVENTS_STARTED,
    #[doc = "0x108 - Description collection\\[n\\]: The whole EPIN\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepin: [EVENTS_ENDEPIN; 8],
    #[doc = "0x128 - An acknowledged data transfer has taken place on the control endpoint"]
    pub events_ep0datadone: EVENTS_EP0DATADONE,
    #[doc = "0x12c - The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoin: EVENTS_ENDISOIN,
    #[doc = "0x130 - Description collection\\[n\\]: The whole EPOUT\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endepout: [EVENTS_ENDEPOUT; 8],
    #[doc = "0x150 - The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
    pub events_endisoout: EVENTS_ENDISOOUT,
    #[doc = "0x154 - Signals that a SOF (start of frame) condition has been detected on USB lines"]
    pub events_sof: EVENTS_SOF,
    #[doc = "0x158 - An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
    pub events_usbevent: EVENTS_USBEVENT,
    #[doc = "0x15c - A valid SETUP token has been received (and acknowledged) on the control endpoint"]
    pub events_ep0setup: EVENTS_EP0SETUP,
    #[doc = "0x160 - A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
    pub events_epdata: EVENTS_EPDATA,
    _reserved20: [u8; 156usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved21: [u8; 252usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved24: [u8; 244usize],
    #[doc = "0x400 - Details on what caused the USBEVENT event"]
    pub eventcause: EVENTCAUSE,
    _reserved25: [u8; 28usize],
    #[doc = "0x420 - Unspecified"]
    pub halted: HALTED,
    _reserved26: [u8; 4usize],
    #[doc = "0x468 - Provides information on which endpoint's EasyDMA registers have been captured"]
    pub epstatus: EPSTATUS,
    #[doc = "0x46c - Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
    pub epdatastatus: EPDATASTATUS,
    #[doc = "0x470 - Device USB address"]
    pub usbaddr: USBADDR,
    _reserved29: [u8; 12usize],
    #[doc = "0x480 - SETUP data, byte 0, bmRequestType"]
    pub bmrequesttype: BMREQUESTTYPE,
    #[doc = "0x484 - SETUP data, byte 1, bRequest"]
    pub brequest: BREQUEST,
    #[doc = "0x488 - SETUP data, byte 2, LSB of wValue"]
    pub wvaluel: WVALUEL,
    #[doc = "0x48c - SETUP data, byte 3, MSB of wValue"]
    pub wvalueh: WVALUEH,
    #[doc = "0x490 - SETUP data, byte 4, LSB of wIndex"]
    pub windexl: WINDEXL,
    #[doc = "0x494 - SETUP data, byte 5, MSB of wIndex"]
    pub windexh: WINDEXH,
    #[doc = "0x498 - SETUP data, byte 6, LSB of wLength"]
    pub wlengthl: WLENGTHL,
    #[doc = "0x49c - SETUP data, byte 7, MSB of wLength"]
    pub wlengthh: WLENGTHH,
    #[doc = "0x4a0 - Unspecified"]
    pub size: SIZE,
    _reserved38: [u8; 60usize],
    #[doc = "0x500 - Enable USB"]
    pub enable: ENABLE,
    #[doc = "0x504 - Control of the USB pull-up"]
    pub usbpullup: USBPULLUP,
    #[doc = "0x508 - State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
    pub dpdmvalue: DPDMVALUE,
    #[doc = "0x50c - Data toggle control and status"]
    pub dtoggle: DTOGGLE,
    #[doc = "0x510 - Endpoint IN enable"]
    pub epinen: EPINEN,
    #[doc = "0x514 - Endpoint OUT enable"]
    pub epouten: EPOUTEN,
    #[doc = "0x518 - STALL endpoints"]
    pub epstall: EPSTALL,
    #[doc = "0x51c - Controls the split of ISO buffers"]
    pub isosplit: ISOSPLIT,
    #[doc = "0x520 - Returns the current value of the start of frame counter"]
    pub framecntr: FRAMECNTR,
    _reserved47: [u8; 8usize],
    #[doc = "0x52c - Controls USBD peripheral low power mode during USB suspend"]
    pub lowpower: LOWPOWER,
    #[doc = "0x530 - Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
    pub isoinconfig: ISOINCONFIG,
    _reserved49: [u8; 204usize],
    #[doc = "0x600 - Unspecified"]
    pub epin0: EPIN,
    _reserved50: [u8; 8usize],
    #[doc = "0x614 - Unspecified"]
    pub epin1: EPIN,
    _reserved51: [u8; 8usize],
    #[doc = "0x628 - Unspecified"]
    pub epin2: EPIN,
    _reserved52: [u8; 8usize],
    #[doc = "0x63c - Unspecified"]
    pub epin3: EPIN,
    _reserved53: [u8; 8usize],
    #[doc = "0x650 - Unspecified"]
    pub epin4: EPIN,
    _reserved54: [u8; 8usize],
    #[doc = "0x664 - Unspecified"]
    pub epin5: EPIN,
    _reserved55: [u8; 8usize],
    #[doc = "0x678 - Unspecified"]
    pub epin6: EPIN,
    _reserved56: [u8; 8usize],
    #[doc = "0x68c - Unspecified"]
    pub epin7: EPIN,
    _reserved57: [u8; 8usize],
    #[doc = "0x6a0 - Unspecified"]
    pub isoin: ISOIN,
    _reserved58: [u8; 84usize],
    #[doc = "0x700 - Unspecified"]
    pub epout0: EPOUT,
    _reserved59: [u8; 8usize],
    #[doc = "0x714 - Unspecified"]
    pub epout1: EPOUT,
    _reserved60: [u8; 8usize],
    #[doc = "0x728 - Unspecified"]
    pub epout2: EPOUT,
    _reserved61: [u8; 8usize],
    #[doc = "0x73c - Unspecified"]
    pub epout3: EPOUT,
    _reserved62: [u8; 8usize],
    #[doc = "0x750 - Unspecified"]
    pub epout4: EPOUT,
    _reserved63: [u8; 8usize],
    #[doc = "0x764 - Unspecified"]
    pub epout5: EPOUT,
    _reserved64: [u8; 8usize],
    #[doc = "0x778 - Unspecified"]
    pub epout6: EPOUT,
    _reserved65: [u8; 8usize],
    #[doc = "0x78c - Unspecified"]
    pub epout7: EPOUT,
    _reserved66: [u8; 8usize],
    #[doc = "0x7a0 - Unspecified"]
    pub isoout: ISOOUT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct HALTED {
    #[doc = "0x00 - Description collection\\[n\\]: IN endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epin: [self::halted::EPIN; 8],
    _reserved1: [u8; 4usize],
    #[doc = "0x24 - Description collection\\[n\\]: OUT endpoint halted status. Can be used as is as response to a GetStatus() request to endpoint."]
    pub epout: [self::halted::EPOUT; 8],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod halted;
#[doc = r"Register block"]
#[repr(C)]
pub struct SIZE {
    #[doc = "0x00 - Description collection\\[n\\]: Number of bytes received last in the data stage of this OUT endpoint"]
    pub epout: [self::size::EPOUT; 8],
    #[doc = "0x20 - Number of bytes received last on this ISO OUT data endpoint"]
    pub isoout: self::size::ISOOUT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod size;
#[doc = r"Register block"]
#[repr(C)]
pub struct EPIN {
    #[doc = "0x00 - Description cluster\\[n\\]: Data pointer"]
    pub ptr: self::epin::PTR,
    #[doc = "0x04 - Description cluster\\[n\\]: Maximum number of bytes to transfer"]
    pub maxcnt: self::epin::MAXCNT,
    #[doc = "0x08 - Description cluster\\[n\\]: Number of bytes transferred in the last transaction"]
    pub amount: self::epin::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod epin;
#[doc = r"Register block"]
#[repr(C)]
pub struct ISOIN {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::isoin::PTR,
    #[doc = "0x04 - Maximum number of bytes to transfer"]
    pub maxcnt: self::isoin::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::isoin::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod isoin;
#[doc = r"Register block"]
#[repr(C)]
pub struct EPOUT {
    #[doc = "0x00 - Description cluster\\[n\\]: Data pointer"]
    pub ptr: self::epout::PTR,
    #[doc = "0x04 - Description cluster\\[n\\]: Maximum number of bytes to transfer"]
    pub maxcnt: self::epout::MAXCNT,
    #[doc = "0x08 - Description cluster\\[n\\]: Number of bytes transferred in the last transaction"]
    pub amount: self::epout::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod epout;
#[doc = r"Register block"]
#[repr(C)]
pub struct ISOOUT {
    #[doc = "0x00 - Data pointer"]
    pub ptr: self::isoout::PTR,
    #[doc = "0x04 - Maximum number of bytes to transfer"]
    pub maxcnt: self::isoout::MAXCNT,
    #[doc = "0x08 - Number of bytes transferred in the last transaction"]
    pub amount: self::isoout::AMOUNT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod isoout;
#[doc = "Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startepin](tasks_startepin) module"]
pub type TASKS_STARTEPIN = crate::pac::generic::Reg<u32, _TASKS_STARTEPIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTEPIN;
#[doc = "`write(|w| ..)` method takes [tasks_startepin::W](tasks_startepin::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_STARTEPIN {}
#[doc = "Description collection\\[n\\]: Captures the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT registers values, and enables endpoint IN n to respond to traffic from host"]
pub mod tasks_startepin;
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startisoin](tasks_startisoin) module"]
pub type TASKS_STARTISOIN = crate::pac::generic::Reg<u32, _TASKS_STARTISOIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTISOIN;
#[doc = "`write(|w| ..)` method takes [tasks_startisoin::W](tasks_startisoin::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_STARTISOIN {}
#[doc = "Captures the ISOIN.PTR and ISOIN.MAXCNT registers values, and enables sending data on ISO endpoint"]
pub mod tasks_startisoin;
#[doc = "Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startepout](tasks_startepout) module"]
pub type TASKS_STARTEPOUT = crate::pac::generic::Reg<u32, _TASKS_STARTEPOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTEPOUT;
#[doc = "`write(|w| ..)` method takes [tasks_startepout::W](tasks_startepout::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_STARTEPOUT {}
#[doc = "Description collection\\[n\\]: Captures the EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers values, and enables endpoint n to respond to traffic from host"]
pub mod tasks_startepout;
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startisoout](tasks_startisoout) module"]
pub type TASKS_STARTISOOUT = crate::pac::generic::Reg<u32, _TASKS_STARTISOOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTISOOUT;
#[doc = "`write(|w| ..)` method takes [tasks_startisoout::W](tasks_startisoout::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_STARTISOOUT {}
#[doc = "Captures the ISOOUT.PTR and ISOOUT.MAXCNT registers values, and enables receiving of data on ISO endpoint"]
pub mod tasks_startisoout;
#[doc = "Allows OUT data stage on control endpoint 0\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ep0rcvout](tasks_ep0rcvout) module"]
pub type TASKS_EP0RCVOUT = crate::pac::generic::Reg<u32, _TASKS_EP0RCVOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_EP0RCVOUT;
#[doc = "`write(|w| ..)` method takes [tasks_ep0rcvout::W](tasks_ep0rcvout::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_EP0RCVOUT {}
#[doc = "Allows OUT data stage on control endpoint 0"]
pub mod tasks_ep0rcvout;
#[doc = "Allows status stage on control endpoint 0\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ep0status](tasks_ep0status) module"]
pub type TASKS_EP0STATUS = crate::pac::generic::Reg<u32, _TASKS_EP0STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_EP0STATUS;
#[doc = "`write(|w| ..)` method takes [tasks_ep0status::W](tasks_ep0status::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_EP0STATUS {}
#[doc = "Allows status stage on control endpoint 0"]
pub mod tasks_ep0status;
#[doc = "Stalls data and status stage on control endpoint 0\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ep0stall](tasks_ep0stall) module"]
pub type TASKS_EP0STALL = crate::pac::generic::Reg<u32, _TASKS_EP0STALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_EP0STALL;
#[doc = "`write(|w| ..)` method takes [tasks_ep0stall::W](tasks_ep0stall::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_EP0STALL {}
#[doc = "Stalls data and status stage on control endpoint 0"]
pub mod tasks_ep0stall;
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_dpdmdrive](tasks_dpdmdrive) module"]
pub type TASKS_DPDMDRIVE = crate::pac::generic::Reg<u32, _TASKS_DPDMDRIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DPDMDRIVE;
#[doc = "`write(|w| ..)` method takes [tasks_dpdmdrive::W](tasks_dpdmdrive::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_DPDMDRIVE {}
#[doc = "Forces D+ and D- lines into the state defined in the DPDMVALUE register"]
pub mod tasks_dpdmdrive;
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_dpdmnodrive](tasks_dpdmnodrive) module"]
pub type TASKS_DPDMNODRIVE = crate::pac::generic::Reg<u32, _TASKS_DPDMNODRIVE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_DPDMNODRIVE;
#[doc = "`write(|w| ..)` method takes [tasks_dpdmnodrive::W](tasks_dpdmnodrive::W) writer structure"]
impl crate::pac::generic::Writable for TASKS_DPDMNODRIVE {}
#[doc = "Stops forcing D+ and D- lines into any state (USB engine takes control)"]
pub mod tasks_dpdmnodrive;
#[doc = "Signals that a USB reset condition has been detected on USB lines\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbreset](events_usbreset) module"]
pub type EVENTS_USBRESET = crate::pac::generic::Reg<u32, _EVENTS_USBRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBRESET;
#[doc = "`read()` method returns [events_usbreset::R](events_usbreset::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_USBRESET {}
#[doc = "`write(|w| ..)` method takes [events_usbreset::W](events_usbreset::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_USBRESET {}
#[doc = "Signals that a USB reset condition has been detected on USB lines"]
pub mod events_usbreset;
#[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_started](events_started) module"]
pub type EVENTS_STARTED = crate::pac::generic::Reg<u32, _EVENTS_STARTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STARTED;
#[doc = "`read()` method returns [events_started::R](events_started::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_STARTED {}
#[doc = "`write(|w| ..)` method takes [events_started::W](events_started::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_STARTED {}
#[doc = "Confirms that the EPIN\\[n\\].PTR and EPIN\\[n\\].MAXCNT, or EPOUT\\[n\\].PTR and EPOUT\\[n\\].MAXCNT registers have been captured on all endpoints reported in the EPSTATUS register"]
pub mod events_started;
#[doc = "Description collection\\[n\\]: The whole EPIN\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endepin](events_endepin) module"]
pub type EVENTS_ENDEPIN = crate::pac::generic::Reg<u32, _EVENTS_ENDEPIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDEPIN;
#[doc = "`read()` method returns [events_endepin::R](events_endepin::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_ENDEPIN {}
#[doc = "`write(|w| ..)` method takes [events_endepin::W](events_endepin::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_ENDEPIN {}
#[doc = "Description collection\\[n\\]: The whole EPIN\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepin;
#[doc = "An acknowledged data transfer has taken place on the control endpoint\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ep0datadone](events_ep0datadone) module"]
pub type EVENTS_EP0DATADONE = crate::pac::generic::Reg<u32, _EVENTS_EP0DATADONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_EP0DATADONE;
#[doc = "`read()` method returns [events_ep0datadone::R](events_ep0datadone::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_EP0DATADONE {}
#[doc = "`write(|w| ..)` method takes [events_ep0datadone::W](events_ep0datadone::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_EP0DATADONE {}
#[doc = "An acknowledged data transfer has taken place on the control endpoint"]
pub mod events_ep0datadone;
#[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endisoin](events_endisoin) module"]
pub type EVENTS_ENDISOIN = crate::pac::generic::Reg<u32, _EVENTS_ENDISOIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDISOIN;
#[doc = "`read()` method returns [events_endisoin::R](events_endisoin::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_ENDISOIN {}
#[doc = "`write(|w| ..)` method takes [events_endisoin::W](events_endisoin::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_ENDISOIN {}
#[doc = "The whole ISOIN buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoin;
#[doc = "Description collection\\[n\\]: The whole EPOUT\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endepout](events_endepout) module"]
pub type EVENTS_ENDEPOUT = crate::pac::generic::Reg<u32, _EVENTS_ENDEPOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDEPOUT;
#[doc = "`read()` method returns [events_endepout::R](events_endepout::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_ENDEPOUT {}
#[doc = "`write(|w| ..)` method takes [events_endepout::W](events_endepout::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_ENDEPOUT {}
#[doc = "Description collection\\[n\\]: The whole EPOUT\\[n\\]
buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endepout;
#[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endisoout](events_endisoout) module"]
pub type EVENTS_ENDISOOUT = crate::pac::generic::Reg<u32, _EVENTS_ENDISOOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDISOOUT;
#[doc = "`read()` method returns [events_endisoout::R](events_endisoout::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_ENDISOOUT {}
#[doc = "`write(|w| ..)` method takes [events_endisoout::W](events_endisoout::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_ENDISOOUT {}
#[doc = "The whole ISOOUT buffer has been consumed. The RAM buffer can be accessed safely by software."]
pub mod events_endisoout;
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sof](events_sof) module"]
pub type EVENTS_SOF = crate::pac::generic::Reg<u32, _EVENTS_SOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SOF;
#[doc = "`read()` method returns [events_sof::R](events_sof::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_SOF {}
#[doc = "`write(|w| ..)` method takes [events_sof::W](events_sof::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_SOF {}
#[doc = "Signals that a SOF (start of frame) condition has been detected on USB lines"]
pub mod events_sof;
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause.\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_usbevent](events_usbevent) module"]
pub type EVENTS_USBEVENT = crate::pac::generic::Reg<u32, _EVENTS_USBEVENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBEVENT;
#[doc = "`read()` method returns [events_usbevent::R](events_usbevent::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_USBEVENT {}
#[doc = "`write(|w| ..)` method takes [events_usbevent::W](events_usbevent::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_USBEVENT {}
#[doc = "An event or an error not covered by specific events has occurred. Check EVENTCAUSE register to find the cause."]
pub mod events_usbevent;
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ep0setup](events_ep0setup) module"]
pub type EVENTS_EP0SETUP = crate::pac::generic::Reg<u32, _EVENTS_EP0SETUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_EP0SETUP;
#[doc = "`read()` method returns [events_ep0setup::R](events_ep0setup::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_EP0SETUP {}
#[doc = "`write(|w| ..)` method takes [events_ep0setup::W](events_ep0setup::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_EP0SETUP {}
#[doc = "A valid SETUP token has been received (and acknowledged) on the control endpoint"]
pub mod events_ep0setup;
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_epdata](events_epdata) module"]
pub type EVENTS_EPDATA = crate::pac::generic::Reg<u32, _EVENTS_EPDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_EPDATA;
#[doc = "`read()` method returns [events_epdata::R](events_epdata::R) reader structure"]
impl crate::pac::generic::Readable for EVENTS_EPDATA {}
#[doc = "`write(|w| ..)` method takes [events_epdata::W](events_epdata::W) writer structure"]
impl crate::pac::generic::Writable for EVENTS_EPDATA {}
#[doc = "A data transfer has occurred on a data endpoint, indicated by the EPDATASTATUS register"]
pub mod events_epdata;
#[doc = "Shortcut register\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::pac::generic::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::pac::generic::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::pac::generic::Writable for SHORTS {}
#[doc = "Shortcut register"]
pub mod shorts;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::pac::generic::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::pac::generic::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::pac::generic::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::pac::generic::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::pac::generic::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::pac::generic::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::pac::generic::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::pac::generic::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::pac::generic::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Details on what caused the USBEVENT event\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eventcause](eventcause) module"]
pub type EVENTCAUSE = crate::pac::generic::Reg<u32, _EVENTCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTCAUSE;
#[doc = "`read()` method returns [eventcause::R](eventcause::R) reader structure"]
impl crate::pac::generic::Readable for EVENTCAUSE {}
#[doc = "`write(|w| ..)` method takes [eventcause::W](eventcause::W) writer structure"]
impl crate::pac::generic::Writable for EVENTCAUSE {}
#[doc = "Details on what caused the USBEVENT event"]
pub mod eventcause;
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstatus](epstatus) module"]
pub type EPSTATUS = crate::pac::generic::Reg<u32, _EPSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTATUS;
#[doc = "`read()` method returns [epstatus::R](epstatus::R) reader structure"]
impl crate::pac::generic::Readable for EPSTATUS {}
#[doc = "`write(|w| ..)` method takes [epstatus::W](epstatus::W) writer structure"]
impl crate::pac::generic::Writable for EPSTATUS {}
#[doc = "Provides information on which endpoint's EasyDMA registers have been captured"]
pub mod epstatus;
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epdatastatus](epdatastatus) module"]
pub type EPDATASTATUS = crate::pac::generic::Reg<u32, _EPDATASTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPDATASTATUS;
#[doc = "`read()` method returns [epdatastatus::R](epdatastatus::R) reader structure"]
impl crate::pac::generic::Readable for EPDATASTATUS {}
#[doc = "`write(|w| ..)` method takes [epdatastatus::W](epdatastatus::W) writer structure"]
impl crate::pac::generic::Writable for EPDATASTATUS {}
#[doc = "Provides information on which endpoint(s) an acknowledged data transfer has occurred (EPDATA event)"]
pub mod epdatastatus;
#[doc = "Device USB address\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbaddr](usbaddr) module"]
pub type USBADDR = crate::pac::generic::Reg<u32, _USBADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBADDR;
#[doc = "`read()` method returns [usbaddr::R](usbaddr::R) reader structure"]
impl crate::pac::generic::Readable for USBADDR {}
#[doc = "Device USB address"]
pub mod usbaddr;
#[doc = "SETUP data, byte 0, bmRequestType\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmrequesttype](bmrequesttype) module"]
pub type BMREQUESTTYPE = crate::pac::generic::Reg<u32, _BMREQUESTTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMREQUESTTYPE;
#[doc = "`read()` method returns [bmrequesttype::R](bmrequesttype::R) reader structure"]
impl crate::pac::generic::Readable for BMREQUESTTYPE {}
#[doc = "SETUP data, byte 0, bmRequestType"]
pub mod bmrequesttype;
#[doc = "SETUP data, byte 1, bRequest\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brequest](brequest) module"]
pub type BREQUEST = crate::pac::generic::Reg<u32, _BREQUEST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BREQUEST;
#[doc = "`read()` method returns [brequest::R](brequest::R) reader structure"]
impl crate::pac::generic::Readable for BREQUEST {}
#[doc = "SETUP data, byte 1, bRequest"]
pub mod brequest;
#[doc = "SETUP data, byte 2, LSB of wValue\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvaluel](wvaluel) module"]
pub type WVALUEL = crate::pac::generic::Reg<u32, _WVALUEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WVALUEL;
#[doc = "`read()` method returns [wvaluel::R](wvaluel::R) reader structure"]
impl crate::pac::generic::Readable for WVALUEL {}
#[doc = "SETUP data, byte 2, LSB of wValue"]
pub mod wvaluel;
#[doc = "SETUP data, byte 3, MSB of wValue\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvalueh](wvalueh) module"]
pub type WVALUEH = crate::pac::generic::Reg<u32, _WVALUEH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WVALUEH;
#[doc = "`read()` method returns [wvalueh::R](wvalueh::R) reader structure"]
impl crate::pac::generic::Readable for WVALUEH {}
#[doc = "SETUP data, byte 3, MSB of wValue"]
pub mod wvalueh;
#[doc = "SETUP data, byte 4, LSB of wIndex\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [windexl](windexl) module"]
pub type WINDEXL = crate::pac::generic::Reg<u32, _WINDEXL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINDEXL;
#[doc = "`read()` method returns [windexl::R](windexl::R) reader structure"]
impl crate::pac::generic::Readable for WINDEXL {}
#[doc = "SETUP data, byte 4, LSB of wIndex"]
pub mod windexl;
#[doc = "SETUP data, byte 5, MSB of wIndex\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [windexh](windexh) module"]
pub type WINDEXH = crate::pac::generic::Reg<u32, _WINDEXH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WINDEXH;
#[doc = "`read()` method returns [windexh::R](windexh::R) reader structure"]
impl crate::pac::generic::Readable for WINDEXH {}
#[doc = "SETUP data, byte 5, MSB of wIndex"]
pub mod windexh;
#[doc = "SETUP data, byte 6, LSB of wLength\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlengthl](wlengthl) module"]
pub type WLENGTHL = crate::pac::generic::Reg<u32, _WLENGTHL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLENGTHL;
#[doc = "`read()` method returns [wlengthl::R](wlengthl::R) reader structure"]
impl crate::pac::generic::Readable for WLENGTHL {}
#[doc = "SETUP data, byte 6, LSB of wLength"]
pub mod wlengthl;
#[doc = "SETUP data, byte 7, MSB of wLength\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wlengthh](wlengthh) module"]
pub type WLENGTHH = crate::pac::generic::Reg<u32, _WLENGTHH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WLENGTHH;
#[doc = "`read()` method returns [wlengthh::R](wlengthh::R) reader structure"]
impl crate::pac::generic::Readable for WLENGTHH {}
#[doc = "SETUP data, byte 7, MSB of wLength"]
pub mod wlengthh;
#[doc = "Enable USB\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::pac::generic::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::pac::generic::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::pac::generic::Writable for ENABLE {}
#[doc = "Enable USB"]
pub mod enable;
#[doc = "Control of the USB pull-up\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbpullup](usbpullup) module"]
pub type USBPULLUP = crate::pac::generic::Reg<u32, _USBPULLUP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPULLUP;
#[doc = "`read()` method returns [usbpullup::R](usbpullup::R) reader structure"]
impl crate::pac::generic::Readable for USBPULLUP {}
#[doc = "`write(|w| ..)` method takes [usbpullup::W](usbpullup::W) writer structure"]
impl crate::pac::generic::Writable for USBPULLUP {}
#[doc = "Control of the USB pull-up"]
pub mod usbpullup;
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing).\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpdmvalue](dpdmvalue) module"]
pub type DPDMVALUE = crate::pac::generic::Reg<u32, _DPDMVALUE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DPDMVALUE;
#[doc = "`read()` method returns [dpdmvalue::R](dpdmvalue::R) reader structure"]
impl crate::pac::generic::Readable for DPDMVALUE {}
#[doc = "`write(|w| ..)` method takes [dpdmvalue::W](dpdmvalue::W) writer structure"]
impl crate::pac::generic::Writable for DPDMVALUE {}
#[doc = "State D+ and D- lines will be forced into by the DPDMDRIVE task. The DPDMNODRIVE task reverts the control of the lines to MAC IP (no forcing)."]
pub mod dpdmvalue;
#[doc = "Data toggle control and status\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtoggle](dtoggle) module"]
pub type DTOGGLE = crate::pac::generic::Reg<u32, _DTOGGLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTOGGLE;
#[doc = "`read()` method returns [dtoggle::R](dtoggle::R) reader structure"]
impl crate::pac::generic::Readable for DTOGGLE {}
#[doc = "`write(|w| ..)` method takes [dtoggle::W](dtoggle::W) writer structure"]
impl crate::pac::generic::Writable for DTOGGLE {}
#[doc = "Data toggle control and status"]
pub mod dtoggle;
#[doc = "Endpoint IN enable\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epinen](epinen) module"]
pub type EPINEN = crate::pac::generic::Reg<u32, _EPINEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPINEN;
#[doc = "`read()` method returns [epinen::R](epinen::R) reader structure"]
impl crate::pac::generic::Readable for EPINEN {}
#[doc = "`write(|w| ..)` method takes [epinen::W](epinen::W) writer structure"]
impl crate::pac::generic::Writable for EPINEN {}
#[doc = "Endpoint IN enable"]
pub mod epinen;
#[doc = "Endpoint OUT enable\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epouten](epouten) module"]
pub type EPOUTEN = crate::pac::generic::Reg<u32, _EPOUTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPOUTEN;
#[doc = "`read()` method returns [epouten::R](epouten::R) reader structure"]
impl crate::pac::generic::Readable for EPOUTEN {}
#[doc = "`write(|w| ..)` method takes [epouten::W](epouten::W) writer structure"]
impl crate::pac::generic::Writable for EPOUTEN {}
#[doc = "Endpoint OUT enable"]
pub mod epouten;
#[doc = "STALL endpoints\n\nThis register you can [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [epstall](epstall) module"]
pub type EPSTALL = crate::pac::generic::Reg<u32, _EPSTALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EPSTALL;
#[doc = "`write(|w| ..)` method takes [epstall::W](epstall::W) writer structure"]
impl crate::pac::generic::Writable for EPSTALL {}
#[doc = "STALL endpoints"]
pub mod epstall;
#[doc = "Controls the split of ISO buffers\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isosplit](isosplit) module"]
pub type ISOSPLIT = crate::pac::generic::Reg<u32, _ISOSPLIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOSPLIT;
#[doc = "`read()` method returns [isosplit::R](isosplit::R) reader structure"]
impl crate::pac::generic::Readable for ISOSPLIT {}
#[doc = "`write(|w| ..)` method takes [isosplit::W](isosplit::W) writer structure"]
impl crate::pac::generic::Writable for ISOSPLIT {}
#[doc = "Controls the split of ISO buffers"]
pub mod isosplit;
#[doc = "Returns the current value of the start of frame counter\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [framecntr](framecntr) module"]
pub type FRAMECNTR = crate::pac::generic::Reg<u32, _FRAMECNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMECNTR;
#[doc = "`read()` method returns [framecntr::R](framecntr::R) reader structure"]
impl crate::pac::generic::Readable for FRAMECNTR {}
#[doc = "Returns the current value of the start of frame counter"]
pub mod framecntr;
#[doc = "Controls USBD peripheral low power mode during USB suspend\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpower](lowpower) module"]
pub type LOWPOWER = crate::pac::generic::Reg<u32, _LOWPOWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOWPOWER;
#[doc = "`read()` method returns [lowpower::R](lowpower::R) reader structure"]
impl crate::pac::generic::Readable for LOWPOWER {}
#[doc = "`write(|w| ..)` method takes [lowpower::W](lowpower::W) writer structure"]
impl crate::pac::generic::Writable for LOWPOWER {}
#[doc = "Controls USBD peripheral low power mode during USB suspend"]
pub mod lowpower;
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent\n\nThis register you can [`read`](crate::pac::generic::generic::Reg::read), [`reset`](crate::pac::generic::generic::Reg::reset), [`write`](crate::pac::generic::generic::Reg::write), [`write_with_zero`](crate::pac::generic::generic::Reg::write_with_zero), [`modify`](crate::pac::generic::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isoinconfig](isoinconfig) module"]
pub type ISOINCONFIG = crate::pac::generic::Reg<u32, _ISOINCONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISOINCONFIG;
#[doc = "`read()` method returns [isoinconfig::R](isoinconfig::R) reader structure"]
impl crate::pac::generic::Readable for ISOINCONFIG {}
#[doc = "`write(|w| ..)` method takes [isoinconfig::W](isoinconfig::W) writer structure"]
impl crate::pac::generic::Writable for ISOINCONFIG {}
#[doc = "Controls the response of the ISO IN endpoint to an IN token when no data is ready to be sent"]
pub mod isoinconfig;
