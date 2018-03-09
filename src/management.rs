use uio::*;
use volatile_register::RW;

#[repr(C)]
pub struct MgmtRegs {
    /// GPIO mode (0 - PS GPIO, 1 - Logic generator).
    cfg_iom: RW<u32>,
    /// Enable internal digital loop from gen to osc.
    cfg_loop: RW<u32>,
}

pub struct Management {
    regs: MemoryMap,
}

impl Management {
    pub fn new() -> Self {
        let uio_num = get_uio_num("/dev/uio/mgmt");
        let uio = UioDevice::new(uio_num).unwrap();
        let regs = uio.map_mapping(0).unwrap();
        Management { regs }
    }

    /// Get GPIO mode.
    ///
    /// Each bit coresponds to one of {exp_n_io[7:0], exp_p_io[7:0]} GPIO pins.
    /// 0 - pin is connected to PS GPIO controller
    /// 1 - pin is connected to Logic generator.
    pub fn gpio_mode(&self) -> u32 {
        self.regs().cfg_iom.read()
    }

    /// Set GPIO mode
    pub fn set_gpio_mode(&mut self, mode: u32) {
        unsafe { self.mut_regs().cfg_iom.write(mode); }
    }

    /// Get digital loopback register (for debugging purposes).
    ///
    /// Each bit controls one of the loop paths:
    /// 0 - enable loop: gen0 -> osc0,
    /// 1 - enable loop: gen1 -> osc1.
    pub fn gen_osc_loop(&self) -> u32 {
        self.regs().cfg_loop.read()
    }

    /// Set digital loopback register (for debugging purposes).
    pub fn set_gen_osc_loop(&mut self, value: u32) {
        unsafe { self.mut_regs().cfg_loop.write(value); }
    }
}

impl RegDevice for Management {
    type Registers = MgmtRegs;

    fn reg_mmap(&self) -> &MemoryMap {
        &self.regs
    }
}

impl Default for Management {
    fn default(&mut self) {
        unsafe {
            self.mut_regs().cfg_iom.write(0x0);
            self.mut_regs().cfg_loop.write(0x0);
        }
    }
}

impl Show for Management {
    fn show(&self) {
        println!("cfg_iom = {:x}", self.regs().cfg_iom.read());
        println!("cfg_loop = {:x}", self.regs().cfg_loop.read());
    }
}
