use std::fmt;
use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use itertools::Itertools;

use crate::class::FlagsError;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[derive(BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign)]
pub struct Flags(u16);

impl Flags {
    pub const fn new(flags: u16) -> Self {
        Flags(flags)
    }

    pub const fn empty() -> Self {
        Flags(0)
    }

    pub fn has(&self, flags: Self) -> bool {
        (self.0 & flags.0) == flags.0
    }

    pub fn has_any(&self, flags: Self) -> bool {
        (self.0 & flags.0) != 0
    }

    pub fn has_one(&self, flags: Self) -> bool {
        let or = self.0 & flags.0;
        if or == 0 { false } else { (or & (or - 1)) == 1 }
    }

    pub fn has_at_most_one(&self, flags: Self) -> bool {
        let or = self.0 & flags.0;
        if or == 0 { true } else { (or & (or - 1)) == 1 }
    }
}


#[derive(Copy, Clone)]
pub struct ClassFlags {
    flags: Flags
}

impl ClassFlags {
    pub const PUBLIC: Flags = Flags::new(0x0001);
    pub const FINAL: Flags = Flags::new(0x0010);
    pub const SUPER: Flags = Flags::new(0x0020);
    pub const INTERFACE: Flags = Flags::new(0x0200);
    pub const ABSTRACT: Flags = Flags::new(0x0400);
    pub const SYNTHETIC: Flags = Flags::new(0x1000);
    pub const ANNOTATION: Flags = Flags::new(0x2000);
    pub const ENUM: Flags = Flags::new(0x4000);
    pub const MODULE: Flags = Flags::new(0x8000);

    const NAMES: [(Flags, &'static str); 9] = [
        (Self::PUBLIC, "PUBLIC"),
        (Self::FINAL, "FINAL"),
        (Self::SUPER, "SUPER"),
        (Self::INTERFACE, "INTERFACE"),
        (Self::ABSTRACT, "ABSTRACT"),
        (Self::SYNTHETIC, "SYNTHETIC"),
        (Self::ANNOTATION, "ANNOTATION"),
        (Self::ENUM, "ENUM"),
        (Self::MODULE, "MODULE"),
    ];

    pub fn new(flags: Flags) -> Result<Self, FlagsError> {
        if !flags.has(Self::MODULE) {
            if flags.has(Self::INTERFACE) {
                if !flags.has(Self::ABSTRACT) {
                    return Err(FlagsError::InvalidCombination);
                }

                if flags.has_any(Self::FINAL | Self::SUPER | Self::ENUM | Self::MODULE) {
                    return Err(FlagsError::InvalidCombination);
                }
            } else {
                if flags.has(Self::ANNOTATION) {
                    return Err(FlagsError::InvalidCombination);
                }

                if !flags.has_at_most_one(Self::FINAL | Self::ABSTRACT) {
                    return Err(FlagsError::InvalidCombination);
                }
            }
        }

        Ok(ClassFlags { flags })
    }

    pub fn has(&self, flags: Flags) -> bool {
        self.flags.has(flags)
    }

    pub fn has_any(&self, flags: Flags) -> bool {
        self.flags.has_any(flags)
    }
}

impl fmt::Debug for ClassFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Self::NAMES
            .iter()
            .filter_map(|(flag, name)| {
                if self.flags.has(*flag) {
                    Some(name)
                } else {
                    None
                }
            });

        write!(f, "MethodFlags( {} )", flags.join(" | "))
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MethodType {
    Normal,
    Init,
    ClassInit
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MethodOwner {
    Class,
    Interface
}


#[derive(Copy, Clone)]
pub struct MethodFlags {
    flags: Flags
}

impl MethodFlags {
    pub const PUBLIC: Flags = Flags::new(0x0001);
    pub const PRIVATE: Flags = Flags::new(0x0002);
    pub const PROTECTED: Flags = Flags::new(0x0004);
    pub const STATIC: Flags = Flags::new(0x0008);
    pub const FINAL: Flags = Flags::new(0x0010);
    pub const SYNCHRONIZED: Flags = Flags::new(0x0020);
    pub const BRIDGE: Flags = Flags::new(0x0040);
    pub const VARARGS: Flags = Flags::new(0x0080);
    pub const NATIVE: Flags = Flags::new(0x0100);
    pub const ABSTRACT: Flags = Flags::new(0x0400);
    pub const STRICT: Flags = Flags::new(0x0800);
    pub const SYNTHETIC: Flags = Flags::new(0x1000);

    const NAMES: [(Flags, &'static str); 12] = [
        (Self::PUBLIC, "PUBLIC"),
        (Self::PRIVATE, "PRIVATE"),
        (Self::PROTECTED, "PROTECTED"),
        (Self::STATIC, "STATIC"),
        (Self::FINAL, "FINAL"),
        (Self::SYNCHRONIZED, "SYNCHRONIZED"),
        (Self::BRIDGE, "BRIDGE"),
        (Self::VARARGS, "VARARGS"),
        (Self::NATIVE, "NATIVE"),
        (Self::ABSTRACT, "ABSTRACT"),
        (Self::STRICT, "STRICT"),
        (Self::SYNTHETIC, "SYNTHETIC"),
    ];

    pub fn new(flags: Flags, method_owner: MethodOwner, method_type: MethodType) -> Result<Self, FlagsError> {
        if flags.has(Self::ABSTRACT) {
            if flags.has_any(Self::PRIVATE | Self::STATIC | Self::FINAL | Self::SYNCHRONIZED | Self::NATIVE | Self::STRICT) {
                return Err(FlagsError::InvalidCombination)
            }
        }

        match method_owner {
            MethodOwner::Class => {
                if !flags.has_at_most_one(Self::PUBLIC | Self::PRIVATE | Self::PROTECTED) {
                    return Err(FlagsError::InvalidCombination)
                }
            },
            MethodOwner::Interface => {
                if flags.has_any(Self::PROTECTED | Self::FINAL | Self::SYNCHRONIZED | Self::NATIVE) {
                    return Err(FlagsError::InvalidCombination)
                }

                // version >= 52.0
                if !flags.has_one(Self::PUBLIC | Self::PRIVATE) {
                    return Err(FlagsError::InvalidCombination)
                }

                // version < 52.0
                // if !flags.has(Self::PUBLIC | Self::ABSTRACT) {
                //     return Err(FlagsError::InvalidCombination)
                // }
            },
        }

        match method_type {
            MethodType::Init => {
                if !flags.has_at_most_one(Self::PUBLIC | Self::PRIVATE | Self::PROTECTED) {
                    return Err(FlagsError::InvalidCombination)
                }

                if flags.has_any(Self::STATIC | Self::FINAL | Self::SYNCHRONIZED | Self::BRIDGE | Self::NATIVE ) {
                    return Err(FlagsError::InvalidCombination)
                }
            },
            MethodType::ClassInit => {
                // version >= 51.0
                if !flags.has(Self::STATIC) {
                    return Err(FlagsError::InvalidCombination)
                }
            },
            _ => {}
        }


        Ok(MethodFlags { flags })
    }

    pub fn has(&self, flags: Flags) -> bool {
        self.flags.has(flags)
    }

    pub fn has_any(&self, flags: Flags) -> bool {
        self.flags.has_any(flags)
    }
}

impl fmt::Debug for MethodFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Self::NAMES
            .iter()
            .filter_map(|(flag, name)| {
                if self.flags.has(*flag) {
                    Some(name)
                } else {
                    None
                }
            });

        write!(f, "MethodFlags( {} )", flags.join(" | "))
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FieldOwner {
    Class,
    Interface
}


#[derive(Copy, Clone)]
pub struct FieldFlags {
    flags: Flags
}

impl FieldFlags {
    pub const PUBLIC: Flags = Flags::new(0x0001);
    pub const PRIVATE: Flags = Flags::new(0x0002);
    pub const PROTECTED: Flags = Flags::new(0x0004);
    pub const STATIC: Flags = Flags::new(0x0008);
    pub const FINAL: Flags = Flags::new(0x0010);
    pub const VOLATILE: Flags = Flags::new(0x0040);
    pub const TRANSIENT: Flags = Flags::new(0x0080);
    pub const SYNTHETIC: Flags = Flags::new(0x1000);
    pub const ENUM: Flags = Flags::new(0x4000);

    const NAMES: [(Flags, &'static str); 9] = [
        (Self::PUBLIC, "PUBLIC"),
        (Self::PRIVATE, "PRIVATE"),
        (Self::PROTECTED, "PROTECTED"),
        (Self::STATIC, "STATIC"),
        (Self::FINAL, "FINAL"),
        (Self::VOLATILE, "VOLATILE"),
        (Self::TRANSIENT, "TRANSIENT"),
        (Self::SYNTHETIC, "SYNTHETIC"),
        (Self::ENUM, "ENUM"),
    ];

    pub fn new(flags: Flags, field_owner: FieldOwner) -> Result<Self, FlagsError> {
        if !flags.has_at_most_one(Self::PUBLIC | Self::PRIVATE | Self::PROTECTED) {
            return Err(FlagsError::InvalidCombination)
        }

        if !flags.has_at_most_one(Self::FINAL | Self::VOLATILE) {
            return Err(FlagsError::InvalidCombination)
        }

        if field_owner == FieldOwner::Interface {
            if !flags.has(Self::PUBLIC | Self::STATIC | Self::FINAL) {
                return Err(FlagsError::InvalidCombination)
            }

            if !flags.has_any(Self::PRIVATE | Self::PROTECTED | Self::VOLATILE | Self::TRANSIENT | Self::ENUM) {
                return Err(FlagsError::InvalidCombination)
            }
        }

        Ok(FieldFlags{ flags })
    }

    pub fn has(&self, flags: Flags) -> bool {
        self.flags.has(flags)
    }

    pub fn has_any(&self, flags: Flags) -> bool {
        self.flags.has_any(flags)
    }
}

impl fmt::Debug for FieldFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flags = Self::NAMES
            .iter()
            .filter_map(|(flag, name)| {
                if self.flags.has(*flag) {
                    Some(name)
                } else {
                    None
                }
            });

        write!(f, "FieldFlags( {} )", flags.join(" | "))
    }
}
