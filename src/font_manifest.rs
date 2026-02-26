#![allow(dead_code)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LicenseKind {
    Ofl,
}

#[derive(Clone, Copy, Debug)]
pub struct FontFamilySpec {
    pub display_name: &'static str,
    pub license: LicenseKind,
    pub google_ofl_dir: &'static str,
    pub local_family_dir: &'static str,
    pub specimen_url: &'static str,
    pub repo_url: &'static str,
}

macro_rules! define_font_catalog {
    (
        $(
            family {
                display_name: $display_name:literal,
                license: $license:ident,
                google_ofl_dir: $google_ofl_dir:literal,
                local_family_dir: $local_family_dir:literal,
                specimen_url: $specimen_url:literal,
                repo_url: $repo_url:literal,
            }
        )+
    ) => {
        pub const FONT_FAMILIES: &[FontFamilySpec] = &[
            $(
                FontFamilySpec {
                    display_name: $display_name,
                    license: LicenseKind::$license,
                    google_ofl_dir: $google_ofl_dir,
                    local_family_dir: $local_family_dir,
                    specimen_url: $specimen_url,
                    repo_url: $repo_url,
                },
            )+
        ];
    };
}

define_font_catalog! {
    family {
        display_name: "Kiwi Maru",
        license: Ofl,
        google_ofl_dir: "kiwimaru",
        local_family_dir: "Kiwi_Maru",
        specimen_url: "https://fonts.google.com/specimen/Kiwi+Maru",
        repo_url: "https://github.com/google/fonts/tree/main/ofl/kiwimaru",
    }

    family {
        display_name: "Hachi Maru Pop",
        license: Ofl,
        google_ofl_dir: "hachimarupop",
        local_family_dir: "Hachi_Maru_Pop",
        specimen_url: "https://fonts.google.com/specimen/Hachi+Maru+Pop",
        repo_url: "https://github.com/google/fonts/tree/main/ofl/hachimarupop",
    }
}