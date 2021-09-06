use crate::arch::x86_64::mm::paging::{BasePageSize, PageSize, PageTableEntryFlags};
use crate::arch::x86_64::mm::{paging};
use crate::arch::x86_64::mm::{PhysAddr, VirtAddr};
use crate::arch::x86_64::mm::paging::virt_to_phys;
use core::{str};

const MAGIC_VALUE: u32 = 0x74726976 as u32;

pub const MMIO_START: usize = 0x00000000c0000000 as usize;
pub const MMIO_END: usize = 0x00000000c0000fff as usize;

/// The Layout of MMIO Device 
#[repr(C, align(32))]
pub struct MMIO {
    magic_value: u32,
    version: u32,
    device_id: u32,
    vendor_id: u32,
    device_features: u32,
    device_features_sel: u32,
    driver_features: u32,
    driver_features_sel: u32,
    queue_sel: u32,
    queue_num_max: u32,
    queue_num: u32,
    queue_ready: u32,
    queue_notify: u32,
    interrupt_status: u32,
    interrupt_ack: u32,
    status: u32,
    // queue_desc_low: u32,
    // queue_desc_high: u32,
    // queue_avai_low: u32,
    // queue_avai_high: u32,
    // queue_used_low: u32,
    // queue_used_high: u32,
    // config_generation: u32,
    // config: u32,
}

impl MMIO {
    fn magic_value(&self) -> u32 {
        self.magic_value
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn device_id(&self) -> u32 {
            self.device_id
    }

    fn vendor_id(&self) -> u32 {
        self.vendor_id
    }

    fn device_features(&self) -> u32 {
        self.device_features
    }

    fn device_features_sel(&self) -> u32 {
        self.device_features_sel
    }

    fn driver_features(&self) -> u32 {
        self.driver_features
    }

    fn driver_features_sel(&self) -> u32 {
        self.driver_features_sel
    }

    fn queue_sel(&self) -> u32 {
        self.queue_sel
    }

    fn queue_num_max(&self) -> u32 {
        self.queue_num_max
    }

    fn queue_num(&self) -> u32 {
        self.queue_num
    }

    fn queue_ready(&self) -> u32 {
        self.queue_ready
    }

    fn queue_notify(&self) -> u32 {
        self.queue_notify
    }

    fn interrupt_status(&self) -> u32 {
        self.interrupt_status
    }

    fn interrupt_ack(&self) -> u32 {
        self.interrupt_ack
    }

    fn status(&self) -> u32 {
        self.status
    }

    fn print_information(&self) {
        infoheader!(" MMIO INFORMATION ");

        infoentry!("Device version number", "{:#X}", self.version);
        infoentry!("Device ID", "{:#X}", self.device_id);
        infoentry!("Vendor ID", "{:#X}", self.vendor_id);
        infoentry!("Device Features", "{:#X}", self.device_features);
        infoentry!("Device Features selection", "{:#X}", self.device_features_sel);
        infoentry!("Driver Features", "{:#X}", self.driver_features);
        infoentry!("Driver Features selection", "{:#X}", self.driver_features_sel);
        infoentry!("Virtual queue index", "{:#X}", self.queue_sel);
        infoentry!("Maximum virtual queue size", "{:#X}", self.queue_num_max);
        infoentry!("Virtual queue size", "{:#X}", self.queue_num);
        infoentry!("Virtual queue ready bit", "{:#X}", self.queue_ready);
        infoentry!("Queue notifier", "{:#X}", self.queue_notify);
        infoentry!("Interrupt status", "{:#X}", self.interrupt_status);
        infoentry!("Interrupt acknowledge", "{:#X}", self.interrupt_ack);
        infoentry!("Device status", "{:#X}", self.status);

        infofooter!();
    }
}

/// Tries to find the MMIO magic-value within the specified address range.
/// Returns a reference to it within the Ok() if successful or an Err() on failure.
pub fn detect_mmio(start_address: PhysAddr, end_address: PhysAddr) -> Result<&'static MMIO, &'static str> {
    // Trigger page mapping in the first iteration!
	let mut current_page = 0;

    // Look for the MMIO magic-value in all possible 64-byte aligned addresses within this range.
    for current_address in (start_address.as_usize()..end_address.as_usize()).step_by(512) {
        info!("detecting MMIO at {:#X}", current_address);
        // Have we crossed a page boundary in the last iteration?
        // info!("before the {}. paging", current_page);
        if current_address / BasePageSize::SIZE > current_page {
            let mut flags = PageTableEntryFlags::empty();
	        flags.normal().writable();
	        paging::map::<BasePageSize>(
		        VirtAddr::from(align_down!(current_address, BasePageSize::SIZE)),
		        PhysAddr::from(align_down!(current_address, BasePageSize::SIZE)),
		        1,
		        flags,
	        );

			current_page = current_address / BasePageSize::SIZE;

			info!("map {:#X} to {:#X}", current_address, virt_to_phys(VirtAddr::from(align_down!(current_address, BasePageSize::SIZE))).as_u64());
		}   

        // Verify the first register value to find out if this is really an MMIO magic-value.
        let mmio = unsafe { &*(current_address as *const MMIO) };

        let magic = mmio.magic_value();

        // info!("The magic_value at {:#X} is {:#X}", current_address, magic);

        if magic != MAGIC_VALUE {
            continue;
        }

		// We were successful! Return a pointer to the MMIO (whose 512-bit address in this structure).  
        info!(
            "Found a magic-value at {:#X}",
            current_address
        );

        return Ok(mmio);
    }

    // magic-value not found
    Err("Magic-value of MMIO not found!")
}

/// Tries to find the network device within the specified address range.
/// Returns a reference to it within the Ok() if successful or an Err() on failure.
pub fn detect_network() -> Result<&'static MMIO, &'static str> {
    // Trigger page mapping in the first iteration!
	let mut current_page = 0;

    // Look for the device-ID in all possible 64-byte aligned addresses within this range.
    for current_address in (MMIO_START..MMIO_END).step_by(512) {
        info!("detecting MMIO at {:#X}", current_address);
        // Have we crossed a page boundary in the last iteration?
        // info!("before the {}. paging", current_page);
        if current_address / BasePageSize::SIZE > current_page {
            let mut flags = PageTableEntryFlags::empty();
	        flags.normal().writable();
	        paging::map::<BasePageSize>(
		        VirtAddr::from(align_down!(current_address, BasePageSize::SIZE)),
		        PhysAddr::from(align_down!(current_address, BasePageSize::SIZE)),
		        1,
		        flags,
	        );

			current_page = current_address / BasePageSize::SIZE;

			info!("map {:#X} to {:#X}", current_address, virt_to_phys(VirtAddr::from(align_down!(current_address, BasePageSize::SIZE))).as_u64());
		}   

        // Verify the first register value to find out if this is really an MMIO magic-value.
        let mmio = unsafe { &*(current_address as *const MMIO) };

        let magic = mmio.magic_value();

        if magic != MAGIC_VALUE {
            info!("It's not a MMIO-device at {:#X}", current_address);
            continue;
        }

		// We found a MMIO-device (whose 512-bit address in this structure).  
        info!(
            "Found a MMIO-device at {:#X}",
            current_address
        );

        // Verify the device-ID to find the network card
        let id = mmio.device_id();

        if id != 0x1 {
            info!("It's not a network card at {:#X}", current_address);
            continue;
        }

        info!("Found network card at {:#X}", current_address);

        mmio.print_information();
        
        return Ok(mmio);
    }

    Err("Network card not found!")
}