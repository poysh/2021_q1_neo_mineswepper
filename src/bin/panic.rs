#![no_main]
#![no_std]

use 2021_q1_neo_mineswepper as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main");

    defmt::panic!()
}
