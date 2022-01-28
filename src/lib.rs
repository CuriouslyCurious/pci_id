use std::{path::Path, io::Error};

pub const DEFAULT_PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vendor {
    pub id: u16,
    pub name: String,
    pub devices: Vec<Device>,
}

impl Vendor {
    fn new(id: u16, name: String) -> Self {
        Self { id, name, devices: Vec::new() }
    }

    fn set_devices(&mut self, devices: Vec<Device>) {
        self.devices = devices;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Device {
    pub id: u16,
    pub name: String,
    pub subdevices: Vec<SubDevice>,
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
    pub subvendor_id: u16,
    pub subdevice_id: u16,
    pub name: String,
}

impl SubDevice {
    fn new(subvendor_id: u16, subdevice_id: u16, name: String) -> Self {
        Self {
            subvendor_id,
            subdevice_id,
            name,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeviceClass {
    pub id: u8,
    pub name: String,
    pub subclasses: Vec<SubDeviceClass>,
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
    pub id: u8,
    pub name: String,
    pub interfaces: Vec<ProgrammingInterface>,
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
    pub id: u8,
    pub name: String,
}

impl ProgrammingInterface {
    fn new(id: u8, name: String) -> Self {
        Self { id, name }
    }
}

pub fn parse_pci_id_list<P>(path: P) -> Result<(Vec<Vendor>, Vec<DeviceClass>), Error>
where
    P: AsRef<Path>,
{
    let mut vendor_list: Vec<Vendor> = Vec::new();
    let mut class_list: Vec<DeviceClass> = Vec::new();
    //let mut pci_class_list = Vec::with_capacity(50000);
    let data = std::fs::read_to_string(path)?;

    let mut in_class_section = false;
    let mut vendor: Vendor;
    let mut device: Device;
    let mut class: DeviceClass;
    let mut subclass: SubDeviceClass;

    let mut devices = Vec::new();
    let mut subdevices = Vec::new();
    let mut subclasses = Vec::new();
    let mut interfaces = Vec::new();

    // TODO: Need to set the list of things from the last go when a new owner is made
    for line in data.lines() {
        // Skip comments and empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // Should be safe since we check if the line is empty
        let mut chars = line.chars();
        let char;
        unsafe {
            char = chars.next().unwrap_unchecked();
        }

        let (id, name) = line.split_once("  ").unwrap();
        let name = name.trim();

        // Line starts with a digit
        if char.is_digit(16) && char != 'C' && !in_class_section {
            let id = u16::from_str_radix(id.trim(), 16).unwrap();
            match vendor_list.last_mut() {
                Some(v) => v.set_devices(devices),
                None => (),
            }
            vendor = Vendor::new(id, name.to_owned());
            vendor_list.push(vendor);
            devices = Vec::new();
        } else if char == '\t' && !in_class_section {
            // One tab
            if chars.next().unwrap() != '\t' {
                let id = u16::from_str_radix(id.trim(), 16).unwrap();
                match devices.last_mut() {
                    Some(d) => d.set_subdevices(subdevices),
                    None => (),
                }
                device = Device::new(id, name.to_owned());
                devices.push(device);
                subdevices = Vec::new();
            // Two tabs
            } else {
                let (subvendor_id, subdevice_id) = id.split_once(" ").unwrap();
                let subvendor_id = u16::from_str_radix(subvendor_id.trim(), 16).unwrap();
                let subdevice_id = u16::from_str_radix(subdevice_id.trim(), 16).unwrap();
                let subdevice = SubDevice::new(subvendor_id, subdevice_id, name.to_owned());
                subdevices.push(subdevice);
            }

        // Line starts with a C or we have entered the device class section at the bottom of the file
        } else if char == 'C' {
            if !in_class_section {
                in_class_section = true;
            }

            let (_, id) = id.split_once(" ").unwrap();
            let id = u8::from_str_radix(id.trim(), 16).unwrap();
            match class_list.last_mut() {
                Some(c) => c.set_subclasses(subclasses),
                None => (),
            }
            class = DeviceClass::new(id, name.to_owned());
            class_list.push(class);
            subclasses = Vec::new();

        // At this point every line should start with a tab, so no need to check for that
        } else if in_class_section {
            let id = u8::from_str_radix(id.trim(), 16).unwrap();
            // One tab
            if chars.next().unwrap() != '\t' {
                match subclasses.last_mut() {
                    Some(s) => s.set_interfaces(interfaces),
                    None => (),
                }
                subclass = SubDeviceClass::new(id, name.to_owned());
                subclasses.push(subclass);
                interfaces = Vec::new();
            }
            // Two tabs
            else {
                let interface = ProgrammingInterface::new(id, name.to_owned());
                interfaces.push(interface);
            }
        }
    }
    // Add in the last ones
    match devices.last_mut() {
        Some(d) => d.set_subdevices(subdevices),
        None => (),
    };
    match vendor_list.last_mut() {
        Some(v) => v.set_devices(devices),
        None => (),
    };
    match subclasses.last_mut() {
        Some(s) => s.set_interfaces(interfaces),
        None => (),
    };
    match class_list.last_mut() {
        Some(c) => c.set_subclasses(subclasses),
        None => (),
    };

    Ok((vendor_list, class_list))
}

mod tests {
    /// Test the vendors part of the parsed result by picking an example and checking if it is ok
    #[test]
    fn test_vendors_list() {
        let (vendors, _) = crate::parse_pci_id_list(crate::DEFAULT_PATH_TO_PCI_IDS).unwrap();
        let res =  vendors
            .iter()
            .find(|&v| v.id == 0x0e11
                && v.name == "Compaq Computer Corporation"
                && v.devices
                   .iter()
                   .find(|&d| d.id == 0x0046
                        && d.name == "Smart Array 64xx"
                        && d.subdevices
                            .iter()
                            .find(|&s| s.subvendor_id == 0x0e11
                                && s.subdevice_id == 0x409d
                                && s.name == "Smart Array 6400 EM")
                            .is_some())
                    .is_some());
        assert!(!res.is_some());
    }

    /// Test the classes part of the parsed result by picking an example and checking if it is ok
    #[test]
    fn test_classes_list() {
        let (_, classes) = crate::parse_pci_id_list(crate::DEFAULT_PATH_TO_PCI_IDS).unwrap();
        let res = classes.iter().find(|&c| c.id == 0x0c && c.name == "Serial bus controller" && c.subclasses.iter().find(|&s| s.id == 0x03 && s.name == "USB controller" && s.interfaces.iter().find(|&i| i.id == 0xfe && i.name == "USB Device").is_some()).is_some());
        assert!(res.is_some());
    }
}
