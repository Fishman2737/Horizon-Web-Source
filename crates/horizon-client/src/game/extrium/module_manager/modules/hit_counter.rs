use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Hit Counter",
    category: Category::Combat,
    description: "Preview a session combat counter in the HUD.",
    settings: &[
        SettingDefinition { name: "Show counter", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Reset on world change", kind: SettingKind::Toggle { default: true } },
    ],
};