extern crate redpitaya_mercury;

use std::{thread, time};
use redpitaya_mercury::prelude::*;
use redpitaya_mercury::la;

fn main() {
    let hwid = HwId::new();
    hwid.show();

    let mut ps2pl = Interrupt::new();
    ps2pl.enable();

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

    println!("Wait for interrupt");
    let res = ps2pl.wfi();
    println!("Received interrupt {}", res);

    // print data
    //for sample in la.buffer().iter() {
    //    println!("{:016b}", sample.read());
    //}
}
