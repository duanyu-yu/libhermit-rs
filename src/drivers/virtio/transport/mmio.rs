// Copyright (c) 2020 Frederik Schulz, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use crate::arch::kernel::mmio as kernel_mmio;
use crate::arch::kernel::mmio::{MMIO, MAGIC_VALUE};
use crate::arch::x86_64::mm::virtualmem;
use crate::arch::x86_64::mm::paging;
use crate::arch::x86_64::mm::paging::{BasePageSize, PageTableEntryFlags, PageSize};
use crate::arch::x86_64::mm::{PhysAddr, VirtAddr};

static mut VIRTQUEUE_VIRTADDR: VirtAddr = VirtAddr::zero();
static mut VIRTQUEUE_PHYSADDR: PhysAddr = PhysAddr::zero();

// #[repr(u32)]
// pub enum DevId {
//     INVALID = 0x0,
//     VIRTIO_DEV_ID_NET = 0x1,
//     VIRTIO_DEV_ID_BLK = 0x2,
// }

// impl From<DevID> for u32 {
//     fn from(val: DevID) -> u32 {
//         match val {
//             DevID::INVALID => 0x0,
//             DevID::VIRTIO_DEV_ID_NET => 0x1,
//             DevID::VIRTIO_DEV_ID_BLK => 0x2,
//         }
//     }
// }

// impl From<u32> for DevID {
//     fn from(val: u32) -> Self {
//         match val {
//             0x1 => DevID::VIRTIO_DEV_ID_NET,
//             0x2 => DevID::VIRTIO_DEV_ID_BLK,
//             _ => DevID::INVALID,
//         }
//     }
// }

// pub fn init_device(mmio: &MMIO) {
//     assert_eq!(mmio.magic_value, MAGIC_VALUE);
//     assert_eq!(mmio.version, 0x1 as u32);

    
// }

pub fn init_virtqueue(mmio: &mut MMIO, index: u32) -> Result<&MMIO, &'static str> {
    mmio.set_queue_sel(index);

    assert_eq!(
        mmio.queue_pfn,
        0x0 as u32,
        "The queue {} is already in use!",
        index
    );

    let queue_num_max: usize = mmio.queue_num_max as usize;   
    if queue_num_max == 0x0 {
        return Err("The queue is not available!");
    }

    let queue_num: usize = queue_num_max;
    assert!(
        queue_num <= queue_num_max, 
        "The queue size should smaller than or equal to {}",
        queue_num_max
    );

    let queue_size: usize = queue_num * BasePageSize::SIZE;

    VIRTQUEUE_VIRTADDR = virtualmem::allocate_aligned(queue_size, BasePageSize::SIZE).unwrap();
    VIRTQUEUE_PHYSADDR = PhysAddr::from(align_down!(VIRTQUEUE_VIRTADDR.as_usize(), BasePageSize::SIZE));

    let mut flags = PageTableEntryFlags::empty();
	flags.device().writable().execute_disable();
	paging::map::<BasePageSize>(
		VIRTQUEUE_VIRTADDR,
		VIRTQUEUE_PHYSADDR,
		1,
		flags,
	);

    mmio.set_queue_num(queue_num as u32);
    mmio.set_queue_align(BasePageSize::SIZE as u32);

    let page_num = VIRTQUEUE_PHYSADDR.as_usize() / BasePageSize::SIZE;

    mmio.set_queue_pfn(page_num as u32);

    mmio.print_information();

    Ok(mmio)
}
