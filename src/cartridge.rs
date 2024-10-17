use phf::phf_map;
use anyhow::{
    Result, 
    anyhow
};

pub struct Cartridge {
    entry: [u8; 0x4],
    logo: [u8; 0x30],
    title: [u8; 0x10],
    new_lic_code: u16,
    sgb_flag: u8,
    cart_type: u8,
    rom_size: u8,
    ram_size: u8,
    dest_code: u8,
    old_lic_code: u8,
    version: u8,
    checksum: u8,
    global_checksum: u16,
}

static CART_TYPE_MAP: phf::Map<u8, &'static str> = phf_map! {
    0x00u8 => "ROM ONLY",
    0x01u8 => "MBC1",
    0x02u8 => "MBC1+RAM",
    0x03u8 => "MBC1+RAM+BATTERY",
    0x05u8 => "MBC2",
    0x06u8 => "MBC2+BATTERY",
    0x08u8 => "ROM+RAM",
    0x09u8 => "ROM+RAM+BATTERY",
    0x0Bu8 => "MMM01",
    0x0Cu8 => "MMM01+RAM",
    0x0Du8 => "MMM01+RAM+BATTERY",
    0x0Fu8 => "MBC3+TIMER+BATTERY",
    0x10u8 => "MBC3+TIMER+RAM+BATTERY",
    0x11u8 => "MBC3",
    0x12u8 => "MBC3+RAM",
    0x13u8 => "MBC3+RAM+BATTERY",
    0x19u8 => "MBC5",
    0x1Au8 => "MBC5+RAM",
    0x1Bu8 => "MBC5+RAM+BATTERY",
    0x1Cu8 => "MBC5+RUMBLE",
    0x1Du8 => "MBC5+RUMBLE+RAM",
    0x1Eu8 => "MBC5+RUMBLE+RAM+BATTERY",
    0x20u8 => "MBC6",
    0x22u8 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
    0xFCu8 => "POCKET CAMERA",
    0xFDu8 => "BANDAI TAMA5",
    0xFEu8 => "HuC3",
    0xFFu8 => "HuC1+RAM+BATTERY",
};

static NEW_LICENSEE_CODE_MAP: phf::Map<u16, &'static str> = phf_map! {
    0x3030u16 => "None",
    0x3031u16 => "Nintendo Research & Development 1",
    0x3038u16 => "Capcom",
    0x3133u16 => "EA (Electronic Arts)",
    0x3138u16 => "Hudson Soft",
    0x3139u16 => "B-AI",
    0x3230u16 => "KSS",
    0x3232u16 => "Planning Office WADA",
    0x3234u16 => "PCM Complete",
    0x3235u16 => "San-X",
    0x3238u16 => "Kemco",
    0x3239u16 => "SETA Corporation",
    0x3330u16 => "Viacom",
    0x3331u16 => "Nintendo",
    0x3332u16 => "Bandai",
    0x3333u16 => "Ocean Software/Acclaim Entertainment",
    0x3334u16 => "Konami",
    0x3335u16 => "HectorSoft",
    0x3337u16 => "Taito",
    0x3338u16 => "Hudson Soft",
    0x3339u16 => "Banpresto",
    0x3431u16 => "Ubi Soft",
    0x3432u16 => "Atlus",
    0x3434u16 => "Malibu Interactive",
    0x3436u16 => "Angel",
    0x3437u16 => "Bullet-Proof Software",
    0x3439u16 => "Irem",
    0x3530u16 => "Absolute",
    0x3531u16 => "Acclaim Entertainment",
    0x3532u16 => "Activision",
    0x3533u16 => "Sammy USA Corporation",
    0x3534u16 => "Konami",
    0x3535u16 => "Hi Tech Expressions",
    0x3536u16 => "LJN",
    0x3537u16 => "Matchbox",
    0x3538u16 => "Mattel",
    0x3539u16 => "Milton Bradley Company",
    0x3630u16 => "Titus Interactive",
    0x3631u16 => "Virgin Games Ltd.",
    0x3634u16 => "Lucasfilm Games",
    0x3637u16 => "Ocean Software",
    0x3639u16 => "EA (Electronic Arts)",
    0x3730u16 => "Infogrames",
    0x3731u16 => "Interplay Entertainment",
    0x3732u16 => "Broderbund",
    0x3733u16 => "Sculptured Software",
    0x3735u16 => "The Sales Curve Limited",
    0x3738u16 => "THQ",
    0x3739u16 => "Accolade",
    0x3830u16 => "Misawa Entertainment",
    0x3833u16 => "lozc",
    0x3836u16 => "Tokuma Shoten",
    0x3837u16 => "Tsukuda Original",
    0x3931u16 => "Chunsoft Co.",
    0x3932u16 => "Video System",
    0x3933u16 => "Ocean Software/Acclaim Entertainment",
    0x3935u16 => "Varie",
    0x3936u16 => "Yonezawa/s'pal",
    0x3937u16 => "Kaneko",
    0x3939u16 => "Pack-In-Video",
    0x4134u16 => "Konami (Yu-Gi-Oh!)",
};

static OLD_LICENSEE_CODE_MAP: phf::Map<u8, &'static str> = phf_map! {
    0x00u8 => "None",
    0x01u8 => "Nintendo",
    0x08u8 => "",
    0x09u8 => "",
    0x0Au8 => "",
    0x0Bu8 => "",
    0x0Cu8 => "",
    0x13u8 => "",
    0x18u8 => "",
    0x19u8 => "",
    0x1Au8 => "",
    0x1Du8 => "",
    0x1Fu8 => "",
    0x24u8 => "",
    0x25u8 => "",
    0x28u8 => "",
    0x29u8 => "",
    0x30u8 => "",
    0x31u8 => "",
    0x32u8 => "",
    0x33u8 => "",
    0x34u8 => "",
    0x35u8 => "",
    0x38u8 => "",
    0x39u8 => "",
    0x3Cu8 => "",
    0x3Eu8 => "",
    0x41u8 => "",
    0x42u8 => "",
    0x44u8 => "",
    0x46u8 => "",
    // TODO: finish this!
};

impl Cartridge {
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 0x150 {
            return Err(anyhow!("ROM data is not long enough"))
        }

        Ok(Self {
            entry: data[0x100..0x104].try_into()?,
            logo: data[0x104..0x134].try_into()?,
            title: data[0x134..0x144].try_into()?,
            new_lic_code: u16::from_be_bytes([data[0x144], data[0x145]]),
            sgb_flag: data[0x146],
            cart_type: data[0x147],
            rom_size: data[0x148],
            ram_size: data[0x149],
            dest_code: data[0x14A],
            old_lic_code: data[0x14B],
            version: data[0x14C],
            checksum: data[0x14D],
            global_checksum: u16::from_be_bytes([data[0x14E], data[0x14F]]),
        })
    }

    pub fn get_cart_type(&self) -> Result<&'static str> {
        CART_TYPE_MAP
            .get(&self.cart_type)
            .copied()
            .ok_or_else(|| anyhow!("Unknown cartridge type: {:#04x}", self.cart_type))
    }

    pub fn get_lic_code(&self) -> Result<&'static str> {
        // TODO: check if old or new lic code and return appropriate value
        if self.old_lic_code == 0x33 {
            NEW_LICENSEE_CODE_MAP
                .get(&self.new_lic_code)
                .copied()
                .ok_or_else(|| anyhow!("Unknown (new) licensee code: {:#04x}", self.new_lic_code))
        } else {
            OLD_LICENSEE_CODE_MAP
                .get(&self.old_lic_code)
                .copied()
                .ok_or_else(|| anyhow!("Unknown licensee code: {:#04x}", self.new_lic_code))
        }
    }
}

