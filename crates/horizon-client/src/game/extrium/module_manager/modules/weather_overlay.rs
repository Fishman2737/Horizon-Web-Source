use super::super::{Category, ModuleDefinition, SettingDefinition, SettingKind};

pub(super) const DEFINITION: ModuleDefinition = ModuleDefinition {
    name: "Weather Overlay",
    category: Category::Render,
    description: "Preview a compact weather status overlay.",
    settings: &[
        SettingDefinition { name: "Show weather", kind: SettingKind::Toggle { default: true } },
        SettingDefinition { name: "Opacity", kind: SettingKind::Slider { min: 0, max: 100, default: 80 } },
    ],
};