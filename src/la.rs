use acquire::*;
use event::*;
use la_mask::*;
use la_rle::*;
use la_trigger::*;
use uio::*;
use volatile_register::RO;

pub const BUFFER_SIZE: u32 = 0x4000; // 2**14 = 16384
pub const SAMPLING_FREQUENCY: u32 = 125_000_000;

#[repr(C)]
pub struct LaRegs {
    event: EventRegs,
    _rsv0: u32,
    acquire: AcquireRegs,
    trigger: LaTriggerRegs,
    rle: LaRleRegs,
    _rsv1: u32,
    mask: LaMaskRegs,
}

impl Default for LaRegs {
    /// Set registers to default (power-up) state.
    fn default(&mut self) {
        self.event.default();
        self.acquire.default();
        self.trigger.default();
        self.rle.default();
        self.mask.default();
    }
}

impl Show for LaRegs {
    /// Print FPGA module registers.
    fn show(&self) {
        self.event.show();
        self.acquire.show();
        self.trigger.show();
        self.rle.show();
        self.mask.show();
    }
}

pub struct LogicAnalyzer {
    regs: MemoryMap,
    buffer: MemoryMap,
}

impl LogicAnalyzer {
    pub fn new() -> Self {
        let uio_num = get_uio_num("/dev/uio/la");
        let uio = UioDevice::new(uio_num).unwrap();
        let regs = uio.map_mapping(0).unwrap();
        let buffer = uio.map_mapping(1).unwrap();
        LogicAnalyzer { regs, buffer }
    }

    /// Returns the sample rate depending on decimation factor.
    pub fn sample_rate(&self) -> f32 {
        SAMPLING_FREQUENCY as f32 / self.decimation() as f32
    }

    /// Returns the sample period depending on decimation factor.
    pub fn sample_period(&self) -> f32 {
        1.0 / self.sample_rate()
    }

    /*
    /// Mask out overflow bit and sum pre and post trigger counters.
    pub fn pointer(&self) -> usize {
        let count = self.trigger_pre_status() + self.trigger_post_status();
        let address = count as usize % BUFFER_SIZE;
        address
    }

    /// Read data buffer.
    /// num_samples : Number of samples to read from FPGA buffer.
    /// Returns array containing binary samples. The data is aligned to the end
    /// of the last sample stored in the buffer.
    pub fn data(&self, num_samples: usize) -> Iterator<u16> {
        let ptr = self.pointer();
        let address = (BUFFER_SIZE + ptr - num_samples) % BUFFER_SIZE;
    }
    */
}

impl RegDevice for LogicAnalyzer {
    type Registers = LaRegs;

    fn reg_mmap(&self) -> &MemoryMap {
        &self.regs
    }
}

impl BufferDevice for LogicAnalyzer {
    type Buffer = [RO<u16>; BUFFER_SIZE as usize];

    fn buffer_mmap(&self) -> &MemoryMap {
        &self.buffer
    }
}

impl Default for LogicAnalyzer {
    fn default(&mut self) {
        self.mut_regs().default();
    }
}

impl Show for LogicAnalyzer {
    fn show(&self) {
        self.regs().show();
    }
}

impl EventRegsAPI for LogicAnalyzer {
    fn reset(&mut self) {
        self.mut_regs().event.reset();
    }
    fn start(&mut self) {
        self.mut_regs().event.start();
    }
    fn stop(&mut self) {
        self.mut_regs().event.stop();
    }
    fn trigger(&mut self) {
        self.mut_regs().event.trigger();
    }
    fn start_trigger(&mut self) {
        self.mut_regs().event.start_trigger();
    }
    fn is_running(&self) -> bool {
        self.regs().event.is_running()
    }
    fn is_triggered(&self) -> bool {
        self.regs().event.is_triggered()
    }
    fn sync_source(&self) -> SyncSource {
        self.regs().event.sync_source()
    }
    fn set_sync_source(&mut self, event: SyncSource) {
        self.mut_regs().event.set_sync_source(event);
    }
    fn trigger_source(&self) -> TriggerSource {
        self.regs().event.trigger_source()
    }
    fn set_trigger_source(&mut self, trigger: TriggerSource) {
        self.mut_regs().event.set_trigger_source(trigger);
    }
}

impl AcquireRegsAPI for LogicAnalyzer {
    fn trigger_pre(&self) -> u32 {
        self.regs().acquire.trigger_pre()
    }
    fn set_trigger_pre(&mut self, delay: u32) {
        self.mut_regs().acquire.set_trigger_pre(delay);
    }
    fn trigger_post(&self) -> u32 {
        self.regs().acquire.trigger_post()
    }
    fn set_trigger_post(&mut self, delay: u32) {
        self.mut_regs().acquire.set_trigger_post(delay);
    }
    fn trigger_pre_status(&self) -> u32 {
        self.regs().acquire.trigger_pre_status()
    }
    fn trigger_post_status(&self) -> u32 {
        self.regs().acquire.trigger_post_status()
    }
}

impl LaMaskRegsAPI for LogicAnalyzer {
    fn input_mask(&self) -> u32 {
        self.regs().mask.input_mask()
    }
    fn set_input_mask(&mut self, mask: u32) {
        self.mut_regs().mask.set_input_mask(mask);
    }
    fn input_polarity(&self) -> u32 {
        self.regs().mask.input_polarity()
    }
    fn set_input_polarity(&mut self, pol: u32) {
        self.mut_regs().mask.set_input_polarity(pol);
    }
    fn decimation(&self) -> u32 {
        self.regs().mask.decimation()
    }
    fn set_decimation(&mut self, dec: u32) {
        self.mut_regs().mask.set_decimation(dec);
    }
}

impl LaRleRegsAPI for LogicAnalyzer {
    fn rle(&self) -> bool {
        self.regs().rle.rle()
    }
    fn set_rle(&mut self, rle: bool) {
        self.mut_regs().rle.set_rle(rle);
    }
    fn counter_current(&self) -> u32 {
        self.regs().rle.counter_current()
    }
    fn counter_last(&self) -> u32 {
        self.regs().rle.counter_last()
    }
}

impl LaTriggerRegsAPI for LogicAnalyzer {
    fn trigger_mask(&self) -> u32 {
        self.regs().trigger.trigger_mask()
    }
    fn set_trigger_mask(&mut self, mask: u32) {
        self.mut_regs().trigger.set_trigger_mask(mask);
    }
    fn trigger_value(&self) -> u32 {
        self.regs().trigger.trigger_value()
    }
    fn set_trigger_value(&mut self, value: u32) {
        self.mut_regs().trigger.set_trigger_value(value);
    }
    fn pos_edge_trigger_mask(&self) -> u32 {
        self.regs().trigger.pos_edge_trigger_mask()
    }
    fn set_pos_edge_trigger_mask(&mut self, mask: u32) {
        self.mut_regs().trigger.set_pos_edge_trigger_mask(mask);
    }
    fn neg_edge_trigger_mask(&self) -> u32 {
        self.regs().trigger.neg_edge_trigger_mask()
    }
    fn set_neg_edge_trigger_mask(&mut self, mask: u32) {
        self.mut_regs().trigger.set_neg_edge_trigger_mask(mask);
    }
}
