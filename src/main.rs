//! `RustyBoy` a Gameboy emulator and related tools in rust.
//!
//! # Introduction
//!
//! An interpreter and various debugging tools for the Gameboy
//! This project includes an assembler, disassembler, memory visualization,
//! text-based interactive debugger and language, and standard execution.
//!
//! Memory visualization inspired by [ICU64 / Frodo Redpill v0.1](https://icu64.blogspot.com/2009/09/first-public-release-of-icu64frodo.html)
// #[cfg_attr(feature = "cargo-clippy", )]

extern crate clap;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate sdl2;
extern crate ncurses;

/// Simple Gameboy-flavored Z80 assembler
pub mod assembler;

/// The bulk of the hardware emulation
pub mod cpu;

/// Ncurses-based text debugger and parser for debugging language
pub mod debugger;

/// Naive disassembler
pub mod disasm;

/// Functionality for making the Gameboy emulator useful
pub mod io;


use io::applicationstate::*;

#[allow(unused_variables)]
fn main() {
    let arguments = io::arguments::read_arguments();

    // Attempt to read ROM first
    let rom_file = arguments.value_of("game").expect("Could not open specified rom");
    let debug_mode = arguments.is_present("debug");
    let trace_mode = arguments.is_present("trace");

    // Set up gameboy and app state
    let mut appstate = ApplicationState::new(trace_mode, debug_mode, rom_file);
/*
    let mut scale = SCALE;

    // Set up time

    // let timer = sdl_context.timer().unwrap();
    let mut prev_time = 0;

    let mut cycle_count = 0;
    let mut clock_cycles = 0;
    let mut prev_hsync_cycles: u64 = 0;

    // Number of frames saved as screenshots
    let mut frame_num = Wrapping(0);

    */

    loop {
        appstate.handle_events();
        appstate.step();
    }
}

