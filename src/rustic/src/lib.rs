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
#![feature(asm)]
#![feature(llvm_asm)]
#![feature(lang_items)]
#![feature(rustc_private)]
#![feature(restricted_std)]
#![feature(negative_impls)]
#![allow(dead_code)]

#![no_main]

// Publish the main things users care about.
pub use mach::{Machine, TimerHandlers, Mmio, Gpio, IoPort, IrqHandler, Serial};
pub use arch::{Architecture, Threads, ThreadSpawn};

// Pull in the architectural layer (CPU etc).
pub mod arch;

// Pull in the machine layer.
pub mod mach;

// Pull in utils library.
pub mod util;

use std::sync::Arc;
use util::sync::Spinlock;

pub struct Kernel {
    mach: mach::MachineState,
    arch: arch::ArchitectureState,
}

// Required to be defined by the application.
extern "Rust" { fn run(k: &mut Kernel); }

pub trait Idle {
    fn idle();
}

impl Kernel {
    pub fn new() -> Kernel {
        Kernel {
            mach: mach::create(),
            arch: arch::create()
        }
    }

    // Sets up the kernel, and then returns a wrapped version of the Kernel
    // that is correctly prepared for concurrency.
    pub fn start(mut self) -> Arc<Spinlock<Kernel>> {
        // Now we can initialise the system.
        self.arch_initialise();
        self.mach_initialise();

        // All done with initial startup.
        self.serial_write("Built on the Rustic Framework.\n");

        // Enable IRQs and start up the application.
        self.set_interrupts(true);

        Arc::new(Spinlock::new(self))
    }

    pub fn spawn<F>(&mut self, f: F)
    where
        F: FnMut(),
        F: Send,
        F: 'static
    {
        self.spawn_thread(f);
    }
}