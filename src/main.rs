/*
 * Copyright (c) 2013 Matthew Iselin
 *
 * Permission to use, copy, modify, and distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */
#[no_std];

#[path = "rust-core/core/mod.rs"]
mod core;

// Pull in VGA utils - clear screen, write text, etc...
mod vga;

// Grab I/O for test.
mod io;

// Grab serial port I/O stuff.
mod serial;

// Pull in CPU things.
pub mod cpu;

// Pull in the machine layer.
mod mach;

#[no_mangle]
pub extern "C" fn abort() {
    serial::write("ABORT");
    cpu::setirqs(false);
    loop {}
}

#[start]
pub fn kmain(_: int, _: **u8) -> int {
    // Clear to black.
    vga::clear(vga::Black);

    // Start up the serial port...
    serial::config(115200, 8, serial::NoParity, 1);

    // Dump some startup junk to the serial port.
    serial::write("Rustic starting up...\n");

    // Get the CPU into a sane, known state.
    cpu::init();

    // Bring up the machine layer.
    mach::init();

    // Welcome message.
    vga::write("Welcome to Rustic!", 0, 0, vga::LightGray, vga::Black);

    // All done with initial startup.
    serial::write("Rustic startup complete.\n");

    // Loop forever, IRQ handling will do the rest!
    cpu::setirqs(true);
    loop {
        cpu::waitforinterrupt();
    }
}
