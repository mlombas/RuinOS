pub mod algorithm;
pub mod math;

pub fn halt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
