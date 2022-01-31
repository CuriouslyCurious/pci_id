//! ```
//! use std::path::Path;
//! use pci_id::{PciIds, DEFAULT_PATH_TO_PCI_IDS, Device};
//!
//! let pci_ids = PciIds::parse_pci_id_list(Path::new(DEFAULT_PATH_TO_PCI_IDS)).unwrap();
//! let amd_devices = pci_ids.vendors().iter().find(|v| v.name() == "Advanced Micro Devices, Inc. [AMD/ATI]").unwrap();
//! let navi_10: Vec<&Device> = amd_devices.devices().iter().filter(|d| d.name() == "Navi 10 [Radeon RX 5600 OEM/5600 XT / 5700/5700 XT]").collect();
//! for device in navi_10 {
//!     for subdevice in device.subdevices() {
//!        println!("{}", subdevice.name())
//!     }
//! }
//! ```
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod device_class;

use std::num::ParseIntError;
use std::{io, path::Path};

use crate::device_class::DeviceClass;

/// Default path for the pci.ids file.
///
/// # Note
/// If this differs from your system you can supply your own path to the functions that require one.
pub const DEFAULT_PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

/// Wrapper struct around the list of PCI vendors and classes that exist in the pci.ids file.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PciIds {
    vendors: Vec<Vendor>,
    classes: Vec<Class>,
}

impl PciIds {
    /// Create a new PciIds struct with initially empty lists.
    pub fn new() -> Self {
        Self {
            vendors: Vec::new(),
            classes: Vec::new(),
        }
    }

    /// Returns a reference to the list of vendors.
    pub fn vendors(&self) -> &Vec<Vendor> {
        &self.vendors
    }

    /// Returns a reference to the list of classes.
    pub fn classes(&self) -> &Vec<Class> {
        &self.classes
    }

    /// Given the path to a valid pci.ids repository file will only parse the [Vendor]s into `self`,
    /// skipping the [Class]es.
    pub fn parse_vendors(&mut self, path: &Path) -> Result<(), io::Error> {
        let data = std::fs::read_to_string(path)?;
        self.parse_lines(data, false, true).unwrap();
        Ok(())
    }

    /// Given the path to a valid pci.ids repository file will only parse the [Class]es into `self`,
    /// skipping the [Vendor]s.
    pub fn parse_classes(&mut self, path: &Path) -> Result<(), io::Error> {
        let data = std::fs::read_to_string(path)?;
        self.parse_lines(data, true, false).unwrap();
        Ok(())
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
        let mut class: Class;
        let mut subclass: SubClass;

        let mut devices = Vec::new();
        let mut subdevices = Vec::new();
        let mut subclasses = Vec::new();
        let mut interfaces = Vec::new();

        // TODO: Split up list mutation into an inlined function
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
                class = Class::new(id);
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
                    subclass = SubClass::new(id, name.to_owned());
                    subclasses.push(subclass);
                    interfaces = Vec::new();
                }
                // Two tabs
                else {
                    let interface = Interface::new(id, name.to_owned());
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

    /// Try to parse the given pci.ids file to a [PciIds] instance.
    ///
    /// The entire file is first read into a [String]. Parsing is then done line by line of the
    /// string to the various data structures.
    ///
    /// # Errors
    /// Reading in the file can fail for all the usual IO reasons, check [std::io::ErrorKind].
    pub fn parse_pci_id_list(path: &Path) -> Result<Self, io::Error> {
        let mut pci_ids = Self::new();

        let data = std::fs::read_to_string(path)?;
        pci_ids.parse_lines(data, false, false).unwrap();

        Ok(pci_ids)
    }
}

impl Default for PciIds {
    fn default() -> Self {
        Self::new()
    }
}

/// A hardware vendor.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Vendor {
    /// Vendor id
    id: u16,
    name: String,
    devices: Vec<Device>,
}

impl Vendor {
    /// Create a new vendor with a given id and name.
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            devices: Vec::new(),
        }
    }

    /// Unique vendor id.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Name of the vendor.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// List of devices the vendor has been ascribed.
    pub fn devices(&self) -> &Vec<Device> {
        &self.devices
    }

    /// Set the devices to a given list of devices.
    fn set_devices(&mut self, devices: Vec<Device>) {
        self.devices = devices;
    }
}

/// A PCI device.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Device {
    id: u16,
    name: String,
    subdevices: Vec<SubDevice>,
}

impl Device {
    /// Create a new device with a given id and name.
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            subdevices: Vec::new(),
        }
    }

    /// Identifier of the device.
    ///
    /// # Note
    /// Useful for matching against PCI devices retrieved from `/sys/bus/pci/devices`.
    pub fn id(&self) -> u16 {
        self.id
    }

    /// Name of the device.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// List of subdevices the device can be.
    pub fn subdevices(&self) -> &Vec<SubDevice> {
        &self.subdevices
    }

    /// Set the subdevices to a given list of subdevices.
    fn set_subdevices(&mut self, subdevices: Vec<SubDevice>) {
        self.subdevices = subdevices;
    }
}

/// A subset of a PCI device.
///
/// Contains the name and id for a specific version of a device as well as the identifier for the
/// OEM/subvendor/manufacturer that supplies the device.
///
/// # Example
/// Look up all the cards that have Sapphire as a manufacturer. id = 0x1da2
/// ```
///
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubDevice {
    subvendor_id: u16,
    subdevice_id: u16,
    name: String,
}

impl SubDevice {
    /// Create a new subdevice from a given subvendor id, subdevice id and name.
    pub fn new(subvendor_id: u16, subdevice_id: u16, name: String) -> Self {
        Self {
            subvendor_id,
            subdevice_id,
            name,
        }
    }

    /// Identifier of the OEM/subvendor.
    ///
    /// # Note
    /// The same manufacturers have used a lot of different ids over the years thus making it unreliable to only match against one subvendor id.
    pub fn subvendor_id(&self) -> u16 {
        self.subvendor_id
    }

    /// Identifier of the actual device.
    /// # Note
    /// Useful for matching against PCI devices retrieved from `/sys/bus/pci/devices`.
    pub fn subdevice_id(&self) -> u16 {
        self.subdevice_id
    }

    /// Name of the device.
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// A category/class that a PCI device can belong to, along with eventual subclasses for more
/// specificity.
///
/// # Example
/// Lookup all the Floppy disk controllers:
/// ```
///
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Class {
    class: DeviceClass,
    subclasses: Vec<SubClass>,
}

impl Class {
    /// Create a new class struct from a given id.
    /// # Panics
    /// Will panic upon receiving an invalid id that is not (yet) defined in [DeviceClass].
    pub fn new(id: u8) -> Self {
        Self {
            class: DeviceClass::try_from(id).unwrap(),
            subclasses: Vec::new(),
        }
    }

    /// The [DeviceClass] a device can belong to.
    pub fn class(&self) -> DeviceClass {
        self.class
    }

    /// A list of [SubClass]es of the class.
    pub fn subclasses(&self) -> &Vec<SubClass> {
        &self.subclasses
    }

    /// Set the subclasses to a given list of subclasses.
    fn set_subclasses(&mut self, subclasses: Vec<SubClass>) {
        self.subclasses = subclasses;
    }
}

/// A subclass/subcategory of a type of PCI device.
///
/// For example a 'network controller' can be everything from a fabric controller, an ethernet
/// controller to an ATM controller.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SubClass {
    id: u8,
    name: String,
    interfaces: Vec<Interface>,
}

impl SubClass {
    /// Create a new subclass from a given id and name.
    pub fn new(id: u8, name: String) -> Self {
        Self {
            id,
            name,
            interfaces: Vec::new(),
        }
    }

    /// Identifier of the subclass.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Name of the subclass.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// List of potential programming [Interface]s of a subclass.
    pub fn interfaces(&self) -> &Vec<Interface> {
        &self.interfaces
    }

    /// Set the programming interfaces to a given list of interfaces.
    fn set_interfaces(&mut self, interfaces: Vec<Interface>) {
        self.interfaces = interfaces;
    }
}

/// A programming interface of a subclass, so yet a lower level of categorisation of a particular
/// PCI device type.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Interface {
    id: u8,
    name: String,
}

impl Interface {
    /// Create a new programming interface struct from a given id and name.
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name }
    }

    /// Identifier of the programming interface.
    pub fn id(&self) -> u8 {
        self.id
    }

    /// Name of the programming interface.
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
            u8::from(c.class) == 0x0c
                && c.class.to_string() == "Serial Bus Controller"
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
        println!("{:?}", res);
        assert!(res.is_some());
    }
}
