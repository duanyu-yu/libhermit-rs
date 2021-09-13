use crate::arch::x86_64::mm::paging::{BasePageSize, PageSize, PageTableEntryFlags};
use crate::arch::x86_64::mm::{paging};
use crate::arch::x86_64::mm::{PhysAddr, VirtAddr};
use crate::arch::x86_64::mm::paging::virt_to_phys;
use core::{str};
use alloc::vec::Vec;

const MAGIC_VALUE: u32 = 0x74726976 as u32;

pub const MMIO_START: usize = 0x00000000c0000000 as usize;
pub const MMIO_END: usize = 0x00000000c0000fff as usize;

/// The Layout of MMIO Device 
#[repr(C, align(4))]
pub struct MMIO {
    pub magic_value: u32,
    pub version: u32,
    pub device_id: u32,
    pub vendor_id: u32,

    pub device_features: u32, 
    device_features_sel: u32, 
    _reserved0: [u32; 2],
    driver_features: u32, 
    driver_features_sel: u32, 

    guest_page_size: u32, // legacy only
    _reserved1: u32,

    queue_sel: u32,
    pub queue_num_max: u32,
    queue_num: u32,
    queue_align: u32, // legacy only
    pub queue_pfn: u32, // legacy only
    pub queue_ready: u32, // non-legacy only
    _reserved2: [u32; 2],
    queue_notify: u32,
    _reserved3: [u32; 3],

    pub interrupt_status: u32,
    interrupt_ack: u32, 
    _reserved4: [u32; 2],

    pub status: u32,
    _reserved5: [u32; 3],

    queue_desc_low: u32, // non-legacy only
    queue_desc_high: u32, // non-legacy only 
    _reserved6: [u32; 2],
    queue_driver_low: u32, // non-legacy only
    queue_driver_high: u32, // non-legacy only
    _reserved7: [u32; 2],
    queue_device_low: u32, // non-legacy only
    queue_device_high: u32, // non-legacy only
    _reserved8: [u32; 21],
    
    pub config_generation: u32, // non-legacy only
    pub config: [u32; 3],
}

impl MMIO {
    fn set_device_features_sel(&mut self, sel: u32) {
        info!("writing {:#X} to register DeviceFeaturesSel", sel);
        self.device_features_sel = sel;
    } 

    fn set_driver_features(&mut self, features: u32) {
        info!("writing {:#X} to register DriverFeatures", features);
        self.driver_features = features;
    }

    fn set_driver_features_sel(&mut self, sel: u32) {
        info!("writing {:#X} to register DriverFeaturesSel", sel);
        self.driver_features_sel = sel;
    }

    fn set_guest_page_size(&mut self, pagesize: u32) {
        info!("writing {:#X} to register GuestPageSize", pagesize);
        self.guest_page_size = pagesize;
    }

    fn set_queue_sel(&mut self, sel: u32) {
        info!("writing {:#X} to register QueueSel", sel);
        self.queue_sel = sel;
    }

    fn set_queue_num(&mut self, queue_num: u32) {
        info!("writing {:#X} to register QueueNum", queue_num);
        self.queue_num = queue_num;
    }

    fn set_queue_align(&mut self, queue_align: u32) {
        info!("writing {:#X} to register QueueAlign", queue_align);
        self.queue_align = queue_align;
    }

    fn set_queue_pfn(&mut self, queue_pfn: u32) {
        info!("writing {:#X} to register QueuePFN", queue_pfn);
        self.queue_pfn = queue_pfn;
    }

    fn set_queue_ready(&mut self, ready: u32) {
        self.queue_ready = ready;
    }

    fn set_queue_notify(&mut self, queue_notify: u32) {
        info!("writing {:#X} to register QueueNotify", queue_notify);
        self.queue_notify = queue_notify;
    }

    fn set_interrupt_ack(&mut self, interrupt_ack: u32) {
        info!("writing {:#X} to register InterruptACK", interrupt_ack);
        self.interrupt_ack = interrupt_ack;
    }

    fn set_status(&mut self, status: u32) {
        info!("writing {:#X} to register Status", status);
        self.status = status;
    }

    fn set_config(&mut self, config: [u32; 3]) {
        info!("writing {:#X?} to register Config", config);
        self.config = config;
    }

    fn print_information(&self) {
        infoheader!(" MMIO INFORMATION ");

        infoentry!("Device version", "{:#X}", self.version);
        infoentry!("Device ID", "{:#X}", self.device_id);
        infoentry!("Vendor ID", "{:#X}", self.vendor_id);

        infoentry!("Device Features", "{:#X}", self.device_features);
        infoentry!("Device Features selection", "{:#X}", self.device_features_sel);
        infoentry!("Driver Features", "{:#X}", self.driver_features);
        infoentry!("Driver Features selection", "{:#X}", self.driver_features_sel);

        infoentry!("Guest Page Size", "{:#X}", self.guest_page_size);

        infoentry!("Virtual queue index", "{:#X}", self.queue_sel);
        infoentry!("Maximum queue size", "{:#X}", self.queue_num_max);
        infoentry!("Virtual queue size", "{:#X}", self.queue_num);
        infoentry!("Queue alignment", "{:#X}", self.queue_align);
        infoentry!("Physical page number", "{:#X}", self.queue_pfn);
        infoentry!("Queue Ready", "{:#X}", self.queue_ready);
        infoentry!("Queue notifier", "{:#X}", self.queue_notify);

        infoentry!("Interrupt status", "{:#X}", self.interrupt_status);
        infoentry!("Interrupt acknowledge", "{:#X}", self.interrupt_ack);

        infoentry!("Device status", "{:#X}", self.status);

        infoentry!("Queue Desc Low", "{:#X}", self.queue_desc_low);
        infoentry!("Queue Desc High", "{:#X}", self.queue_desc_high);
        infoentry!("Queue Driver Low", "{:#X}", self.queue_driver_low);
        infoentry!("Queue Driver High", "{:#X}", self.queue_driver_high);
        infoentry!("Queue Device Low", "{:#X}", self.queue_device_low);
        infoentry!("Queue Device High", "{:#X}", self.queue_device_high);

        infoentry!("Config atomicity value", "{:#X}", self.config_generation);
        infoentry!("Configuration space", "{:#X?}", self.config);

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

        let magic = mmio.magic_value;

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

        let magic = mmio.magic_value;

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
        let id = mmio.device_id;

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

pub fn test_write(val: u32) {
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
        let mmio = unsafe { &mut *(current_address as *mut MMIO) };

        let magic = mmio.magic_value;

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
        let id = mmio.device_id;

        if id != 0x1 {
            info!("It's not a network card at {:#X}", current_address);
            continue;
        }

        info!("Found network card at {:#X}", current_address);

        mmio.print_information();

        mmio.set_device_features_sel(val);
        mmio.set_driver_features(val);
        mmio.set_driver_features_sel(val);
        mmio.set_guest_page_size(val);
        
        // test write the queue_pfn register
        mmio.set_queue_sel(val);
        mmio.set_queue_num(val);
        mmio.set_queue_align(val);
        mmio.set_queue_pfn(val);
        mmio.set_queue_notify(val);

        mmio.set_interrupt_ack(val);
        mmio.set_status(val);

        mmio.print_information();
    }
}