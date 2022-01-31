use std::num::ParseIntError;
use std::{io, path::Path};

pub const DEFAULT_PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PciIds {
    vendors: Vec<Vendor>,
    classes: Vec<DeviceClass>,
}

impl PciIds {
    pub fn new() -> Self {
        Self {
            vendors: Vec::new(),
            classes: Vec::new(),
        }
    }

    pub fn vendors(&self) -> &Vec<Vendor> {
        &self.vendors
    }

    pub fn classes(&self) -> &Vec<DeviceClass> {
        &self.classes
    }

    pub fn parse_vendors(&mut self, path: &Path) -> Result<(), io::Error> {
        let data = std::fs::read_to_string(path)?;
        self.parse_lines(data, false, true).unwrap();
        Ok(())
    }

    pub fn parse_classes(&mut self, path: &Path) -> Result<(), io::Error> {
        let data = std::fs::read_to_string(path)?;
        self.parse_lines(data, true, false).unwrap();
        Ok(())
    }

    pub fn get_vendor(_: &str) -> Result<Vendor, io::Error> {
        todo!()
    }

    pub fn get_device(_: &str) -> Result<Device, io::Error> {
        todo!()
    }

    pub fn get_subdevice(_: &str) -> Result<SubDevice, io::Error> {
        todo!()
    }

    pub fn get_class(_: &str) -> Result<DeviceClass, io::Error> {
        todo!()
    }

    pub fn get_subclass(_: &str) -> Result<SubDeviceClass, io::Error> {
        todo!()
    }

    pub fn get_programming_interface(_: &str) -> Result<ProgrammingInterface, io::Error> {
        todo!()
    }

    #[inline(always)]
    fn parse_lines(
        &mut self,
        data: String,
        skip_vendors: bool,
        skip_classes: bool,
    ) -> Result<(), ParseIntError> {
        let mut in_class_section = false;
        let mut vendor: Vendor;
        let mut device: Device;
        let mut class: DeviceClass;
        let mut subclass: SubDeviceClass;

        let mut devices = Vec::new();
        let mut subdevices = Vec::new();
        let mut subclasses = Vec::new();
        let mut interfaces = Vec::new();

        // TODO:Split up list mutation into an inlined function
        for line in data.lines() {
            // Skip comments and empty lines
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            // Should be safe since we check if the line is empty thus the next char is guaranteed
            // to be there
            let mut chars = line.chars();
            let char = chars.next().unwrap();

            let (id, name) = line.split_once("  ").unwrap();
            let name = name.trim();

            // Line starts with a digit
            if !skip_vendors && char.is_digit(16) && char != 'C' && !in_class_section {
                let id = u16::from_str_radix(id.trim(), 16)?;
                if let Some(v) = self.vendors.last_mut() {
                    v.set_devices(devices);
                }
                vendor = Vendor::new(id, name.to_owned());
                self.vendors.push(vendor);
                devices = Vec::new();
            } else if !skip_vendors && char == '\t' && !in_class_section {
                // One tab
                if chars.next().unwrap() != '\t' {
                    let id = u16::from_str_radix(id.trim(), 16)?;
                    if let Some(d) = devices.last_mut() {
                        d.set_subdevices(subdevices);
                    }
                    device = Device::new(id, name.to_owned());
                    devices.push(device);
                    subdevices = Vec::new();
                // Two tabs
                } else {
                    let (subvendor_id, subdevice_id) = id.split_once(" ").unwrap();
                    let subvendor_id = u16::from_str_radix(subvendor_id.trim(), 16)?;
                    let subdevice_id = u16::from_str_radix(subdevice_id.trim(), 16)?;
                    let subdevice = SubDevice::new(subvendor_id, subdevice_id, name.to_owned());
                    subdevices.push(subdevice);
                }

            // Line starts with a C meaning we are in the class section
            } else if char == 'C' {
                if skip_classes {
                    break;
                }

                if !in_class_section {
                    in_class_section = true;
                }

                let (_, id) = id.split_once(" ").unwrap();
                let id = u8::from_str_radix(id.trim(), 16)?;
                if let Some(c) = self.classes.last_mut() {
                    c.set_subclasses(subclasses);
                }
                class = DeviceClass::new(id, name.to_owned());
                self.classes.push(class);
                subclasses = Vec::new();

            // At this point every line should start with a tab, so no need to check for that
            } else if !skip_classes && in_class_section {
                let id = u8::from_str_radix(id.trim(), 16)?;
                // One tab
                if chars.next().unwrap() != '\t' {
                    if let Some(s) = subclasses.last_mut() {
                        s.set_interfaces(interfaces);
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
        if let Some(d) = devices.last_mut() {
            d.set_subdevices(subdevices);
        };
        if let Some(v) = self.vendors.last_mut() {
            v.set_devices(devices);
        };
        if let Some(s) = subclasses.last_mut() {
            s.set_interfaces(interfaces);
        };
        if let Some(c) = self.classes.last_mut() {
            c.set_subclasses(subclasses);
        };
        Ok(())
    }

    pub fn parse_pci_id_list(&mut self, path: &Path) -> Result<(), io::Error> {
        let data = std::fs::read_to_string(path)?;
        self.parse_lines(data, false, false).unwrap();

        Ok(())
    }
}

impl Default for PciIds {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vendor {
    id: u16,
    name: String,
    devices: Vec<Device>,
}

impl Vendor {
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            devices: Vec::new(),
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn devices(&self) -> &Vec<Device> {
        &self.devices
    }

    pub fn set_devices(&mut self, devices: Vec<Device>) {
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
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            subdevices: Vec::new(),
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subdevices(&self) -> &Vec<SubDevice> {
        &self.subdevices
    }

    fn set_subdevices(&mut self, subdevices: Vec<SubDevice>) {
        self.subdevices = subdevices;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubDevice {
    subvendor_id: u16,
    subdevice_id: u16,
    name: String,
}

impl SubDevice {
    pub fn new(subvendor_id: u16, subdevice_id: u16, name: String) -> Self {
        Self {
            subvendor_id,
            subdevice_id,
            name,
        }
    }

    pub fn subvendor_id(&self) -> u16 {
        self.subvendor_id
    }

    pub fn subdevice_id(&self) -> u16 {
        self.subdevice_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct DeviceClass {
    id: u8,
    name: String,
    subclasses: Vec<SubDeviceClass>,
}

impl DeviceClass {
    pub fn new(id: u8, name: String) -> Self {
        Self {
            id,
            name,
            subclasses: Vec::new(),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn subclasses(&self) -> &Vec<SubDeviceClass> {
        &self.subclasses
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
    pub fn new(id: u8, name: String) -> Self {
        Self {
            id,
            name,
            interfaces: Vec::new(),
        }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn interfaces(&self) -> &Vec<ProgrammingInterface> {
        &self.interfaces
    }

    fn set_interfaces(&mut self, interfaces: Vec<ProgrammingInterface>) {
        self.interfaces = interfaces;
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProgrammingInterface {
    id: u8,
    name: String,
}

impl ProgrammingInterface {
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::{PciIds, DEFAULT_PATH_TO_PCI_IDS};
    use std::path::Path;
    /// Test the vendors part of the parsed result by picking an example and checking if it is ok
    #[test]
    fn test_vendors_list() {
        let mut pci_ids = PciIds::new();
        pci_ids
            .parse_vendors(Path::new(DEFAULT_PATH_TO_PCI_IDS))
            .unwrap();
        let res = pci_ids.vendors().iter().find(|&v| {
            v.id == 0x0e11
                && v.name == "Compaq Computer Corporation"
                && v.devices
                    .iter()
                    .find(|&d| {
                        d.id == 0x0046
                            && d.name == "Smart Array 64xx"
                            && d.subdevices
                                .iter()
                                .find(|&s| {
                                    s.subvendor_id == 0x0e11
                                        && s.subdevice_id == 0x409d
                                        && s.name == "Smart Array 6400 EM"
                                })
                                .is_some()
                    })
                    .is_some()
        });
        assert!(res.is_some());
    }

    /// Test the classes part of the parsed result by picking an example and checking if it is ok
    #[test]
    fn test_classes_list() {
        let mut pci_ids = PciIds::new();
        pci_ids
            .parse_classes(Path::new(DEFAULT_PATH_TO_PCI_IDS))
            .unwrap();
        let res = pci_ids.classes().iter().find(|&c| {
            c.id == 0x0c
                && c.name == "Serial bus controller"
                && c.subclasses
                    .iter()
                    .find(|&s| {
                        s.id == 0x03
                            && s.name == "USB controller"
                            && s.interfaces
                                .iter()
                                .find(|&i| i.id == 0xfe && i.name == "USB Device")
                                .is_some()
                    })
                    .is_some()
        });
        assert!(res.is_some());
    }
}
