use uio::*;
use volatile_register::RW;

#[repr(C)]
pub struct LaTriggerRegs {
    /// Comparator mask
    cfg_cmp_mask: RW<u32>,
    /// Comparator value
    cfg_cmp_value: RW<u32>,
    /// Positive edge
    cfg_edge_pos: RW<u32>,
    /// Negative edge
    cfg_edge_neg: RW<u32>,
}

impl Default for LaTriggerRegs {
    fn default(&mut self) {
        unsafe {
            self.cfg_cmp_mask.write(0);
            self.cfg_cmp_value.write(0);
            self.cfg_edge_pos.write(0);
            self.cfg_edge_neg.write(0);
        }
    }
}

impl Show for LaTriggerRegs {
    fn show(&self) {
        println!("cfg_cmp_msk = {:x}", self.cfg_cmp_mask.read());
        println!("cfg_cmp_val = {:x}", self.cfg_cmp_value.read());
        println!("cfg_edg_pos = {:x}", self.cfg_edge_pos.read());
        println!("cfg_edg_neg = {:x}", self.cfg_edge_neg.read());
    }
}

pub trait LaTriggerRegsAPI {
    /// Get trigger comparator mask.
    fn trigger_mask(&self) -> u32;
    /// Set trigger comparator mask.
    fn set_trigger_mask(&mut self, mask: u32);
    /// Get trigger comparator value.
    fn trigger_value(&self) -> u32;
    /// Set trigger comparator value.
    fn set_trigger_value(&mut self, value: u32);
    /// Get positive trigger edge detection mask.
    fn pos_edge_trigger_mask(&self) -> u32;
    /// Set positive trigger edge detection mask.
    fn set_pos_edge_trigger_mask(&mut self, mask: u32);
    /// Get negative trigger edge detection mask.
    fn neg_edge_trigger_mask(&self) -> u32;
    /// Set negative trigger edge detection mask.
    fn set_neg_edge_trigger_mask(&mut self, mask: u32);
}

impl LaTriggerRegsAPI for LaTriggerRegs {
    fn trigger_mask(&self) -> u32 {
        self.cfg_cmp_mask.read()
    }
    fn set_trigger_mask(&mut self, mask: u32) {
        unsafe { self.cfg_cmp_mask.write(mask); }
    }
    fn trigger_value(&self) -> u32 {
        self.cfg_cmp_value.read()
    }
    fn set_trigger_value(&mut self, value: u32) {
        unsafe { self.cfg_cmp_value.write(value); }
    }
    fn pos_edge_trigger_mask(&self) -> u32 {
        self.cfg_edge_pos.read()
    }
    fn set_pos_edge_trigger_mask(&mut self, mask: u32) {
        unsafe { self.cfg_edge_pos.write(mask); }
    }
    fn neg_edge_trigger_mask(&self) -> u32 {
        self.cfg_edge_neg.read()
    }
    fn set_neg_edge_trigger_mask(&mut self, mask: u32) {
        unsafe { self.cfg_edge_neg.write(mask); }
    }
}
