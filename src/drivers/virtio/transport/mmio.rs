// Copyright (c) 2020 Frederik Schulz, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::arch::kernel::mmio as kernel_mmio;
use crate::arch::kernel::mmio::{MMIO, MAGIC_VALUE};

#[repr(u32)]
pub enum DevId {
    INVALID = 0x0,
    VIRTIO_DEV_ID_NET = 0x1,
    VIRTIO_DEV_ID_BLK = 0x2,
}

impl From<DevID> for u32 {
    fn from(val: DevID) -> u32 {
        match val {
            DevID::INVALID => 0x0,
            DevID::VIRTIO_DEV_ID_NET => 0x1,
            DevID::VIRTIO_DEV_ID_BLK => 0x2,
        }
    }
}

impl From<u32> for DevID {
    fn from(val: u32) -> Self {
        match val {
            0x1 => DevID::VIRTIO_DEV_ID_NET,
            0x2 => DevID::VIRTIO_DEV_ID_BLK,
            _ => DevID::INVALID,
        }
    }
}

pub fn init_device(mmio: &MMIO) {
    assert_eq!(mmio.magic_value, MAGIC_VALUE);
    assert_eq!(mmio.version, 0x1 as u32);

    
}