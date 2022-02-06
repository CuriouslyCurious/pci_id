//! Main wrapper around all the vendors and classes that exist in the pci.ids file.
//!
//! This wrapper struct contains lists of the vendors and classes once they are read in
//! as well as the methods parsing for the pci.ids file.
//!
//! # Example
//! ```
//!
//! ```

// TODO: Replace manual parsing with either `nom` or `pest` if performance is better.

use std::num::ParseIntError;
use std::{io, path::Path};

use crate::class::{Class, SubClass, Interface};
use crate::vendor::{Vendor, Device, SubDevice};

/// Default path for the pci.ids file.
///
/// # Note
/// If this differs from your system you can supply your own path to the functions that require one.
pub const PATH_TO_PCI_IDS: &str = "/usr/share/hwdata/pci.ids";

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


#[cfg(test)]
mod tests {
    use crate::pci_ids::{PciIds, PATH_TO_PCI_IDS};
    use std::path::Path;
    /// Test the vendors part of the parsed result by picking an example and checking if it is ok
    #[test]
    fn test_vendors_list() {
        let mut pci_ids = PciIds::new();
        pci_ids
            .parse_vendors(Path::new(PATH_TO_PCI_IDS))
            .unwrap();
        let res = pci_ids.vendors().iter().find(|&v| {
            v.id() == 0x0e11
                && v.name() == "Compaq Computer Corporation"
                && v.devices()
                    .iter()
                    .find(|&d| {
                        d.id() == 0x0046
                            && d.name() == "Smart Array 64xx"
                            && d.subdevices()
                                .iter()
                                .find(|&s| {
                                    s.subvendor_id() == 0x0e11
                                        && s.subdevice_id() == 0x409d
                                        && s.name() == "Smart Array 6400 EM"
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
            .parse_classes(Path::new(PATH_TO_PCI_IDS))
            .unwrap();
        let res = pci_ids.classes().iter().find(|&c| {
            u8::from(c.class()) == 0x0c
                && c.class().to_string() == "Serial Bus Controller"
                && c.subclasses()
                    .iter()
                    .find(|&s| {
                        s.id() == 0x03
                            && s.name() == "USB controller"
                            && s.interfaces()
                                .iter()
                                .find(|&i| i.id() == 0xfe && i.name() == "USB Device")
                                .is_some()
                    })
                    .is_some()
        });
        println!("{:?}", res);
        assert!(res.is_some());
    }
}
