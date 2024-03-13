use std::{collections::HashMap, rc::Rc};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Bitness
{
    Bit32 = 0x01,
    Bit64 = 0x02
}

impl Bitness
{
    pub fn to_num_bits(&self) -> u32
    {
        match self
        {
            Bitness::Bit32 => 32,
            Bitness::Bit64 => 64
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Endianness
{
    Little = 0x01,
    Big = 0x02
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ABI
{
    SystemV = 0x00,
    HPUX = 0x01,
    NetBSD = 0x02,
    Linux = 0x03,
    GNUHurd = 0x04,
    Solaris = 0x06,
    AIXMonterey = 0x07,
    IRIX = 0x08,
    FreeBSD = 0x09,
    Tru64 = 0x0A,
    NovellModesto = 0x0B,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    NonStopKernel = 0x0E,
    AROS = 0x0F,
    FenixOS = 0x10,
    NuxiCloudABI = 0x11,
    StratusTechnologiesOpenVOS = 0x12,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum FileType
{
    EtNone = 0x00,
    EtRel = 0x01,
    EtExec = 0x02,
    EtDyn = 0x03,
    EtCore = 0x04,
    EtLoos = 0xFE00,
    EtHios = 0xFEFF,
    EtLoproc = 0xFF00,
    EtHiproc = 0xFFFF,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum InstructionSet
{
    NospecificinstructionSet = 0x00,
    ATandTWE32100 = 0x01,
    SPARC = 0x02,
    X86 = 0x03,
    Motorola68000M68k = 0x04,
    Motorola88000M88k = 0x05,
    IntelMCU = 0x06,
    Intel80860 = 0x07,
    MIPS = 0x08,
    IBMSystem370 = 0x09,
    MIPSRS3000LittleEndian = 0x0A,
    Reserved0 = 0x0B,
    Reserved1 = 0x0C,
    Reserved2 = 0x0D,
    Reserved3 = 0x0E,
    HewlettPackardPARISC = 0x0F,
    Intel80960 = 0x13,
    PowerPC = 0x14,
    PowerPC64bit = 0x15,
    S390 = 0x16,
    IBMSPUSPC = 0x17,
    Reserved4 = 0x18,
    Reserved5 = 0x19,
    Reserved6 = 0x1A,
    Reserved7 = 0x1B,
    Reserved8 = 0x1C,
    Reserved9 = 0x1D,
    Reserved10 = 0x1E,
    Reserved11 = 0x1F,
    Reserved12 = 0x20,
    Reserved13 = 0x21,
    Reserved14 = 0x22,
    Reserved15 = 0x23,
    NECV800 = 0x24,
    FujitsuFR20 = 0x25,
    TRWRH32 = 0x26,
    MotorolaRCE = 0x27,
    ArmArmv7AArch32 = 0x28,
    DigitalAlpha = 0x29,
    SuperH = 0x2A,
    SPARCVersion9 = 0x2B,
    SiemensTriCoreEmbeddedProcessor = 0x2C,
    ArgonautRISCCore = 0x2D,
    HitachiH8300 = 0x2E,
    HitachiH8300H = 0x2F,
    HitachiH8S = 0x30,
    HitachiH8500 = 0x31,
    IA64 = 0x32,
    StanfordMIPSX = 0x33,
    MotorolaColdFire = 0x34,
    MotorolaM68HC12 = 0x35,
    FujitsuMMAMultimediaAccelerator = 0x36,
    SiemensPCP = 0x37,
    SonynCPUembeddedRISCProcessor = 0x38,
    DensoNDR1microProcessor = 0x39,
    MotorolaStarCoreProcessor = 0x3A,
    ToyotaME16Processor = 0x3B,
    STMicroelectronicsST100Processor = 0x3C,
    AdvancedLogicCorpTinyJEmbeddedProcessorFamily = 0x3D,
    AMDx86_64 = 0x3E,
    SonyDSPProcessor = 0x3F,
    DigitalEquipmentCorpPDP10 = 0x40,
    DigitalEquipmentCorpPDP11 = 0x41,
    SiemensFX66microcontroller = 0x42,
    STMicroelectronicsST9816bitMicrocontroller = 0x43,
    STMicroelectronicsST78bitMicrocontroller = 0x44,
    MotorolaMC68HC16Microcontroller = 0x45,
    MotorolaMC68HC11Microcontroller = 0x46,
    MotorolaMC68HC08Microcontroller = 0x47,
    MotorolaMC68HC05Microcontroller = 0x48,
    SiliconGraphicsSVx = 0x49,
    STMicroelectronicsST198bitMicrocontroller = 0x4A,
    DigitalVAX = 0x4B,
    AxisCommunications32bitEmbeddedProcessor = 0x4C,
    InfineonTechnologies32bitEmbeddedProcessor = 0x4D,
    Element1464bitDSPProcessor = 0x4E,
    LSILogic16bitDSPProcessor = 0x4F,
    TMS320C6000Family = 0x8C,
    MCSTElbruse2k = 0xAF,
    Arm64bitsArmv8AArch64 = 0xB7,
    ZilogZ80 = 0xDC,
    RISCV = 0xF3,
    BerkeleyPacketFilter = 0xF7,
    WDC65C816 = 0x101,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Section
{
    pub name: String,
    pub section_type: u32,
    pub flags: u64,
    pub address: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub address_alignment: u64,
    pub entry_size: u64,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ElfHeader
{
    pub bitness: Bitness,
    pub endianness: Endianness,
    pub abi: ABI,
    pub dynamic_linker_version: u8,
    pub file_type: FileType,
    pub instruction_set: InstructionSet,
    pub elf_version: u8,
    pub entry_point: u64,
    pub program_header_offset: u64,
    pub section_header_table: u64,
    pub flags: u32,
    pub header_size: u16,
    pub program_header_entry_size: u16,
    pub program_header_entry_count: u16,
    pub section_header_entry_size: u16,
    pub section_header_entry_count: u16,
    pub section_header_string_table_index: u16,
    pub section_table: Vec<Section>,
    pub symbol_table: Rc<HashMap<u64, String>>,
    pub inverse_symbol_table: HashMap<String, u64>
}

pub struct Elf
{
    pub header: ElfHeader,
    pub text: Vec<u8>,
}

impl Elf
{
    pub fn parse_elf(bytes: &[u8]) -> Option<Self>
    {
        let header = ElfHeader::parse_header(bytes)?;
        let text = Vec::from(&bytes[header.program_header_offset as usize..]);
        Some(Self { header, text })
    }
}

impl ElfHeader
{
    fn is_elf(bytes: &[u8]) -> bool
    {
        bytes.len() >= 4 && bytes[0] == 0x7F && bytes[1] == 'E' as u8 && bytes[2] == 'L' as u8 && bytes[3] == 'F' as u8
    }

    fn parse_symbols(symbol_table: Option<&Section>, string_table: Option<&Section>, bytes: &[u8], bitness: Bitness, endianness: Endianness) -> HashMap<u64, String>
    {
        let mut symbols = HashMap::new();
        if symbol_table.is_some() && string_table.is_some()
        {
            let symbol_table = symbol_table.unwrap();
            let string_table = string_table.unwrap();
            let symbol_table_bytes = &bytes[symbol_table.offset as usize..symbol_table.offset as usize + symbol_table.size as usize];
            let string_table_bytes = &bytes[string_table.offset as usize..string_table.offset as usize + string_table.size as usize];
            let symbol_entry_size = match bitness
            {
                Bitness::Bit32 => 16,
                Bitness::Bit64 => 24
            };
            for symbol in symbol_table_bytes.chunks_exact(symbol_entry_size)
            {
                let name_offset = match endianness
                {
                    Endianness::Little => u32::from_le_bytes([symbol[0], symbol[1], symbol[2], symbol[3]]),
                    Endianness::Big => u32::from_be_bytes([symbol[0], symbol[1], symbol[2], symbol[3]])
                };
                let name = match string_table_bytes.get(name_offset as usize..)
                {
                    Some(name) => 
                    {
                        let max_name_size = 0x40;
                        let name: String = name.iter().take_while(|&&c| c != 0).take(max_name_size).map(|&c| c as char).collect();
                        name
                    },
                    None => 
                    {
                        continue;
                    }
                };

                let value = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([symbol[4], symbol[5], symbol[6], symbol[7]]) as u64,
                        Endianness::Big => u32::from_be_bytes([symbol[4], symbol[5], symbol[6], symbol[7]]) as u64
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u64::from_le_bytes([symbol[8], symbol[9], symbol[10], symbol[11], symbol[12], symbol[13], symbol[14], symbol[15]]),
                        Endianness::Big => u64::from_be_bytes([symbol[8], symbol[9], symbol[10], symbol[11], symbol[12], symbol[13], symbol[14], symbol[15]])
                    }
                };
                symbols.insert(value, name);
            }
        }
        symbols
    }

    pub fn parse_header(bytes: &[u8]) -> Option<Self>
    {
        if Self::is_elf(bytes)
        {
            if bytes.len() >= 0x34
            {
                let bitness = match bytes[4]
                {
                    1 => Bitness::Bit32,
                    2 => Bitness::Bit64,
                    _ => return None
                };
                if bitness == Bitness::Bit64 && bytes.len() < 0x40
                {
                    return None
                }
                let endianness = match bytes[5]
                {
                    1 => Endianness::Little,
                    2 => Endianness::Big,
                    _ => return None
                };
                let _ident = match bytes[6]
                {
                    1 => 1,
                    _ => return None
                };
                let abi = match bytes[7]
                {
                    0 => ABI::SystemV,
                    1 => ABI::HPUX,
                    2 => ABI::NetBSD,
                    3 => ABI::Linux,
                    4 => ABI::GNUHurd,
                    6 => ABI::Solaris,
                    7 => ABI::AIXMonterey,
                    8 => ABI::IRIX,
                    9 => ABI::FreeBSD,
                    10 => ABI::Tru64,
                    11 => ABI::NovellModesto,
                    12 => ABI::OpenBSD,
                    13 => ABI::OpenVMS,
                    14 => ABI::NonStopKernel,
                    15 => ABI::AROS,
                    16 => ABI::FenixOS,
                    17 => ABI::NuxiCloudABI,
                    18 => ABI::StratusTechnologiesOpenVOS,
                    _ => return None
                };

                let dynamic_linker_version = bytes[8];

                let _padding = &bytes[0x9..0x10];

                let file_type = match endianness
                {
                    Endianness::Little => u16::from_le_bytes([bytes[0x10], bytes[0x11]]),
                    Endianness::Big => u16::from_be_bytes([bytes[0x10], bytes[0x11]])
                };

                let file_type = match file_type
                {
                    0 => FileType::EtNone,
                    1 => FileType::EtRel,
                    2 => FileType::EtExec,
                    3 => FileType::EtDyn,
                    4 => FileType::EtCore,
                    0xFE00..=0xFEFF => FileType::EtLoos,
                    0xFF00..=0xFFFF => FileType::EtLoproc,
                    _ => return None
                };

                let instruction_set = match endianness
                {
                    Endianness::Little => u16::from_le_bytes([bytes[0x12], bytes[0x13]]),
                    Endianness::Big => u16::from_be_bytes([bytes[0x12], bytes[0x13]])
                };

                let instruction_set = match instruction_set
                {
                    0 => InstructionSet::NospecificinstructionSet,
                    1 => InstructionSet::ATandTWE32100,
                    2 => InstructionSet::SPARC,
                    3 => InstructionSet::X86,
                    4 => InstructionSet::Motorola68000M68k,
                    5 => InstructionSet::Motorola88000M88k,
                    6 => InstructionSet::IntelMCU,
                    7 => InstructionSet::Intel80860,
                    8 => InstructionSet::MIPS,
                    9 => InstructionSet::IBMSystem370,
                    0xA => InstructionSet::MIPSRS3000LittleEndian,
                    0xB => InstructionSet::Reserved0,
                    0xC => InstructionSet::Reserved1,
                    0xD => InstructionSet::Reserved2,
                    0xE => InstructionSet::Reserved3,
                    0xF => InstructionSet::HewlettPackardPARISC,
                    0x13 => InstructionSet::Intel80960,
                    0x14 => InstructionSet::PowerPC,
                    0x15 => InstructionSet::PowerPC64bit,
                    0x16 => InstructionSet::S390,
                    0x17 => InstructionSet::IBMSPUSPC,
                    0x18 => InstructionSet::Reserved4,
                    0x19 => InstructionSet::Reserved5,
                    0x1A => InstructionSet::Reserved6,
                    0x1B => InstructionSet::Reserved7,
                    0x1C => InstructionSet::Reserved8,
                    0x1D => InstructionSet::Reserved9,
                    0x1E => InstructionSet::Reserved10,
                    0x1F => InstructionSet::Reserved11,
                    0x20 => InstructionSet::Reserved12,
                    0x21 => InstructionSet::Reserved13,
                    0x22 => InstructionSet::Reserved14,
                    0x23 => InstructionSet::Reserved15,
                    0x24 => InstructionSet::NECV800,
                    0x25 => InstructionSet::FujitsuFR20,
                    0x26 => InstructionSet::TRWRH32,
                    0x27 => InstructionSet::MotorolaRCE,
                    0x28 => InstructionSet::ArmArmv7AArch32,
                    0x29 => InstructionSet::DigitalAlpha,
                    0x2A => InstructionSet::SuperH,
                    0x2B => InstructionSet::SPARCVersion9,
                    0x2C => InstructionSet::SiemensTriCoreEmbeddedProcessor,
                    0x2D => InstructionSet::ArgonautRISCCore,
                    0x2E => InstructionSet::HitachiH8300,
                    0x2F => InstructionSet::HitachiH8300H,
                    0x30 => InstructionSet::HitachiH8S,
                    0x31 => InstructionSet::HitachiH8500,
                    0x32 => InstructionSet::IA64,
                    0x33 => InstructionSet::StanfordMIPSX,
                    0x34 => InstructionSet::MotorolaColdFire,
                    0x35 => InstructionSet::MotorolaM68HC12,
                    0x36 => InstructionSet::FujitsuMMAMultimediaAccelerator,
                    0x37 => InstructionSet::SiemensPCP,
                    0x38 => InstructionSet::SonynCPUembeddedRISCProcessor,
                    0x39 => InstructionSet::DensoNDR1microProcessor,
                    0x3A => InstructionSet::MotorolaStarCoreProcessor,
                    0x3B => InstructionSet::ToyotaME16Processor,
                    0x3C => InstructionSet::STMicroelectronicsST100Processor,
                    0x3D => InstructionSet::AdvancedLogicCorpTinyJEmbeddedProcessorFamily,
                    0x3E => InstructionSet::AMDx86_64,
                    0x3F => InstructionSet::SonyDSPProcessor,
                    0x40 => InstructionSet::DigitalEquipmentCorpPDP10,
                    0x41 => InstructionSet::DigitalEquipmentCorpPDP11,
                    0x42 => InstructionSet::SiemensFX66microcontroller,
                    0x43 => InstructionSet::STMicroelectronicsST9816bitMicrocontroller,
                    0x44 => InstructionSet::STMicroelectronicsST78bitMicrocontroller,
                    0x45 => InstructionSet::MotorolaMC68HC16Microcontroller,
                    0x46 => InstructionSet::MotorolaMC68HC11Microcontroller,
                    0x47 => InstructionSet::MotorolaMC68HC08Microcontroller,
                    0x48 => InstructionSet::MotorolaMC68HC05Microcontroller,
                    0x49 => InstructionSet::SiliconGraphicsSVx,
                    0x4A => InstructionSet::STMicroelectronicsST198bitMicrocontroller,
                    0x4B => InstructionSet::DigitalVAX,
                    0x4C => InstructionSet::AxisCommunications32bitEmbeddedProcessor,
                    0x4D => InstructionSet::InfineonTechnologies32bitEmbeddedProcessor,
                    0x4E => InstructionSet::Element1464bitDSPProcessor,
                    0x4F => InstructionSet::LSILogic16bitDSPProcessor,
                    0x8C => InstructionSet::TMS320C6000Family,
                    0xAF => InstructionSet::MCSTElbruse2k,
                    0xB7 => InstructionSet::Arm64bitsArmv8AArch64,
                    0xDC => InstructionSet::ZilogZ80,
                    0xF3 => InstructionSet::RISCV,
                    0xF7 => InstructionSet::BerkeleyPacketFilter,
                    0x101 => InstructionSet::WDC65C816,
                    _ => return None
                };

                let elf_version = match bytes[0x14]
                {
                    1 => 1,
                    _ => return None
                };

                let entry_point = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([bytes[0x18], bytes[0x19], bytes[0x1A], bytes[0x1B]]) as u64,
                        Endianness::Big => u32::from_be_bytes([bytes[0x18], bytes[0x19], bytes[0x1A], bytes[0x1B]]) as u64
                    }
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u64::from_le_bytes([bytes[0x18], bytes[0x19], bytes[0x1A], bytes[0x1B], bytes[0x1C], bytes[0x1D], bytes[0x1E], bytes[0x1F]]),
                        Endianness::Big => u64::from_be_bytes([bytes[0x18], bytes[0x19], bytes[0x1A], bytes[0x1B], bytes[0x1C], bytes[0x1D], bytes[0x1E], bytes[0x1F]])
                    }
                };

                let program_header_offset = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([bytes[0x1C], bytes[0x1D], bytes[0x1E], bytes[0x1F]]) as u64,
                        Endianness::Big => u32::from_be_bytes([bytes[0x1C], bytes[0x1D], bytes[0x1E], bytes[0x1F]]) as u64

                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u64::from_le_bytes([bytes[0x20], bytes[0x21], bytes[0x22], bytes[0x23], bytes[0x24], bytes[0x25], bytes[0x26], bytes[0x27]]),
                        Endianness::Big => u64::from_be_bytes([bytes[0x20], bytes[0x21], bytes[0x22], bytes[0x23], bytes[0x24], bytes[0x25], bytes[0x26], bytes[0x27]])
                    }
                };

                let section_header_table = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([bytes[0x20], bytes[0x21], bytes[0x22], bytes[0x23]]) as u64,
                        Endianness::Big => u32::from_be_bytes([bytes[0x20], bytes[0x21], bytes[0x22], bytes[0x23]]) as u64
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u64::from_le_bytes([bytes[0x28], bytes[0x29], bytes[0x2A], bytes[0x2B], bytes[0x2C], bytes[0x2D], bytes[0x2E], bytes[0x2F]]),
                        Endianness::Big => u64::from_be_bytes([bytes[0x28], bytes[0x29], bytes[0x2A], bytes[0x2B], bytes[0x2C], bytes[0x2D], bytes[0x2E], bytes[0x2F]])
                    }
                };

                let flags = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([bytes[0x24], bytes[0x25], bytes[0x26], bytes[0x27]]),
                        Endianness::Big => u32::from_be_bytes([bytes[0x24], bytes[0x25], bytes[0x26], bytes[0x27]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([bytes[0x30], bytes[0x31], bytes[0x32], bytes[0x33]]),
                        Endianness::Big => u32::from_be_bytes([bytes[0x30], bytes[0x31], bytes[0x32], bytes[0x33]])
                    }
                };

                let header_size = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x28], bytes[0x29]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x28], bytes[0x29]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x34], bytes[0x35]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x34], bytes[0x35]])
                    }
                };

                let program_header_entry_size = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x2A], bytes[0x2B]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x2A], bytes[0x2B]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x36], bytes[0x37]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x36], bytes[0x37]])
                    }
                };

                let program_header_entry_count = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x2C], bytes[0x2D]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x2C], bytes[0x2D]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x38], bytes[0x39]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x38], bytes[0x39]])
                    }
                };

                let section_header_entry_size = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x2E], bytes[0x2F]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x2E], bytes[0x2F]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x3A], bytes[0x3B]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x3A], bytes[0x3B]])
                    }
                };

                let section_header_entry_count = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x30], bytes[0x31]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x30], bytes[0x31]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x3C], bytes[0x3D]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x3C], bytes[0x3D]])
                    }
                };

                let section_header_string_table_index = match bitness
                {
                    Bitness::Bit32 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x32], bytes[0x33]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x32], bytes[0x33]])
                    },
                    Bitness::Bit64 => match endianness
                    {
                        Endianness::Little => u16::from_le_bytes([bytes[0x3E], bytes[0x3F]]),
                        Endianness::Big => u16::from_be_bytes([bytes[0x3E], bytes[0x3F]])
                    }
                };

                if section_header_table as usize + section_header_entry_count as usize * section_header_entry_size as usize > bytes.len()
                {
                    return None;
                }
                let section_table = &bytes[section_header_table as usize .. section_header_table as usize + section_header_entry_count as usize * section_header_entry_size as usize];
                let mut sections = Vec::new();
                let section_table = section_table.chunks_exact(section_header_entry_size as usize);
                if section_table.len() != section_header_entry_count as usize
                {
                    return None;
                }
                let section_string_table = &bytes[section_header_table as usize + section_header_string_table_index as usize * section_header_entry_size as usize..];
                let string_table_offset = match bitness
                {
                    Bitness::Bit32 => 
                    {
                        match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section_string_table[0x10], section_string_table[0x11], section_string_table[0x12], section_string_table[0x13]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section_string_table[0x10], section_string_table[0x11], section_string_table[0x12], section_string_table[0x13]]) as u64,
                        }
                    },
                    Bitness::Bit64 => 
                    {
                        match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section_string_table[0x18], section_string_table[0x19], section_string_table[0x1A], section_string_table[0x1B], section_string_table[0x1C], section_string_table[0x1D], section_string_table[0x1E], section_string_table[0x1F]]),
                            Endianness::Big => u64::from_be_bytes([section_string_table[0x18], section_string_table[0x19], section_string_table[0x1A], section_string_table[0x1B], section_string_table[0x1C], section_string_table[0x1D], section_string_table[0x1E], section_string_table[0x1F]]),
                        }
                    },
                };
                let string_table = &bytes[string_table_offset as usize..];
                for section in section_table
                {
                    let name_offset = match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([section[0], section[1], section[2], section[3]]),
                        Endianness::Big => u32::from_be_bytes([section[0], section[1], section[2], section[3]])
                    };
                    let name = match string_table.get(name_offset as usize..)
                    {
                        Some(name) => 
                        {
                            let max_name_size = 0x40;
                            let name: String = name.iter().take_while(|&&c| c != 0).take(max_name_size).map(|&c| c as char).collect();
                            name
                        },
                        None => 
                        {
                            return None
                        }
                    };
                    let section_type = match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([section[4], section[5], section[6], section[7]]),
                        Endianness::Big => u32::from_be_bytes([section[4], section[5], section[6], section[7]])
                    };
                    let flags = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[8], section[9], section[10], section[11]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[8], section[9], section[10], section[11]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[8], section[9], section[10], section[11], section[12], section[13], section[14], section[15]]),
                            Endianness::Big => u64::from_be_bytes([section[8], section[9], section[10], section[11], section[12], section[13], section[14], section[15]])
                        }
                    };
                    let address = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[12], section[13], section[14], section[15]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[12], section[13], section[14], section[15]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[16], section[17], section[18], section[19], section[20], section[21], section[22], section[23]]),
                            Endianness::Big => u64::from_be_bytes([section[16], section[17], section[18], section[19], section[20], section[21], section[22], section[23]])
                        }
                    };
                    let offset = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[16], section[17], section[18], section[19]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[16], section[17], section[18], section[19]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[24], section[25], section[26], section[27], section[28], section[29], section[30], section[31]]),
                            Endianness::Big => u64::from_be_bytes([section[24], section[25], section[26], section[27], section[28], section[29], section[30], section[31]])
                        }
                    };
                    let size = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[20], section[21], section[22], section[23]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[20], section[21], section[22], section[23]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[32], section[33], section[34], section[35], section[36], section[37], section[38], section[39]]),
                            Endianness::Big => u64::from_be_bytes([section[32], section[33], section[34], section[35], section[36], section[37], section[38], section[39]])
                        }
                    };
                    let link = match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([section[24], section[25], section[26], section[27]]),
                        Endianness::Big => u32::from_be_bytes([section[24], section[25], section[26], section[27]])
                    };
                    let info = match endianness
                    {
                        Endianness::Little => u32::from_le_bytes([section[28], section[29], section[30], section[31]]),
                        Endianness::Big => u32::from_be_bytes([section[28], section[29], section[30], section[31]])
                    };
                    let address_alignment = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[32], section[33], section[34], section[35]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[32], section[33], section[34], section[35]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[40], section[41], section[42], section[43], section[44], section[45], section[46], section[47]]),
                            Endianness::Big => u64::from_be_bytes([section[40], section[41], section[42], section[43], section[44], section[45], section[46], section[47]])
                        }
                    };
                    let entry_size = match bitness
                    {
                        Bitness::Bit32 => match endianness
                        {
                            Endianness::Little => u32::from_le_bytes([section[36], section[37], section[38], section[39]]) as u64,
                            Endianness::Big => u32::from_be_bytes([section[36], section[37], section[38], section[39]]) as u64
                        },
                        Bitness::Bit64 => match endianness
                        {
                            Endianness::Little => u64::from_le_bytes([section[48], section[49], section[50], section[51], section[52], section[53], section[54], section[55]]),
                            Endianness::Big => u64::from_be_bytes([section[48], section[49], section[50], section[51], section[52], section[53], section[54], section[55]])
                        }
                    };
                    if section_type & 0x8 != 0
                    {
                        // this is a SHT_NOBITS section
                        continue;
                    }
                    sections.push(
                        Section{
                            name,
                            section_type,
                            flags,
                            address,
                            offset,
                            size,
                            link,
                            info,
                            address_alignment,
                            entry_size
                        }
                    );
                }

                let mut symbols = HashMap::new();
                let mut symbol_table = None;
                let mut dynamic_symbol_table = None;
                let mut string_table = None;
                for section in &sections
                {
                    if section.name == ".symtab"
                    {
                        symbol_table = Some(section);
                    }
                    else if section.name == ".strtab"
                    {
                        string_table = Some(section);
                    }
                    else if section.name == ".dynsym"
                    {
                        dynamic_symbol_table = Some(section);
                    }
                }

                if dynamic_symbol_table.is_some()
                {
                    symbols.extend(Self::parse_symbols(dynamic_symbol_table, string_table, bytes, bitness, endianness).into_iter());
                }
                if symbol_table.is_some()
                {
                    symbols.extend(Self::parse_symbols(symbol_table, string_table, bytes, bitness, endianness).into_iter());
                }

                let inverse_symbol_table = symbols.iter().map(|(k,v)| (v.clone(), *k)).collect();

                Some(ElfHeader {
                    bitness,
                    endianness,
                    abi,
                    dynamic_linker_version,
                    file_type,
                    instruction_set,
                    elf_version,
                    entry_point,
                    program_header_offset,
                    section_header_table,
                    flags,
                    header_size,
                    program_header_entry_size,
                    program_header_entry_count,
                    section_header_entry_size,
                    section_header_entry_count,
                    section_header_string_table_index,
                    section_table: sections,
                    symbol_table: Rc::new(symbols),
                    inverse_symbol_table
                })
            }
            else
            {
                None
            }
        }
        else
        {
            None
        }
    }

    pub fn bitness(&self) -> u32
    {
        self.bitness.to_num_bits()
    }

    pub fn print_header(data: &[u8])
    {
        let header = ElfHeader::parse_header(data).expect("File is not ELF");
        dbg!(header);
    }

    pub fn get_symbols(&self) -> Rc<HashMap<u64,String>>
    {
        self.symbol_table.clone()
    }
}
