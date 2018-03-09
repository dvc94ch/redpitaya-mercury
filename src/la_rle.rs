use uio::*;
use volatile_register::{RO, RW};

#[repr(C)]
pub struct LaRleRegs {
    /// RLE mode
    cfg_rle: RW<u32>,
    /// Current counter
    status_current: RO<u32>,
    /// Last counter
    status_last: RO<u32>,
}

impl Default for LaRleRegs {
    fn default(&mut self) {
        unsafe { self.cfg_rle.write(0); }
    }
}

impl Show for LaRleRegs {
    fn show(&self) {
        println!("cfg_rle = {:x}", self.cfg_rle.read());
        println!("status_current = {:x}", self.status_current.read());
        println!("status_last = {:x}", self.status_last.read());
    }
}

pub trait LaRleRegsAPI {
    /// Get RLE mode.
    fn rle(&self) -> bool;
    /// Set RLE mode.
    fn set_rle(&mut self, rle: bool);
    /// Current data stream length counter.
    fn counter_current(&self) -> u32;
    /// Last data stream length counter.
    fn counter_last(&self) -> u32;
}

impl LaRleRegsAPI for LaRleRegs {
    fn rle(&self) -> bool {
        self.cfg_rle.read() > 0
    }
    fn set_rle(&mut self, rle: bool) {
        unsafe { self.cfg_rle.write(rle as u32); }
    }
    fn counter_current(&self) -> u32 {
        self.status_current.read()
    }
    fn counter_last(&self) -> u32 {
        self.status_last.read()
    }
}
