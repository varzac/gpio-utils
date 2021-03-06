// Copyright (C) 2016, The gpio-utils Authors

use options::GpioUnexportOptions;
use config::GpioConfig;
use std::process::exit;
use export;

pub fn main(config: &GpioConfig, opts: &GpioUnexportOptions) {
    let pin_config = config.get_pin(&opts.pin[..]).unwrap_or_else(|| {
        println!("Unable to find config entry for pin '{}'", opts.pin);
        exit(1)
    });

    let symlink_root = match opts.symlink_root {
        Some(ref slr) => &slr[..],
        None => config.get_symlink_root(),
    };

    if let Err(e) = export::unexport(pin_config, Some(symlink_root)) {
        println!("Error occurred while unexport pin {:?}", pin_config);
        println!("{}", e);
        exit(1);
    }
}
