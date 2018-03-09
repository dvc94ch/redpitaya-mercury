use std::fs;
use std::mem;
use std::path::PathBuf;
pub use _uio::{UioDevice, MemoryMap};

pub fn get_uio_num(path: &str) -> usize {
    let path = fs::read_link(PathBuf::from(path)).unwrap();
    let string = path.to_str().unwrap();
    let substring = &string[6..];
    substring.parse::<usize>().unwrap()
}

pub trait Default {
    /// Set registers into default (power-up) state.
    fn default(&mut self);
}

pub trait Show {
    /// Print FPGA module registers for debugging purposes.
    fn show(&self);
}

/*pub struct Device {
    uio: uio::UioDevice,
    regs: uio::MemoryMap,
    buffer: Option<uio::MemoryMap>,
}

impl Device {
    pub fn new(path: &'str) {
        let uio_num = util::get_uio_num(Self::DEVICE_FILE_PATH);
        let uio = _uio::UioDevice::new(uio_num).unwrap();
        let regs = uio.map_mapping(0).unwrap();
        let buffer = uio.map_mapping(1);
        Self { uio, regs, buffer }
    }
}

impl Show for T where T: Device + RegDevice {
    fn show(&self) {
        self.regs().show();
    }
}*/

pub trait RegDevice {
    type Registers;

    fn reg_mmap(&self) -> &MemoryMap;

    fn regs(&self) -> &Self::Registers {
        unsafe { mem::transmute::<*mut u8, &Self::Registers>(self.reg_mmap().data()) }
    }

    fn mut_regs(&self) -> &mut Self::Registers {
        unsafe { mem::transmute::<*mut u8, &mut Self::Registers>(self.reg_mmap().data()) }
    }
}

pub trait BufferDevice {
    type Buffer;

    fn buffer_mmap(&self) -> &MemoryMap;

    fn buffer(&self) -> &Self::Buffer {
        unsafe { mem::transmute::<*mut u8, &Self::Buffer>(self.buffer_mmap().data()) }
    }

    fn mut_buffer(&self) -> &mut Self::Buffer {
        unsafe { mem::transmute::<*mut u8, &mut Self::Buffer>(self.buffer_mmap().data()) }
    }
}
