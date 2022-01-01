#![no_std]
#![no_main]

use defmt_rtt as _;
use nrf_usbd::{UsbPeripheral, Usbd};
// global logger
use panic_probe as _;
use usb_device::test_class::TestClass;

use core::str;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use nrf52840_pac as pac;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
//use usbd_serial::{SerialPort, USB_CLASS_CDC};

defmt::timestamp! {"{=u64}", {
        static COUNT: AtomicUsize = AtomicUsize::new(0);
        // NOTE(no-CAS) `timestamps` runs with interrupts disabled
        let n = COUNT.load(Ordering::Relaxed);
        COUNT.store(n + 1, Ordering::Relaxed);
        n as u64
    }
}

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    info!("Enabling ext hfosc...");
    p.CLOCK.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
    while p.CLOCK.events_hfclkstarted.read().bits() != 1 {}

    info!("Waiting for vbus...");
    while !p.POWER.usbregstatus.read().vbusdetect().is_vbus_present() {}

    //info!("Waiting for usbpwr...");
    //// wait until USB 3.3V supply is stable
    //while !p.POWER.events_usbpwrrdy.read().events_usbpwrrdy().bit() {}

    info!("starting...");

    let mut usb_bus = Usbd::new(Peripheral);

    let mut test = TestClass::new(&mut usb_bus);
    let mut usb_dev = test.make_device(usb_bus);

    info!("started!");

    loop {
        if !usb_dev.poll(&mut [&mut test]) {
            continue;
        }

        test.poll(usb_dev.bus_mut());
    }
}

struct Peripheral;
unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = pac::USBD::ptr() as *const ();
}
