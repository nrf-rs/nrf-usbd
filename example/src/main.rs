#![no_std]
#![no_main]

use defmt_rtt as _;
use nrf_usbd::{UsbPeripheral, Usbd};
// global logger
use panic_probe as _;
use usb_device::class_prelude::UsbBusAllocator;

use core::str;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use nrf52840_pac as pac;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

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

    let usb_bus = UsbBusAllocator::new(Usbd::new(Peripheral));
    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .product("nRF52840 Serial Port Demo")
        .device_class(USB_CLASS_CDC)
        .max_packet_size_0(64) // (makes control transfers 8x faster)
        .build();

    info!("started!");

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                let mut buf = &mut buf[..count];

                info!("read: {=str}", unsafe { str::from_utf8_unchecked(buf) });

                // Echo back in upper case
                for c in buf.iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                info!("writing: {=str}", unsafe { str::from_utf8_unchecked(buf) });
                while !buf.is_empty() {
                    match serial.write(buf) {
                        Ok(len) => buf = &mut buf[len..],
                        _ => {}
                    }
                }

                info!("write done")
            }
            _ => {}
        }
    }
}

struct Peripheral;
unsafe impl UsbPeripheral for Peripheral {
    const REGISTERS: *const () = pac::USBD::ptr() as *const ();
}
