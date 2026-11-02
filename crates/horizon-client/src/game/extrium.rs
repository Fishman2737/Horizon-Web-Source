use super::{Game, Screen};
use horizon_client::hud::{HudAtlas, HudBatch};
use winit::event::MouseButton;

mod module_manager;
use module_manager::{Category, ModuleManager, SettingKind, SettingState, SettingValue};

const BG: u32 = 0xF5_14_09_0E;
const PANEL: u32 = 0xFF_20_13_18;
const RAISED: u32 = 0xFF_38_1D_2B;
const BORDER: u32 = 0xFF_67_3C_55;
const ACCENT: u32 = 0xFF_BB_83_FF;
const TEXT: u32 = 0xFF_FA_F4_FF;
const MUTED: u32 = 0xFF_AF_97_BA;

pub(super) struct Extrium {
    manager: ModuleManager,
    category: Category,
    scroll: usize,
    settings_module: Option<usize>,
}

impl Default for Extrium {
    fn default() -> Self {
        Self {
            manager: ModuleManager::new(),
            category: Category::Combat,
            scroll: 0,
            settings_module: None,
        }
    }
}

impl Game<'_> {
    pub(super) fn open_extrium(&mut self) {
        self.extrium.settings_module = None;
        self.keys.clear();
        self.left = false;
        self.right = false;
        self.mining = None;
        self.screen = Screen::Extrium;
        self.capture(false);
    }

    pub(super) fn close_extrium(&mut self) {
        self.screen = Screen::Playing;
        self.capture(true);
    }

    pub(super) fn extrium_escape(&mut self) {
        if self.extrium.settings_module.take().is_none() {
            self.close_extrium();
        }
    }

    pub(super) fn extrium_click(&mut self, button: MouseButton) {
        let panel = panel_rect(self.gui_size());
        let (x, y) = self.cursor;
        if close_hover(panel, x, y) && button == MouseButton::Left {
            self.close_extrium();
            return;
        }
        if let Some(index) = self.extrium.settings_module {
            if button == MouseButton::Left && inside(x, y, panel.0 + 202.0, panel.1 + 76.0, 24.0, 32.0) {
                self.extrium.settings_module = None;
                return;
            }
            if button != MouseButton::Left {
                return;
            }
            let row = ((y - (panel.1 + 158.0)) / 54.0).floor() as isize;
            if row == 0 {
                self.extrium.manager.toggle_module(index);
            } else if row > 0 {
                let setting_index = (row - 1) as usize;
                let slider = self
                    .extrium
                    .manager
                    .module(index)
                    .and_then(|module| module.settings.get(setting_index))
                    .is_some_and(|setting| matches!(setting.definition.kind, SettingKind::Slider { .. }));
                if slider {
                    let start = panel.0 + 202.0 + 180.0;
                    self.extrium.manager.set_slider(index, setting_index, (x - start) / 140.0);
                } else {
                    self.extrium.manager.toggle_setting(index, setting_index);
                }
            }
            return;
        }
        if button == MouseButton::Right {
            if let Some(index) = module_at(&self.extrium, panel, x, y) {
                self.extrium.settings_module = Some(index);
            }
            return;
        }
        if button != MouseButton::Left {
            return;
        }
        if let Some(category) = Category::ALL.into_iter().find(|category| {
            inside(x, y, panel.0 + 16.0, tab_y(panel, *category), 152.0, 34.0)
        }) {
            self.extrium.category = category;
            self.extrium.scroll = 0;
            return;
        }
        if let Some(index) = module_at(&self.extrium, panel, x, y) {
            if x >= panel.0 + panel.2 - 88.0 {
                self.extrium.settings_module = Some(index);
            } else {
                self.extrium.manager.toggle_module(index);
            }
        }
    }

    pub(super) fn extrium_scroll(&mut self, delta: f32) {
        let panel = panel_rect(self.gui_size());
        if self.extrium.settings_module.is_some() {
            return;
        }
        let total = self.extrium.manager.category_indices(self.extrium.category).count();
        let visible = ((panel.3 - 194.0).max(52.0) / 58.0).floor() as usize;
        self.extrium.scroll = if delta < 0.0 {
            self.extrium
                .scroll
                .saturating_add(1)
                .min(total.saturating_sub(visible))
        } else {
            self.extrium.scroll.saturating_sub(1)
        };
    }

    pub(super) fn draw_extrium(&mut self, batch: &mut HudBatch) {
        let panel = panel_rect((batch.width, batch.height));
        let (x, y, w, h) = panel;
        let atlas = &self.gui.atlas;
        batch.fill(atlas, 0.0, 0.0, batch.width, batch.height, 0x9100_0000);
        rounded(batch, atlas, x, y, w, h, 12.0, BORDER);
        rounded(batch, atlas, x + 1.0, y + 1.0, w - 2.0, h - 2.0, 11.0, BG);
        rounded(batch, atlas, x + 2.0, y + 2.0, w - 4.0, 62.0, 10.0, PANEL);
        batch.fill(atlas, x + 2.0, y + 58.0, w - 4.0, 7.0, PANEL);
        batch.text(atlas, "Extrium", x + 22.0, y + 22.0, TEXT, true);
        batch.text(atlas, "v1.0.0", x + 94.0, y + 25.0, MUTED, false);
        batch.text(atlas, "RIGHT SHIFT", x + w - 132.0, y + 25.0, MUTED, false);
        batch.text(atlas, "X", x + w - 34.0, y + 23.0, TEXT, false);
        batch.fill(atlas, x + 178.0, y + 66.0, 1.0, h - 68.0, 0xFF_3C_30_48);

        if let Some(index) = self.extrium.settings_module {
            self.draw_settings(batch, panel, index);
        } else {
            self.draw_main(batch, panel);
        }
    }

    fn draw_main(&mut self, batch: &mut HudBatch, panel: (f32, f32, f32, f32)) {
        let (x, y, w, h) = panel;
        let atlas = &self.gui.atlas;
        batch.text(atlas, "CATEGORIES", x + 20.0, y + 89.0, MUTED, false);
        for category in Category::ALL {
            let top = tab_y(panel, category);
            let selected = category == self.extrium.category;
            rounded(batch, atlas, x + 16.0, top, 152.0, 34.0, 6.0, if selected { RAISED } else { PANEL });
            batch.fill(atlas, x + 16.0, top + 5.0, 2.0, 24.0, if selected { ACCENT } else { PANEL });
            batch.text(atlas, category.name(), x + 29.0, top + 11.0, if selected { TEXT } else { MUTED }, false);
        }

        let list_x = x + 202.0;
        batch.text(atlas, self.extrium.category.name(), list_x, y + 88.0, TEXT, true);
        batch.text(atlas, "Modules", list_x, y + 105.0, MUTED, false);
        let items: Vec<_> = self
            .extrium
            .manager
            .category_indices(self.extrium.category)
            .skip(self.extrium.scroll)
            .collect();
        let count = ((h - 194.0).max(52.0) / 58.0).floor() as usize;
        for (row, index) in items.iter().take(count).enumerate() {
            let row_y = y + 132.0 + row as f32 * 58.0;
            let Some(module) = self.extrium.manager.module(*index) else {
                continue;
            };
            rounded(batch, atlas, list_x, row_y, w - 228.0, 52.0, 6.0, PANEL);
            batch.text(atlas, module.definition.name, list_x + 12.0, row_y + 8.0, TEXT, true);
            batch.text(atlas, module.definition.description, list_x + 12.0, row_y + 27.0, MUTED, false);
            draw_switch(batch, atlas, x + w - 66.0, row_y + 17.0, module.enabled);
            batch.text(atlas, ">", x + w - 26.0, row_y + 20.0, MUTED, false);
        }
        batch.text(atlas, "Right-click a module for settings", list_x, y + h - 29.0, MUTED, false);

        if let Some(category) = Category::ALL.into_iter().find(|category| {
            inside(self.cursor.0, self.cursor.1, x + 16.0, tab_y(panel, *category), 152.0, 34.0)
        }) {
            draw_category_tooltip(batch, atlas, category, self.cursor);
        }
    }

    fn draw_settings(&self, batch: &mut HudBatch, panel: (f32, f32, f32, f32), index: usize) {
        let (x, y, w, h) = panel;
        let atlas = &self.gui.atlas;
        let list_x = x + 202.0;
        let Some(module) = self.extrium.manager.module(index) else {
            return;
        };
        batch.text(atlas, "<", list_x, y + 86.0, ACCENT, false);
        batch.text(atlas, "MODULE SETTINGS", list_x + 26.0, y + 82.0, MUTED, false);
        batch.text(atlas, module.definition.name, list_x + 26.0, y + 101.0, TEXT, true);
        batch.text(atlas, module.definition.description, list_x + 26.0, y + 119.0, MUTED, false);
        let enabled_value = if module.enabled { "ON" } else { "OFF" };
        draw_setting_row(batch, atlas, list_x, y + 158.0, w - 228.0, "Enabled", enabled_value, module.enabled);
        for (index, setting) in module.settings.iter().enumerate() {
            let row_y = y + 212.0 + index as f32 * 54.0;
            let (value, active) = setting_value(setting);
            draw_setting_row(batch, atlas, list_x, row_y, w - 228.0, setting.definition.name, &value, active);
            if matches!(setting.definition.kind, SettingKind::Slider { .. }) {
                let SettingValue::Slider(value) = setting.value else { continue };
                let (min, max) = match setting.definition.kind {
                    SettingKind::Slider { min, max, .. } => (min, max),
                    _ => unreachable!(),
                };
                draw_slider(batch, atlas, list_x + 180.0, row_y + 22.0, 140.0, value, min, max);
            }
        }
        batch.text(atlas, "UI preview only; module effects are not connected", list_x, y + h - 28.0, MUTED, false);
    }
}

fn setting_value(setting: &SettingState) -> (String, bool) {
    match (setting.definition.kind, setting.value) {
        (SettingKind::Toggle { .. }, SettingValue::Toggle(value)) => {
            (if value { "ON" } else { "OFF" }.to_owned(), value)
        }
        (SettingKind::Choice { values, .. }, SettingValue::Choice(index)) => (
            values.get(index).copied().unwrap_or("Default").to_owned(),
            false,
        ),
        (SettingKind::Slider { .. }, SettingValue::Slider(value)) => (value.to_string(), false),
        _ => (String::new(), false),
    }
}

fn module_at(state: &Extrium, panel: (f32, f32, f32, f32), x: f32, y: f32) -> Option<usize> {
    let list_x = panel.0 + 202.0;
    let first_y = panel.1 + 132.0;
    if !inside(x, y, list_x, first_y, panel.2 - 228.0, panel.3 - 194.0) {
        return None;
    }
    let row = ((y - first_y) / 58.0).floor() as usize + state.scroll;
    state.manager.category_module(state.category, row)
}

fn panel_rect((width, height): (f32, f32)) -> (f32, f32, f32, f32) {
    let panel_w = width.min(820.0).max(320.0);
    let panel_h = height.min(540.0).max(220.0);
    ((width - panel_w) / 2.0, (height - panel_h) / 2.0, panel_w, panel_h)
}

fn tab_y(panel: (f32, f32, f32, f32), category: Category) -> f32 {
    let index = Category::ALL.iter().position(|candidate| *candidate == category).unwrap_or(0);
    panel.1 + 122.0 + index as f32 * 42.0
}

fn close_hover(panel: (f32, f32, f32, f32), x: f32, y: f32) -> bool {
    inside(x, y, panel.0 + panel.2 - 48.0, panel.1 + 8.0, 40.0, 44.0)
}

fn inside(x: f32, y: f32, left: f32, top: f32, width: f32, height: f32) -> bool {
    x >= left && x < left + width && y >= top && y < top + height
}

fn draw_category_tooltip(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    category: Category,
    cursor: (f32, f32),
) {
    let width = 196.0;
    let x = if cursor.0 + width + 14.0 <= batch.width {
        cursor.0 + 14.0
    } else {
        cursor.0 - width - 14.0
    }
    .clamp(4.0, batch.width - width - 4.0);
    let y = (cursor.1 + 14.0).min(batch.height - 38.0).max(4.0);
    rounded(batch, atlas, x, y, width, 30.0, 5.0, BORDER);
    rounded(batch, atlas, x + 1.0, y + 1.0, width - 2.0, 28.0, 4.0, PANEL);
    batch.text(atlas, category.description(), x + 9.0, y + 11.0, TEXT, false);
}

fn draw_switch(batch: &mut HudBatch, atlas: &HudAtlas, x: f32, y: f32, enabled: bool) {
    rounded(batch, atlas, x, y, 34.0, 18.0, 9.0, if enabled { ACCENT } else { RAISED });
    rounded(
        batch,
        atlas,
        x + if enabled { 18.0 } else { 2.0 },
        y + 2.0,
        14.0,
        14.0,
        7.0,
        TEXT,
    );
}

fn draw_setting_row(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    x: f32,
    y: f32,
    width: f32,
    label: &str,
    value: &str,
    active: bool,
) {
    rounded(batch, atlas, x, y, width, 48.0, 6.0, PANEL);
    batch.text(atlas, label, x + 12.0, y + 17.0, TEXT, false);
    if !value.is_empty() {
        batch.text(atlas, value, x + width - 84.0, y + 17.0, if active { ACCENT } else { MUTED }, false);
    }
}

fn draw_slider(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    x: f32,
    y: f32,
    width: f32,
    value: u8,
    min: u8,
    max: u8,
) {
    let progress = f32::from(value.saturating_sub(min)) / f32::from(max.saturating_sub(min).max(1));
    rounded(batch, atlas, x, y, width, 5.0, 2.0, RAISED);
    rounded(batch, atlas, x, y, width * progress, 5.0, 2.0, ACCENT);
}

fn rounded(
    batch: &mut HudBatch,
    atlas: &HudAtlas,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    radius: f32,
    color: u32,
) {
    let radius = radius.min(width / 2.0).min(height / 2.0).max(0.0);
    if radius == 0.0 {
        batch.fill(atlas, x, y, width, height, color);
        return;
    }
    batch.fill(atlas, x + radius, y, width - 2.0 * radius, height, color);
    for row in 0..radius.ceil() as usize {
        let dy = radius - row as f32 - 0.5;
        let inset = radius - (radius * radius - dy * dy).max(0.0).sqrt();
        batch.fill(atlas, x + inset, y + row as f32, width - 2.0 * inset, 1.0, color);
        batch.fill(
            atlas,
            x + inset,
            y + height - row as f32 - 1.0,
            width - 2.0 * inset,
            1.0,
            color,
        );
    }
}