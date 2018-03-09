use std::convert::TryFrom;
use uio;
use volatile_register::RW;

const CTL_RESET_MASK: u32 = 1 << 0;
const CTL_START_MASK: u32 = 1 << 1;
const CTL_STOP_MASK: u32 = 1 << 2;
const CTL_TRIGGER_MASK: u32 = 1 << 3;

pub enum SyncSource {
    Gen0 = 0,
    Gen1 = 1,
    Osc0 = 2,
    Osc1 = 3,
    Lg = 4,
    La = 5,
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromSyncSourceError(());

impl TryFrom<u32> for SyncSource {
    type Error = TryFromSyncSourceError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(SyncSource::Gen0),
            1 => Ok(SyncSource::Gen1),
            2 => Ok(SyncSource::Osc0),
            3 => Ok(SyncSource::Osc1),
            4 => Ok(SyncSource::Lg),
            5 => Ok(SyncSource::La),
            _ => Err(TryFromSyncSourceError(())),
        }
    }
}

pub enum TriggerSource {
    Gen0 = 1 << 0,
    Gen1 = 1 << 1,
    Osc0 = 1 << 2,
    Osc1 = 1 << 3,
    Lg = 1 << 4,
    La = 1 << 5,
}

#[derive(Debug, Copy, Clone)]
pub struct TryFromTriggerSourceError(());

impl TryFrom<u32> for TriggerSource {
    type Error = TryFromTriggerSourceError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0b000001 => Ok(TriggerSource::Gen0),
            0b000010 => Ok(TriggerSource::Gen1),
            0b000100 => Ok(TriggerSource::Osc0),
            0b001000 => Ok(TriggerSource::Osc1),
            0b010000 => Ok(TriggerSource::Lg),
            0b100000 => Ok(TriggerSource::La),
            _ => Err(TryFromTriggerSourceError(())),
        }
    }
}

#[repr(C)]
pub struct EventRegs {
    ctl_status: RW<u32>,
    cfg_event: RW<u32>,
    cfg_trigger: RW<u32>
}

impl uio::Default for EventRegs {
    fn default(&mut self) {
        unsafe {
            self.cfg_event.write(0);
            self.cfg_trigger.write(0);
        }
    }
}

impl uio::Show for EventRegs {
    fn show(&self) {
        println!("ctl_status = {:x}", self.ctl_status.read());
        println!("cfg_event = {:x}", self.cfg_event.read());
        println!("cfg_trigger = {:x}", self.cfg_trigger.read());
    }
}

pub trait EventRegsAPI {
    /// Reset state machine
    /// Is used to synchronize always running streams.
    fn reset(&mut self);
    /// Start state machine
    fn start(&mut self);
    /// Stop state machine
    fn stop(&mut self);
    /// Activate software trigger
    fn trigger(&mut self);
    /// Start state machine and activate software trigger
    fn start_trigger(&mut self);
    /// Run status
    fn is_running(&self) -> bool;
    /// Trigger status
    fn is_triggered(&self) -> bool;
    /// Get software event source
    fn sync_source(&self) -> SyncSource;
    /// Set software event source
    fn set_sync_source(&mut self, event: SyncSource);
    /// Get hardware trigger source
    fn trigger_source(&self) -> TriggerSource;
    /// Set hardware trigger source
    fn set_trigger_source(&mut self, trigger: TriggerSource);
}

impl EventRegsAPI for EventRegs {
    fn reset(&mut self) {
        unsafe { self.ctl_status.write(CTL_RESET_MASK); }
    }

    fn start(&mut self) {
        unsafe { self.ctl_status.write(CTL_START_MASK); }
    }

    fn stop(&mut self) {
        unsafe { self.ctl_status.write(CTL_STOP_MASK); }
    }

    fn trigger(&mut self) {
        unsafe { self.ctl_status.write(CTL_TRIGGER_MASK); }
    }

    fn start_trigger(&mut self) {
        unsafe { self.ctl_status.write(CTL_TRIGGER_MASK | CTL_START_MASK); }
    }

    fn is_running(&self) -> bool {
        self.ctl_status.read() & CTL_START_MASK == CTL_START_MASK
    }

    fn is_triggered(&self) -> bool {
        self.ctl_status.read() & CTL_TRIGGER_MASK == CTL_TRIGGER_MASK
    }

    fn sync_source(&self) -> SyncSource {
        SyncSource::try_from(self.cfg_event.read()).unwrap()
    }

    fn set_sync_source(&mut self, event: SyncSource) {
        unsafe { self.cfg_event.write(event as u32); }
    }

    fn trigger_source(&self) -> TriggerSource {
        TriggerSource::try_from(self.cfg_trigger.read()).unwrap()
    }

    fn set_trigger_source(&mut self, trigger: TriggerSource) {
        unsafe { self.cfg_trigger.write(trigger as u32); }
    }
}
