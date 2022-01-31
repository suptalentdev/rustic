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
#[allow(ctypes)];
#[no_std];

mod zero;

// Pull in VGA utils - clear screen, write text, etc...
mod vga;

// Grab I/O for test.
mod io;

// Grab serial port I/O stuff.
mod serial;

// Pull in CPU things.
mod cpu;

#[start]
#[fixed_stack_segment]
pub fn kmain(_: int, _: **u8) -> int {
    // Clear to black.
    vga::clear(vga::Black);

    // Start up the serial port...
    serial::config(115200, 8, serial::NoParity, 1);

    // Dump some startup junk to the serial port.
    serial::write("Rustic starting up...\n");

    // Get the CPU into a sane, known state.
    cpu::init();

    // Welcome message.
    vga::write("Welcome to Rustic!", 0, 0, vga::LightGray, vga::Black);

    // All done with initial startup.
    serial::write("Rustic startup complete.\n");

    0
}
