use uio::*;
use volatile_register::RW;

#[repr(C)]
pub struct LaMaskRegs {
    /// Input mask
    cfg_mask: RW<u32>,
    /// Input polarity
    cfg_pol: RW<u32>,
    /// Decimation factor
    cfg_dec: RW<u32>,
}

impl Default for LaMaskRegs {
    fn default(&mut self) {
        unsafe {
            self.cfg_mask.write(0);
            self.cfg_pol.write(0);
            self.cfg_dec.write(0);
        }
    }
}

impl Show for LaMaskRegs {
    fn show(&self) {
        println!("cfg_mask = {:x}", self.cfg_mask.read());
        println!("cfg_pol = {:x}", self.cfg_pol.read());
        println!("cfg_dec = {:x}", self.cfg_dec.read());
    }
}

pub trait LaMaskRegsAPI {
    /// Get input signal bit mask.
    fn input_mask(&self) -> u32;
    /// Set input signal bit mask (can be used to reduce power consumption).
    fn set_input_mask(&mut self, mask: u32);
    /// Get input signal bit polarity.
    fn input_polarity(&self) -> u32;
    /// Set input signal bit polarity.
    fn set_input_polarity(&mut self, pol: u32);
    /// Get decimation factor.
    fn decimation(&self) -> u32;
    /// Set decimation factor.
    fn set_decimation(&mut self, dec: u32);
}

impl LaMaskRegsAPI for LaMaskRegs {
    fn input_mask(&self) -> u32 {
        self.cfg_mask.read()
    }
    fn set_input_mask(&mut self, mask: u32) {
        unsafe { self.cfg_mask.write(mask); }
    }
    fn input_polarity(&self) -> u32 {
        self.cfg_pol.read()
    }
    fn set_input_polarity(&mut self, pol: u32) {
        unsafe { self.cfg_pol.write(pol); }
    }
    fn decimation(&self) -> u32 {
        self.cfg_dec.read() + 1
    }
    fn set_decimation(&mut self, dec: u32) {
        unsafe { self.cfg_dec.write(dec - 1); }
    }
}
