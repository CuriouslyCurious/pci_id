//! A vendor in the pci.ids file can contain a bunch of devices who in turn can have its own subdevices
//! made.
//!
//! # Example
//! ```
//!
//! ```

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
    pub(crate) fn set_devices(&mut self, devices: Vec<Device>) {
        self.devices = devices;
    }
}

/// A PCI device.
///
/// # Example
/// ```
///
/// ```
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
    pub(crate) fn set_subdevices(&mut self, subdevices: Vec<SubDevice>) {
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


