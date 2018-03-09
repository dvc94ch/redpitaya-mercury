use uio::*;
use volatile_register::{RW, RO};

#[repr(C)]
pub struct AcquireRegs {
    /// Delay pre trigger
    cfg_pre: RW<u32>,
    /// Delay post trigger
    cfg_post: RW<u32>,
    /// Status pre trigger
    status_pre: RO<u32>,
    /// Status post trigger
    status_post: RO<u32>,
}

impl Default for AcquireRegs {
    fn default(&mut self) {
        unsafe {
            self.cfg_pre.write(0);
            self.cfg_post.write(0);
        }
    }
}

impl Show for AcquireRegs {
    fn show(&self) {
        println!("cfg_pre = {:x}", self.cfg_pre.read());
        println!("cfg_post = {:x}", self.cfg_post.read());
        println!("status_pre = {:x}", self.status_pre.read());
        println!("status_post = {:x}", self.status_post.read());
    }
}

pub trait AcquireRegsAPI {
    /// Get pre trigger delay.
    ///
    /// Number of samples stored into the buffer after start()
    /// before a trigger event is accepted.
    fn trigger_pre(&self) -> u32;
    /// Set pre trigger delay.
    ///
    /// Number of samples stored into the buffer after a trigger,
    /// before writing stops automatically.
    fn set_trigger_pre(&mut self, delay: u32);
    /// Get post trigger delay.
    fn trigger_post(&self) -> u32;
    /// Set post trigger delay.
    fn set_trigger_post(&mut self, delay: u32);
    /// Pre trigger sample counter status.
    fn trigger_pre_status(&self) -> u32;
    /// Post trigger sample counter status.
    fn trigger_post_status(&self) -> u32;
}

impl AcquireRegsAPI for AcquireRegs {
    fn trigger_pre(&self) -> u32 {
        self.cfg_pre.read()
    }
    fn set_trigger_pre(&mut self, delay: u32) {
        unsafe { self.cfg_pre.write(delay); }
    }
    fn trigger_post(&self) -> u32 {
        self.cfg_post.read()
    }
    fn set_trigger_post(&mut self, delay: u32) {
        unsafe { self.cfg_post.write(delay); }
    }
    fn trigger_pre_status(&self) -> u32 {
        self.status_pre.read()
    }
    fn trigger_post_status(&self) -> u32 {
        self.status_post.read()
    }
}
