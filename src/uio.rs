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

pub trait IrqAPI {
    /// Enable interrupt
    fn enable_irq(&mut self);
    /// Disable interrupt
    fn disable_irq(&mut self);
    /// Wait for interrupt
    fn wait_irq(&mut self) -> u32;
}
