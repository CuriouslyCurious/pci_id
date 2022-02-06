//! ```
//! use std::path::Path;
//! use pci_id::{PciIds, PATH_TO_PCI_IDS, Device};
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
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod device_class;
pub mod pci_ids;
pub mod vendor;
pub mod class;

