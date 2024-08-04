use sp_core::crypto::Ss58AddressFormat;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum Chain {
    Kusama,
    Polkadot,
    Westend,
    PolkadotPeople,
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::Kusama => "Kusama",
            Self::Polkadot => "Polkadot",
            Self::Westend => "Westend",
            Self::PolkadotPeople => "Polkadot People",
        };
        write!(f, "{display}")
    }
}

impl FromStr for Chain {
    type Err = std::string::ParseError;

    /// Get chain from string.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kusama" | "ksm" => Ok(Self::Kusama),
            "polkadot" | "dot" => Ok(Self::Polkadot),
            "westend" | "wnd" => Ok(Self::Westend),
            "polkadot people" => Ok(Self::PolkadotPeople),
            _ => panic!("Unkown chain: {s}"),
        }
    }
}

impl Chain {
    /// SS58 encoding format for the chain.
    fn get_ss58_address_format(&self) -> Ss58AddressFormat {
        match self {
            Self::Kusama => Ss58AddressFormat::from(2u16),
            Self::Polkadot => Ss58AddressFormat::from(0u16),
            Self::Westend => Ss58AddressFormat::from(42u16),
            Self::PolkadotPeople => Ss58AddressFormat::from(0u16),
        }
    }

    pub fn sp_core_set_default_ss58_version(&self) {
        sp_core::crypto::set_default_ss58_version(self.get_ss58_address_format())
    }
}
