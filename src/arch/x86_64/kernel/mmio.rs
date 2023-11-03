use alloc::vec::Vec;
use core::str;

use align_address::Align;
use hermit_sync::{without_interrupts, InterruptTicketMutex};

use crate::arch::x86_64::mm::paging::{
	BasePageSize, PageSize, PageTableEntryFlags, PageTableEntryFlagsExt,
};
use crate::arch::x86_64::mm::{paging, PhysAddr};
use crate::drivers::net::virtio_net::VirtioNetDriver;
use crate::drivers::virtio::transport::mmio as mmio_virtio;
use crate::drivers::virtio::transport::mmio::{DevId, MmioRegisterLayout, VirtioDriver};
#[cfg(feature = "fc")]
use crate::env;

pub const MAGIC_VALUE: u32 = 0x74726976;

#[cfg(feature = "fc")]
pub const MMIO_START: usize = 0x00000000d0000000;
#[cfg(not(feature = "fc"))]
pub const MMIO_START: usize = 0x00000000c0000000;

#[cfg(feature = "fc")]
pub const MMIO_END: usize = 0x00000000ffffffff;
#[cfg(not(feature = "fc"))]
pub const MMIO_END: usize = 0x00000000c0000fff;

#[cfg(feature = "fc")]
pub const MMIO_LEN: usize = 0x1000;
#[cfg(not(feature = "fc"))]
pub const MMIO_LEN: usize = 0x200;

const IRQ_NUMBER: u8 = 12;

static mut MMIO_DRIVERS: Vec<MmioDriver> = Vec::new();

pub(crate) enum MmioDriver {
	VirtioNet(InterruptTicketMutex<VirtioNetDriver>),
}

impl MmioDriver {
	#[allow(unreachable_patterns)]
	fn get_network_driver(&self) -> Option<&InterruptTicketMutex<VirtioNetDriver>> {
		match self {
			Self::VirtioNet(drv) => Some(drv),
			_ => None,
		}
	}
}

/// Tries to find the network device within the specified address range.
/// Returns a reference to it within the Ok() if successful or an Err() on failure.
#[cfg(not(feature = "fc"))]
pub fn detect_network() -> Result<&'static mut MmioRegisterLayout, &'static str> {
	// Trigger page mapping in the first iteration!
	let mut current_page = 0;
	let virtual_address =
		crate::arch::mm::virtualmem::allocate(BasePageSize::SIZE as usize).unwrap();

	// Look for the device-ID in all possible 64-byte aligned addresses within this range.
	for current_address in (MMIO_START..MMIO_END).step_by(MMIO_LEN) {
		trace!(
			"try to detect MMIO device at physical address {:#X}",
			current_address
		);
		// Have we crossed a page boundary in the last iteration?
		// info!("before the {}. paging", current_page);
		if current_address / BasePageSize::SIZE as usize > current_page {
			let mut flags = PageTableEntryFlags::empty();
			flags.normal().writable();
			paging::map::<BasePageSize>(
				virtual_address,
				PhysAddr::from(current_address.align_down(BasePageSize::SIZE as usize)),
				1,
				flags,
			);

			current_page = current_address / BasePageSize::SIZE as usize;
		}

		// Verify the first register value to find out if this is really an MMIO magic-value.
		let mmio = unsafe {
			&mut *((virtual_address.as_usize()
				| (current_address & (BasePageSize::SIZE as usize - 1)))
				as *mut MmioRegisterLayout)
		};

		let magic = mmio.get_magic_value();
		let version = mmio.get_version();

		if magic != MAGIC_VALUE {
			trace!("It's not a MMIO-device at {:#X}", mmio as *const _ as usize);
			continue;
		}

		if version != 2 {
			trace!("Found a legacy device, which isn't supported");
			continue;
		}

		// We found a MMIO-device (whose 512-bit address in this structure).
		trace!("Found a MMIO-device at {:#X}", mmio as *const _ as usize);

		// Verify the device-ID to find the network card
		let id = mmio.get_device_id();

		if id != DevId::VIRTIO_DEV_ID_NET {
			trace!(
				"It's not a network card at {:#X}",
				mmio as *const _ as usize
			);
			continue;
		}

		info!("Found network card at {:#X}", mmio as *const _ as usize);

		crate::arch::mm::physicalmem::reserve(
			PhysAddr::from(current_address.align_down(BasePageSize::SIZE as usize)),
			BasePageSize::SIZE as usize,
		);

		//mmio.print_information();

		return Ok(mmio);
	}

	// frees obsolete virtual memory region for MMIO devices
	crate::arch::mm::virtualmem::deallocate(virtual_address, BasePageSize::SIZE as usize);

	Err("Network card not found!")
}

/// Detect network card from cmdline if running on firecracker.
/// Returns a reference to it within the Ok() if successful or an Err() on failure.
#[cfg(feature = "fc")]
pub fn detect_network() -> Result<&'static mut MmioRegisterLayout, &'static str> {
	let mut current_page = 0;
	let virtual_address = crate::arch::mm::virtualmem::allocate(BasePageSize::SIZE as usize).unwrap();

	let clis = env::devices();

	if clis.is_empty() {
		return Err("No devices info from cmd!");
	}

	for cli in clis {
		debug!("MMIO Device infomation from CLI: {cli}");

		let mut words: Vec<_> = cli.split(|c| c == '@' || c == ':').collect();

		let mut id = "";

		if words.len() == 4 {
			id = words.pop().unwrap();
		}

		let _irq = words.pop().unwrap();
		let baseaddr_str = words.pop().unwrap();
		let _size = words.pop().unwrap();

		let baseaddr = usize::from_str_radix(&baseaddr_str[2..], 16).unwrap();

		if baseaddr / BasePageSize::SIZE as usize > current_page {
			let mut flags = PageTableEntryFlags::empty();
			flags.normal().writable();
			paging::map::<BasePageSize>(
				virtual_address,
				PhysAddr::from(baseaddr.align_down(BasePageSize::SIZE as usize)),
				1,
				flags,
			);

			current_page = baseaddr / BasePageSize::SIZE as usize;
		}

		let mmio = unsafe {
			&mut *((virtual_address.as_usize()
				| (baseaddr & (BasePageSize::SIZE as usize - 1)))
				as *mut MmioRegisterLayout)
		};

		if !mmio.check_magic() || !mmio.is_non_legacy() {
			debug!("Invalid Magic or/and it's a legacy device!");
			continue;
		}

		trace!("Found a MMIO-device at {:#X}", mmio as *const _ as usize);

		if mmio.get_device_id() != DevId::VIRTIO_DEV_ID_NET {
			trace!(
				"It's not a network card at {:#X}",
				mmio as *const _ as usize
			);
			continue;
		}

		trace!("Found network card at {:#X}", mmio as *const _ as usize);

		crate::arch::mm::physicalmem::reserve(
			PhysAddr::from(baseaddr.align_down(BasePageSize::SIZE as usize)),
			BasePageSize::SIZE as usize,
		);

		return Ok(mmio);
	}

	// frees obsolete virtual memory region for MMIO devices
	crate::arch::mm::virtualmem::deallocate(virtual_address, BasePageSize::SIZE as usize);

	Err("Network card not found!")
}

#[cfg(not(feature = "fc"))]
fn detect_from_addr(start: usize, end: usize, len: usize) -> Vec<&'static mut MmioRegisterLayout> {
	let mut mmios = Vec::new();

	// Trigger page mapping in the first iteration!
	let mut current_page = 0;
	let virtual_address =
		crate::arch::mm::virtualmem::allocate(BasePageSize::SIZE as usize).unwrap();

	for current_address in (start..end).step_by(len) {
		trace!(
			"Detecting MMIO device at physical address {:#X}",
			current_address
		);
		// Check if we crossed a page boundary in the last iteration.
		if current_address / BasePageSize::SIZE as usize > current_page {
			let mut flags = PageTableEntryFlags::empty();
			flags.normal().writable();
			paging::map::<BasePageSize>(
				virtual_address,
				PhysAddr::from(current_address.align_down(BasePageSize::SIZE as usize)),
				1,
				flags,
			);

			current_page = current_address / BasePageSize::SIZE as usize;
		}

		let mmio = unsafe {
			&mut *((virtual_address.as_usize()
				| (current_address & (BasePageSize::SIZE as usize - 1)))
				as *mut MmioRegisterLayout)
		};

		if !mmio.check_magic() || !mmio.is_non_legacy() {
			debug!("Invalid Magic or/and it's a legacy device!");
			continue;
		}

		trace!("Found a MMIO-device at {:#X}", mmio as *const _ as usize);

		mmios.push(mmio);
	}

	if mmios.is_empty() {
		// frees obsolete virtual memory region for MMIO devices
		crate::arch::mm::virtualmem::deallocate(virtual_address, BasePageSize::SIZE as usize);

		debug!("MMIO device not found!")
	}

	return mmios;
}

pub(crate) fn register_driver(drv: MmioDriver) {
	unsafe {
		MMIO_DRIVERS.push(drv);
	}
}

pub(crate) fn get_network_driver() -> Option<&'static InterruptTicketMutex<VirtioNetDriver>> {
	unsafe { MMIO_DRIVERS.iter().find_map(|drv| drv.get_network_driver()) }
}

pub(crate) fn init_drivers() {
	// virtio: MMIO Device Discovery
	without_interrupts(|| {
		if let Ok(mmio) = detect_network() {
			warn!(
				"Found MMIO device, but we guess the interrupt number {}!",
				IRQ_NUMBER
			);
			if let Ok(VirtioDriver::Network(drv)) = mmio_virtio::init_device(mmio, IRQ_NUMBER) {
				register_driver(MmioDriver::VirtioNet(InterruptTicketMutex::new(drv)))
			}
		} else {
			warn!("Unable to find mmio device");
		}
	});
}
