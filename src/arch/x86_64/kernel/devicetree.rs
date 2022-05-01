use dtb::{Reader, StructItem};
use core::fmt;
use core::convert::TryInto;
pub struct MemoryRegion {
    base_address: u64,
    length: u64,
}

impl fmt::Debug for MemoryRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            write!(
                f,
                "MemoryRegion {{ base_address: {}, length: {} }}",
                self.base_address, self.length
            )
        }
    }
}

impl Default for MemoryRegion {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl MemoryRegion {
    pub fn new(base_addr: u64, length: u64) -> Self {
        Self { 
            base_address: base_addr,
            length: length }
    }

    pub fn base_address(&self) -> u64 {
        self.base_address
    }

    pub fn length(&self) -> u64 {
        self.length
    }
}

pub fn print_information(reader: &Reader<'_>) {
    info!("DEVICE TREE:");

    let mut indent = 0;
    for entry in reader.struct_items() {
        match entry {
            StructItem::BeginNode { name } => {
                info!("{:indent$}{} {{", "", name, indent = indent);
                indent += 2;
            }
            StructItem::EndNode => {
                indent -= 2;
                info!("{:indent$}}}", "", indent = indent);
            }
            StructItem::Property { name, value } => {
                info!("{:indent$}{}: {:?}", "", name, value, indent = indent);
            }
        }
    }
    info! ("End DEVICE TREE");
} 

pub fn get_cpu_count(dtb_addr: usize) -> u32 {
    let reader = unsafe {
        Reader::read_from_address(dtb_addr).unwrap()
    };

    let mut count = 0;
    for entry in reader.struct_items() {
        if entry.is_begin_node() {
            if entry.node_name() == Ok("cpu") {
                count += 1;
            }
        }
    }

    count
}

pub fn get_memory_regions(dtb_addr: usize) -> Option<MemoryRegion> {
    let reader = unsafe {
        Reader::read_from_address(dtb_addr).unwrap()
    };

    let mut reg: &[u8] = &[];
    for entry in reader.struct_items() {
        if entry.node_name() == Ok("memory") {
            if entry.is_property() && entry.name() == Ok("reg") {
                reg = entry.value().unwrap();
            }
        }
    }

    if reg.is_empty() {
        return None;
    }

    let reg_length = u64::from_be_bytes(reg.take(4..).unwrap().try_into().unwrap());

    let base_addr = u64::from_be_bytes(reg.try_into().unwrap());

    Some(MemoryRegion {
        base_address: base_addr,
        length: reg_length,
    })
}
