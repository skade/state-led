//! A rainbow-LED example application
//! This example uses launchpad-rs.

#![no_std]
#![no_main]
#![feature(alloc, collections, asm)]
#![crate_type = "staticlib"]

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

extern crate alloc;
extern crate embedded_hal;
extern crate stellaris_launchpad;

use core::fmt::Write;
use embedded_hal::serial::Read;
use stellaris_launchpad::cpu::{gpio, timer, uart};

mod lights {
    pub trait LightState {
        fn rgb(&self) -> (u32, u32, u32);
    }

    pub struct Red;
    pub struct Green;
    pub struct Blue;

    impl LightState for Red {
        fn rgb(&self) -> (u32, u32, u32) {
            (255, 0, 0)
        }
    }

    impl LightState for Green {
        fn rgb(&self) -> (u32, u32, u32) {
            (0, 255, 0)
        }
    }

    impl LightState for Blue {
        fn rgb(&self) -> (u32, u32, u32) {
            (0, 0, 255)
        }
    }

    pub struct Blinklight<T: LightState> {
        state: T
    }

    impl<T: LightState> Blinklight<T> {
        pub fn rgb(&self) -> (u32, u32, u32) {
            self.state.rgb()
        }
    }

    impl From<Blinklight<Red>> for Blinklight<Green> {
        fn from(_: Blinklight<Red>) -> Blinklight<Green> {
            Blinklight { state: Green }
        }
    }

    impl From<Blinklight<Green>> for Blinklight<Blue> {
        fn from(_: Blinklight<Green>) -> Blinklight<Blue> {
            Blinklight { state: Blue }
        }
    }

    impl From<Blinklight<Blue>> for Blinklight<Red> {
        fn from(_: Blinklight<Blue>) -> Blinklight<Red> {
            Blinklight { state: Red }
        }
    }

    pub fn start() -> Blinklight<Red> {
        Blinklight { state: Red }
    }
}

#[no_mangle]
pub extern "C" fn main() {
    use lights::*;

    let mut uart = uart::Uart::new(uart::UartId::Uart0, 115200, uart::NewlineMode::SwapLFtoCRLF);

    let mut tr = timer::Timer::new(timer::TimerId::Timer0B);
    let mut tb = timer::Timer::new(timer::TimerId::Timer1A);
    let mut tg = timer::Timer::new(timer::TimerId::Timer1B);
    tr.enable_pwm(255);
    tb.enable_pwm(255);
    // Green is a bit bright! Tone it down.
    tg.enable_pwm(512);
    gpio::PinPort::PortF(gpio::Pin::Pin1).set_direction(gpio::PinMode::Peripheral);
    gpio::PinPort::PortF(gpio::Pin::Pin2).set_direction(gpio::PinMode::Peripheral);
    gpio::PinPort::PortF(gpio::Pin::Pin3).set_direction(gpio::PinMode::Peripheral);
    gpio::PinPort::PortF(gpio::Pin::Pin1).enable_ccp();
    gpio::PinPort::PortF(gpio::Pin::Pin2).enable_ccp();
    gpio::PinPort::PortF(gpio::Pin::Pin3).enable_ccp();

    loop {
        let light = lights::start();

        let (red, green, blue) = light.rgb();
        
        tr.set_pwm(red);
        tb.set_pwm(green);
        tg.set_pwm(blue);

        stellaris_launchpad::delay(500);

        let green_light: Blinklight<Green> = light.into();

        let (red, green, blue) = green_light.rgb();

        tr.set_pwm(red);
        tb.set_pwm(green);
        tg.set_pwm(blue);

        stellaris_launchpad::delay(500);

        let blue_light: Blinklight<Blue> = green_light.into();

        let (red, green, blue) = blue_light.rgb();

        tr.set_pwm(red);
        tb.set_pwm(green);
        tg.set_pwm(blue);
        
        while let Ok(ch) = uart.read() {
            writeln!(uart, "byte read {}", ch).unwrap();
        }
        
        stellaris_launchpad::delay(500);
    }
}

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
