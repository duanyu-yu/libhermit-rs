use dtb::{Reader, StructItem};

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

pub fn get_cpu_frequency() {
    
}