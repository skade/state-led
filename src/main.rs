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
use stellaris_launchpad::cpu::{gpio, systick, timer, uart};

// ****************************************************************************
//
// Public Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

enum Color {
    Red,
    Green,
    Blue,
}

trait LightState {
    fn rgb(&self) -> (u32, u32, u32);
}

struct Red;
struct Green;
struct Blue;

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

struct Blinklight<T: LightState> {
    state: T
}

impl From<Blinklight<Red>> for Blinklight<Green> {
    fn from(state: Blinklight<Red>) -> Blinklight<Green> {
        Blinklight { state: Green }
    }
}

impl From<Blinklight<Green>> for Blinklight<Blue> {
    fn from(state: Blinklight<Green>) -> Blinklight<Blue> {
        Blinklight { state: Blue }
    }
}

impl From<Blinklight<Blue>> for Blinklight<Red> {
    fn from(state: Blinklight<Blue>) -> Blinklight<Red> {
        Blinklight { state: Red }
    }
}

#[no_mangle]
pub extern "C" fn main() {
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
        let mut light = Blinklight { state: Red };

        let (red, green, blue) = light.state.rgb();
        
        tr.set_pwm(red);
        tb.set_pwm(green);
        tg.set_pwm(blue);

        stellaris_launchpad::delay(500);

        let green_light: Blinklight<Green> = light.into();

        let (red, green, blue) = green_light.state.rgb();

        tr.set_pwm(red);
        tb.set_pwm(green);
        tg.set_pwm(blue);

        stellaris_launchpad::delay(500);

        let blue_light: Blinklight<Blue> = green_light.into();

        let (red, green, blue) = blue_light.state.rgb();

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
