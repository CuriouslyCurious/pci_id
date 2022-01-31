//! The PCI ID repository specifies a bunch of classes and subclasses a device can be a part of.
//! This module translates the hexadecimal value of a class or subclass to a more useful enum.

use std::fmt;

/// The different classes a device can be apart of, as defined by: [https://pci-ids.ucw.cz/read/PD/](https://pci-ids.ucw.cz/read/PD/)
// TODO: Make the subdevice classes and programming interfaces into their own enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceClass {
    Unclassified,                      // ID: 00
    MassStorageController,             // ID: 01
    NetworkController,                 // ID: 02
    DisplayController,                 // ID: 03
    MultimediaController,              // ID: 04
    MemoryController,                  // ID: 05
    Bridge,                            // ID: 06
    CommunicationController,           // ID: 07
    GenericSystemPeripheral,           // ID: 08
    InputDeviceController,             // ID: 09
    DockingStation,                    // ID: 0a
    Processor,                         // ID: 0b
    SerialBusController,               // ID: 0c
    WirelessController,                // ID: 0d
    IntelligentController,             // ID: 0e
    SatelliteCommunicationsController, // ID: 0f
    EncryptionController,              // ID: 10
    SignalProcessingController,        // ID: 11
    ProcessingAccelerator,             // ID: 12
    NonEssentialInstrumentation,       // ID: 13
    Coprocessor,                       // ID: 40
    Unassigned,                        // ID: ff
}

impl TryFrom<u8> for DeviceClass {
    type Error = &'static str;
    /// Retrieve the device class with the given byte. Will panic if the byte value does
    /// not have a corresponding device class defined.
    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            0x00 => Ok(Self::Unclassified),
            0x01 => Ok(Self::MassStorageController),
            0x02 => Ok(Self::NetworkController),
            0x03 => Ok(Self::DisplayController),
            0x04 => Ok(Self::MultimediaController),
            0x05 => Ok(Self::MemoryController),
            0x06 => Ok(Self::Bridge),
            0x07 => Ok(Self::CommunicationController),
            0x08 => Ok(Self::GenericSystemPeripheral),
            0x09 => Ok(Self::InputDeviceController),
            0x0a => Ok(Self::DockingStation),
            0x0b => Ok(Self::Processor),
            0x0c => Ok(Self::SerialBusController),
            0x0d => Ok(Self::WirelessController),
            0x0e => Ok(Self::IntelligentController),
            0x0f => Ok(Self::SatelliteCommunicationsController),
            0x10 => Ok(Self::EncryptionController),
            0x11 => Ok(Self::SignalProcessingController),
            0x12 => Ok(Self::ProcessingAccelerator),
            0x13 => Ok(Self::NonEssentialInstrumentation),
            0x40 => Ok(Self::Coprocessor),
            0xff => Ok(Self::Unassigned),
            _ => Err("Invalid DeviceClass byte")
        }
    }
}

impl From<DeviceClass> for u8 {
    fn from(class: DeviceClass) -> u8 {
        match class {
            DeviceClass::Unclassified                     => 0x00,
            DeviceClass::MassStorageController            => 0x01,
            DeviceClass::NetworkController                => 0x02,
            DeviceClass::DisplayController                => 0x03,
            DeviceClass::MultimediaController             => 0x04,
            DeviceClass::MemoryController                 => 0x05,
            DeviceClass::Bridge                           => 0x06,
            DeviceClass::CommunicationController          => 0x07,
            DeviceClass::GenericSystemPeripheral          => 0x08,
            DeviceClass::InputDeviceController            => 0x09,
            DeviceClass::DockingStation                   => 0x0a,
            DeviceClass::Processor                        => 0x0b,
            DeviceClass::SerialBusController              => 0x0c,
            DeviceClass::WirelessController               => 0x0d,
            DeviceClass::IntelligentController            => 0x0e,
            DeviceClass::SatelliteCommunicationsController=> 0x0f,
            DeviceClass::EncryptionController             => 0x10,
            DeviceClass::SignalProcessingController       => 0x11,
            DeviceClass::ProcessingAccelerator            => 0x12,
            DeviceClass::NonEssentialInstrumentation      => 0x13,
            DeviceClass::Coprocessor                      => 0x40,
            DeviceClass::Unassigned                       => 0xff,
        }
    }
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeviceClass::Unclassified => write!(f, "Unclassified"),
            DeviceClass::MassStorageController => write!(f, "Mass Storage Controller"),
            DeviceClass::NetworkController => write!(f, "Network Controller"),
            DeviceClass::DisplayController => write!(f, "Display Controller"),
            DeviceClass::MultimediaController => write!(f, "Multimedia Controller"),
            DeviceClass::MemoryController => write!(f, "Memory Controller"),
            DeviceClass::Bridge => write!(f, "Bridge"),
            DeviceClass::CommunicationController => write!(f, "Communication Controller"),
            DeviceClass::GenericSystemPeripheral => write!(f, "Generic System Peripheral"),
            DeviceClass::InputDeviceController => write!(f, "Input Device Controller"),
            DeviceClass::DockingStation => write!(f, "Docking Station"),
            DeviceClass::Processor => write!(f, "Processor"),
            DeviceClass::Coprocessor => write!(f, "Coprocessor"),
            DeviceClass::SerialBusController => write!(f, "Serial Bus Controller"),
            DeviceClass::WirelessController => write!(f, "Wireless Controller"),
            DeviceClass::IntelligentController => write!(f, "Intelligent Controller"),
            DeviceClass::SatelliteCommunicationsController => {
                write!(f, "Satellite Communications Controller")
            }
            DeviceClass::EncryptionController => write!(f, "Encryption Controller"),
            DeviceClass::SignalProcessingController => write!(f, "Signal Processing Controller"),
            DeviceClass::ProcessingAccelerator => write!(f, "Processing Accelerators"),
            DeviceClass::NonEssentialInstrumentation => write!(f, "Non Essential Instrumentation"),
            DeviceClass::Unassigned => write!(f, "Unassigned"),
        }
    }
}
