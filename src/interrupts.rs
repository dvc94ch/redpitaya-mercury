use uio::*;

pub struct Interrupt {
    uio: UioDevice,
    regs: MemoryMap,
}

impl Interrupt {
    pub fn new() -> Self {
        let uio_num = get_uio_num("/dev/uio/ps2pl");
        let uio = UioDevice::new(uio_num).unwrap();
        let regs = uio.map_mapping(0).unwrap();
        Interrupt { uio, regs }
    }

    pub fn enable(&mut self) {
        self.uio.irq_enable().unwrap();
    }

    pub fn disable(&mut self) {
        self.uio.irq_disable().unwrap();
    }

    pub fn wfi(&mut self) -> u32 {
        self.uio.irq_wait().unwrap()
    }
}
