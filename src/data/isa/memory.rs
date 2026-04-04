#[derive(Clone, PartialEq, Debug)]
pub struct MemoryRegion {
    pub name: &'static str,
    pub start: u32,
    pub end: u32,
    pub size: &'static str,
    pub region_type: MemoryType,
    pub description: &'static str,
}

#[derive(Clone, PartialEq, Debug)]
pub enum MemoryType {
    Ram,
    Stack,
    Unmapped,
    Io,
}

impl MemoryType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Ram => "SRAM",
            Self::Stack => "EBR Stack",
            Self::Unmapped => "Unmapped",
            Self::Io => "I/O",
        }
    }
}

pub fn all_regions() -> &'static [MemoryRegion; 6] {
    &REGIONS
}

static REGIONS: [MemoryRegion; 6] = [
    MemoryRegion {
        name: "SRAM",
        start: 0x000000,
        end: 0x0FFFFF,
        size: "1 MB",
        region_type: MemoryType::Ram,
        description: "Main memory. Code, data, heap. Reset vector at 0x000000.",
    },
    MemoryRegion {
        name: "Unmapped",
        start: 0x100000,
        end: 0xFEDFFF,
        size: "~14 MB",
        region_type: MemoryType::Unmapped,
        description: "Addressable but no physical memory. Reads return 0.",
    },
    MemoryRegion {
        name: "EBR",
        start: 0xFEE000,
        end: 0xFEFFFF,
        size: "8 KB window",
        region_type: MemoryType::Stack,
        description:
            "Embedded Block RAM. 3 KB populated on MachXO. Stack grows down from 0xFEEC00.",
    },
    MemoryRegion {
        name: "Reserved",
        start: 0xFF0000 - 1,
        end: 0xFF0000 - 1,
        size: "0 bytes",
        region_type: MemoryType::Unmapped,
        description: "Gap between EBR window and I/O space.",
    },
    MemoryRegion {
        name: "LED/Switch",
        start: 0xFF0000,
        end: 0xFF0000,
        size: "1 byte",
        region_type: MemoryType::Io,
        description: "Bit 0: write=LED D2, read=button S2.",
    },
    MemoryRegion {
        name: "UART",
        start: 0xFF0100,
        end: 0xFF0101,
        size: "2 bytes",
        region_type: MemoryType::Io,
        description: "0xFF0100: data (R/W), 0xFF0101: status (RX ready, TX busy, CTS, overflow).",
    },
];

#[derive(Clone, PartialEq, Debug)]
pub struct IoRegister {
    pub name: &'static str,
    pub address: u32,
    pub size: u8,
    pub read_write: &'static str,
    pub description: &'static str,
}

pub fn all_io_registers() -> &'static [IoRegister; 4] {
    &IO_REGISTERS
}

static IO_REGISTERS: [IoRegister; 4] = [
    IoRegister {
        name: "IO_LEDSWDAT",
        address: 0xFF0000,
        size: 1,
        read_write: "R/W",
        description: "LED/switch data. Write bit 0 = LED D2 on. Read bit 0 = button S2 state.",
    },
    IoRegister {
        name: "IO_INTENABLE",
        address: 0xFF0010,
        size: 1,
        read_write: "R/W",
        description: "Interrupt enable. Bit 0 = UART RX interrupt enable.",
    },
    IoRegister {
        name: "IO_UARTDATA",
        address: 0xFF0100,
        size: 1,
        read_write: "R/W",
        description:
            "UART data. Write = transmit byte. Read = receive byte (auto-acknowledges RX).",
    },
    IoRegister {
        name: "IO_UARTSTAT",
        address: 0xFF0101,
        size: 1,
        read_write: "R",
        description:
            "UART status. Bit 0: RX ready, bit 1: CTS, bit 2: RX overflow, bit 7: TX busy.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_count() {
        assert_eq!(all_regions().len(), 6);
    }

    #[test]
    fn regions_sorted_by_address() {
        let regions = all_regions();
        for i in 1..regions.len() {
            assert!(
                regions[i].start >= regions[i - 1].start,
                "regions not sorted at index {}",
                i
            );
        }
    }

    #[test]
    fn io_register_count() {
        assert_eq!(all_io_registers().len(), 4);
    }
}
