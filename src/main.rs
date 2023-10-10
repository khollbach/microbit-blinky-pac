#![allow(unused_imports)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    display::{self, blocking},
    hal::{prelude::*, Timer},
    pac::{P0, P1, TIMER0},
    Board,
};
use nrf52833_pac::Peripherals;
use panic_rtt_target as _;
use rtt_target::{rdbg, rprintln, rtt_init_print};
use void::ResultVoidExt;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // let board = Board::take().unwrap();

    // let mut row1 = board.display_pins.row1;
    // let mut col1 = board.display_pins.col1;

    // let mut timer = Timer::periodic(board.TIMER0);
    // timer.start(1_000_000u32);

    let peripherals = Peripherals::take().unwrap();
    let timer 

    unsafe { &TIMER0::PTR

    // // rdbg!(P0::PTR);
    // let word = unsafe { &(*P0::PTR).out }.read().bits();
    // rprintln!("{:08x}", word);
    // rprintln!("{:032b}", word);
    // let word = unsafe { &(*P0::PTR).in_ }.read().bits();
    // rprintln!("{:08x}", word);
    // rprintln!("{:032b}", word);

    // for i in 0..16 {
    //     let word = unsafe { &(*P0::PTR).pin_cnf[i] }.read().bits();
    //     rprintln!("{:2} {:08x} {:032b}", i, word, word);
    // }
    // rprintln!("{:#?}", unsafe { &*P0::ptr() });

    // todo: find the docs to set GPIO P0_28 to low on nRF5
    // col1.set_low().void_unwrap();

    unsafe { &(*P0::PTR).outclr }.write(|w| w.pin28().set_bit());

    // panic!("done");

    loop {
        // row1.set_high().void_unwrap();
        unsafe { &(*P0::PTR).outset }.write(|w| w.pin21().set_bit());
        rprintln!("Light!");

        while unsafe { &(*TIMER0::PTR).events_compare[0] }
            .read()
            .events_compare()
            .is_not_generated()
        {}
        unsafe { &(*TIMER0::PTR).events_compare[0] }.write(|w| w.events_compare().not_generated());

        // nb::block!(timer.wait()).void_unwrap();

        // row1.set_low().void_unwrap();
        unsafe { &(*P0::PTR).outclr }.write(|w| w.pin21().set_bit());
        rprintln!("Dark!");

        // nb::block!(timer.wait()).void_unwrap();
        while unsafe { &(*TIMER0::PTR).events_compare[0] }
            .read()
            .events_compare()
            .is_not_generated()
        {}
        unsafe { &(*TIMER0::PTR).events_compare[0] }.write(|w| w.events_compare().not_generated());
    }
}
