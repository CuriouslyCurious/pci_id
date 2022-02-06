//! A PCI device can belong to a [DeviceClass] and its subsequent [SubClass]es and programming [Interface]s.
//!
//! # Example
//! ```
//!
//! ```

use crate::device_class::DeviceClass;

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
    pub(crate) fn set_subclasses(&mut self, subclasses: Vec<SubClass>) {
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
    pub(crate) fn set_interfaces(&mut self, interfaces: Vec<Interface>) {
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

