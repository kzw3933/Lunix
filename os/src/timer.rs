use riscv::register::time;
use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;

const TICKS_PER_SEC: usize = 100000;
const MICRO_PER_SEC: usize = 1_000;

pub fn set_next_trigger() {
    set_timer(get_time()+CLOCK_FREQ/TICKS_PER_SEC);
}

pub fn get_time_ms() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

pub fn get_time() -> usize {
    time::read()
}