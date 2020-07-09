use std::fmt;
use std::fmt::Debug;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};

use derive_more::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use itertools::Itertools;

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

    pub fn new() -> Self {
        ClassFlags{ flags: Flags::empty() }
    }
}

impl From<Flags> for ClassFlags {
    fn from(flags: Flags) -> Self {
        ClassFlags{flags}
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

    pub fn new(flags: Flags) -> Self {
        MethodFlags{ flags }
    }

    pub fn is_static(&self) -> bool {
        self.flags.has(Self::STATIC)
    }
}


impl From<Flags> for MethodFlags {
    fn from(flags: Flags) -> Self {
        MethodFlags{flags}
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

    pub fn new(flags: Flags) -> Self {
        FieldFlags{ flags }
    }

    pub fn is_static(&self) -> bool {
        self.flags.has(Self::STATIC)
    }
}


impl From<Flags> for FieldFlags {
    fn from(flags: Flags) -> Self {
        FieldFlags{flags}
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
