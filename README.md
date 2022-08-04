# PCI IDs
PCI IDs is a small crate that extends an API over the pci.ids file that exists on Linux systems making it possible to easily extract information about a given PCI device's name, class and vendor.

## What is the pci.ids file?
In short, it is a giant text file containing unique(-ish) identifiers for hardware vendors' PCI devices. It makes lookup of the human-readable names category (class) these devices belong to much easier as it is given in the form of a file that follows very rigorous formatting.

Read [https://pci-ids.ucw.cz/](https://pci-ids.ucw.cz/) for more info.
