extern crate redpitaya_mercury;

use std::{thread, time};
use redpitaya_mercury::prelude::*;
use redpitaya_mercury::la;

fn main() {
    let hwid = HwId::new();
    hwid.show();

    // setup logic analyzer
    let mut la = LogicAnalyzer::new();
    la.default();
    la.set_input_mask(0xffff);
    la.set_trigger_pre(la::BUFFER_SIZE / 2);
    la.set_trigger_post(la::BUFFER_SIZE / 2);
    la.set_decimation(1000);
    la.set_sync_source(SyncSource::La);
    la.set_trigger_source(TriggerSource::La);
    la.show();

    thread::sleep(time::Duration::from_millis(1000));

    // reset and start
    la.reset();
    la.start_trigger();

    // wait for data
    while la.is_running() {}

    // print data
    for sample in la.buffer().iter() {
        println!("{:016b}", sample.read());
    }
}
