use std::path::Path;
use std::collections::HashSet;

pub const PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

pub trait PCIEntry {}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vendor {
    id: u16,
    name: String,
    devices: Vec<Device>,
}

impl Vendor {
    fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            devices: Vec::new(),
        }
    }

    fn set_devices(&mut self, devices: Vec<Device>) {
        self.devices = devices;
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Device {
    id: u16,
    name: String,
    subdevices: Vec<SubDevice>,
}

impl Device {
    fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            subdevices: Vec::new(),
        }
    }

    fn set_subdevices(&mut self, subdevices: Vec<SubDevice>) {
        self.subdevices = subdevices;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubDevice {
    subvendor: u16,
    id: u16,
    name: String,
}

impl SubDevice {
    fn new(subvendor: u16, id: u16, name: String) -> Self {
        Self {
            subvendor,
            id,
            name,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeviceClass {
    id: u8,
    name: String,
    subclasses: Vec<SubDeviceClass>,
}

impl DeviceClass {
    fn new(id: u8, name: String) -> Self {
        Self {
            id,
            name,
            subclasses: Vec::new(),
        }
    }

    fn set_subclasses(&mut self, subclasses: Vec<SubDeviceClass>) {
        self.subclasses = subclasses;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubDeviceClass {
    id: u8,
    name: String,
    interfaces: Vec<ProgrammingInterface>,
}

impl SubDeviceClass {
    fn new(id: u8, name: String) -> Self {
        Self {
            id,
            name,
            interfaces: Vec::new(),
        }
    }

    fn set_interfaces(&mut self, interfaces: Vec<ProgrammingInterface>) {
        self.interfaces = interfaces;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProgrammingInterface {
    interface: u8,
    name: String,
}

impl ProgrammingInterface {
    fn new(interface: u8, name: String) -> Self {
        Self {
            interface,
            name,
        }
    }
}

pub fn parse_pci_id_list<P>(path: P) -> (Vec<Vendor>, Vec<DeviceClass>) where P: AsRef<Path> {
    let mut vendor_list = Vec::new();
    let mut class_list = Vec::new();
    let data = std::fs::read_to_string(path).unwrap();

    let mut in_class_section = false;
    let mut vendor: Vendor;
    let mut device: Device;
    let mut class: DeviceClass;
    let mut subclass: SubDeviceClass;
    let mut devices = Vec::new();
    let mut subdevices = Vec::new();
    let mut subclasses = Vec::new();
    let mut interfaces = Vec::new();

    for line in data.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("C") || in_class_section {
            in_class_section = true;
            let mut s = line.split("  ");
            let id = s.next().unwrap();
            let name = s.next().unwrap();

            if !id.starts_with("\t") {
                let (_, id) = id.split_once(" ").unwrap();
                let id = u8::from_str_radix(id, 16).unwrap();
                class = DeviceClass::new(id, name.to_owned());
                class.set_subclasses(subclasses);
                class_list.push(class);
                subclasses = Vec::new();
            } else {
                if id.starts_with("\t\t") {
                    let id = u8::from_str_radix(id.trim(), 16).unwrap();
                    let interface = ProgrammingInterface::new(id, name.to_owned());
                    interfaces.push(interface);
                } else if id.starts_with("\t") {
                    let id = id.trim();
                    let id = u8::from_str_radix(id, 16).unwrap();
                    subclass = SubDeviceClass::new(id, name.to_owned());
                    subclass.set_interfaces(interfaces);
                    subclasses.push(subclass);
                    interfaces = Vec::new();
                }
            }

        } else {
            let mut s = line.split("  ");
            let id = s.next().unwrap();
            let name = s.next().unwrap();

            if !id.starts_with("\t") {
                let id = u16::from_str_radix(id, 16).unwrap();
                vendor = Vendor::new(id, name.to_owned());
                vendor.set_devices(devices);
                vendor_list.push(vendor);
                devices = Vec::new();
            } else {
                if id.starts_with("\t\t") {
                    let (id, subvendor_id) = id.split_once(" ").unwrap();
                    let subvendor_id = u16::from_str_radix(subvendor_id.trim(), 16).unwrap();
                    let id = u16::from_str_radix(id.trim(), 16).unwrap();
                    let subdevice = SubDevice::new(subvendor_id, id, name.to_owned());
                    subdevices.push(subdevice);
                } else if id.starts_with("\t") {
                    let id = id.trim();
                    let id = u16::from_str_radix(id, 16).unwrap();
                    device = Device::new(id, name.to_owned());
                    device.set_subdevices(subdevices);
                    devices.push(device);
                    subdevices = Vec::new();
                }
            }
        }

    }
    (vendor_list, class_list)
}

pub fn parse_pci_id_hash<P>(path: P) -> (HashSet<Vendor>, HashSet<DeviceClass>) where P: AsRef<Path> {
    let mut vendor_set = HashSet::new();
    let mut class_set = HashSet::new();
    let data = std::fs::read_to_string(path).unwrap();

    let mut in_class_section = false;
    let mut vendor: Vendor;
    let mut device: Device;
    let mut class: DeviceClass;
    let mut subclass: SubDeviceClass;
    let mut devices = Vec::new();
    let mut subdevices = Vec::new();
    let mut subclasses = Vec::new();
    let mut interfaces = Vec::new();

    for line in data.lines() {
        if line.starts_with("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("C") || in_class_section {
            in_class_section = true;
            let mut s = line.split("  ");
            let id = s.next().unwrap();
            let name = s.next().unwrap();

            if !id.starts_with("\t") {
                let (_, id) = id.split_once(" ").unwrap();
                let id = u8::from_str_radix(id, 16).unwrap();
                class = DeviceClass::new(id, name.to_owned());
                class.set_subclasses(subclasses);
                class_set.insert(class);
                subclasses = Vec::new();
            } else {
                if id.starts_with("\t\t") {
                    let id = u8::from_str_radix(id.trim(), 16).unwrap();
                    let interface = ProgrammingInterface::new(id, name.to_owned());
                    interfaces.push(interface);
                } else if id.starts_with("\t") {
                    let id = id.trim();
                    let id = u8::from_str_radix(id, 16).unwrap();
                    subclass = SubDeviceClass::new(id, name.to_owned());
                    subclass.set_interfaces(interfaces);
                    subclasses.push(subclass);
                    interfaces = Vec::new();
                }
            }

        } else {
            let mut s = line.split("  ");
            let id = s.next().unwrap();
            let name = s.next().unwrap();

            if !id.starts_with("\t") {
                let id = u16::from_str_radix(id, 16).unwrap();
                vendor = Vendor::new(id, name.to_owned());
                vendor.set_devices(devices);
                vendor_set.insert(vendor);
                devices = Vec::new();
            } else {
                if id.starts_with("\t\t") {
                    let (id, subvendor_id) = id.split_once(" ").unwrap();
                    let subvendor_id = u16::from_str_radix(subvendor_id.trim(), 16).unwrap();
                    let id = u16::from_str_radix(id.trim(), 16).unwrap();
                    let subdevice = SubDevice::new(subvendor_id, id, name.to_owned());
                    subdevices.push(subdevice);
                } else if id.starts_with("\t") {
                    let id = id.trim();
                    let id = u16::from_str_radix(id, 16).unwrap();
                    device = Device::new(id, name.to_owned());
                    device.set_subdevices(subdevices);
                    devices.push(device);
                    subdevices = Vec::new();
                }
            }
        }

    }
    (vendor_set, class_set)
}

mod tests {
    use crate::*;

    #[test]
    fn test_parse_list() {
        parse_pci_id_list(PATH_TO_PCI_IDS);
    }

    #[test]
    fn test_parse_hash() {
        parse_pci_id_hash(PATH_TO_PCI_IDS);
    }
}
