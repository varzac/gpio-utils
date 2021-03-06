// Copyright (C) 2016, Paul Osborne <osbpau@gmail.com>

use options::GpioExportAllOptions;
use config::GpioConfig;
use std::process::exit;
use export;

pub fn main(config: &GpioConfig, opts: &GpioExportAllOptions) {
    let symlink_root = match opts.symlink_root {
        Some(ref slr) => &slr[..],
        None => config.get_symlink_root(),
    };

    // export all pins except those for which export is set to false
    for pin in config.get_pins().iter().filter(|p| p.export) {
        if let Err(e) = export::export(pin, Some(symlink_root)) {
            println!("Error occurred while exporting pin: {:?}", pin);
            println!("{}", e);
            exit(1);
        }
    }
}
