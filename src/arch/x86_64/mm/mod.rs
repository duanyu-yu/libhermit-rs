// Copyright (c) 2017 Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

pub mod paging;
pub mod physicalmem;
pub mod virtualmem;

pub use self::paging::init_page_tables;
use core::mem;
use core::slice;

pub use x86::bits64::paging::PAddr as PhysAddr;
pub use x86::bits64::paging::VAddr as VirtAddr;

#[cfg(feature = "mmio")]
use crate::kernel::mmio::{MMIO_START, MMIO_END};
// pub use self::paging::{BasePageSize, PageTableEntryFlags, PageSize};

/// Memory translation, allocation and deallocation for MultibootInformation
struct MultibootMemory;

impl MultibootMemory {
	const fn new() -> Self {
		Self {}
	}
}

impl multiboot::information::MemoryManagement for MultibootMemory {
	unsafe fn paddr_to_slice(
		&self,
		p: multiboot::information::PAddr,
		sz: usize,
	) -> Option<&'static [u8]> {
		let ptr = mem::transmute(p);
		Some(slice::from_raw_parts(ptr, sz))
	}

	unsafe fn allocate(
		&mut self,
		_length: usize,
	) -> Option<(multiboot::information::PAddr, &mut [u8])> {
		None
	}

	unsafe fn deallocate(&mut self, addr: multiboot::information::PAddr) {
		if addr != 0 {
			unimplemented!()
		}
	}
}

static mut MEM: MultibootMemory = MultibootMemory::new();

#[cfg(feature = "mmio")]
fn init_mmio_region() {
	physicalmem::reserve(PhysAddr(0x700000), 0x100000); // test reserve from physical address space

	physicalmem::reserve(PhysAddr(MMIO_START as u64), MMIO_END - MMIO_START);
	// virtualmem::reserve(VirtAddr(MMIO_START as u64), MMIO_END - MMIO_START); // Error by reserve from virtual address space

	// paging::identity_map(PhysAddr(MMIO_START as u64), PhysAddr(MMIO_END as u64)); 

	// let count = (MMIO_END - MMIO_START) / BasePageSize::SIZE;
	// let flags = PageTableEntryFlags::empty();
	// paging::map::<BasePageSize>(VirtAddr(MMIO_START as u64), PhysAddr(MMIO_START as u64), count, flags);
	info!("after init mmio");
}

pub fn init() {
	paging::init();
	physicalmem::init();
	virtualmem::init();

	#[cfg(feature = "mmio")]
	init_mmio_region();
}
