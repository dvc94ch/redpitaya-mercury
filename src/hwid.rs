use uio::*;
use volatile_register::RO;

#[repr(C)]
pub struct HwIdRegs {
    hwid: RO<u32>,
    _rsv0: u32,
    efuse: RO<u32>,
    _rsv1: u32,
    dna: [RO<u32>; 2],
    _rsv2: [RO<u32>; 2],
    gith: [RO<u32>; 5],
}

pub struct HwId {
    regs: MemoryMap,
}

impl HwId {
    pub fn new() -> Self {
        let uio_num = get_uio_num("/dev/uio/hwid");
        let uio = UioDevice::new(uio_num).unwrap();
        let regs = uio.map_mapping(0).unwrap();
        HwId { regs }
    }

    /// Red Pitaya FPGA identification number (32bit).
    pub fn hwid(&self) -> u32 {
        self.regs().hwid.read()
    }

    /// Zynq FPGA efuse (32bit).
    pub fn efuse(&self) -> u32 {
        self.regs().efuse.read()
    }

    /// Zynq FPGA DNA number (57bit).
    ///
    /// A read-only value defined during manufacturing. Can be
    /// used as an almost unique device identifier.
    pub fn dna(&self) -> u64 {
        ((self.regs().dna[1].read() as u64) << 32) | self.regs().dna[0].read() as u64
    }

    /// Git hash.
    ///
    /// A full SHA-1 hash (160 bits, 40 hex characters) for the
    /// repository from which the FPGA was built.
    pub fn gith(&self) -> String {
        format!("{:08x}{:08x}{:08x}{:08x}{:08x}",
                self.regs().gith[4].read(),
                self.regs().gith[3].read(),
                self.regs().gith[2].read(),
                self.regs().gith[1].read(),
                self.regs().gith[0].read())
    }
}

impl RegDevice for HwId {
    type Registers = HwIdRegs;

    fn reg_mmap(&self) -> &MemoryMap {
        &self.regs
    }
}

impl Show for HwId {
    fn show(&self) {
        println!("hwid = {:x}", self.hwid());
        println!("efuse = {:x}", self.efuse());
        println!("dna = {:x}", self.dna());
        println!("gith = {}", self.gith());
    }
}
