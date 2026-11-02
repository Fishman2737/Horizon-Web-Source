use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Clean Screenshots",
    category: Category::Misc,
    description: "Preview hiding the click GUI in screenshots.",
    settings: &[
        SettingDefinition { name: "Hide click GUI", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Hide HUD", kind: SettingKind::Toggle { default: false } },
    ],
};