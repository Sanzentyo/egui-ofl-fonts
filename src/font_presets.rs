#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFont {
    KiwiMaruRegular,
    KiwiMaruMedium,
    KiwiMaruLight,
    HachiMaruPopRegular,
}

impl BuiltinFont {
    pub const fn as_name(self) -> &'static str {
        match self {
            BuiltinFont::KiwiMaruRegular => "KiwiMaru-Regular",
            BuiltinFont::KiwiMaruMedium => "KiwiMaru-Medium",
            BuiltinFont::KiwiMaruLight => "KiwiMaru-Light",
            BuiltinFont::HachiMaruPopRegular => "HachiMaruPop-Regular",
        }
    }
}
