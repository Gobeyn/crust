// TODO: Add a way to search for $HOME/.config/crust/crust.toml for user defined configuration
// instead of the default, along with a way to use that user defined configuration.

// External crates
extern crate ratatui;
use ratatui::prelude::*;

// Rose Pine Moon colors
const OVERLAY: Color = Color::Rgb(57, 53, 82);
const MUTED: Color = Color::Rgb(110, 106, 135);
const LOVE: Color = Color::Rgb(235, 111, 146);
const GOLD: Color = Color::Rgb(246, 193, 119);
const ROSE: Color = Color::Rgb(234, 154, 151);
const PINE: Color = Color::Rgb(62, 143, 176);
const FOAM: Color = Color::Rgb(156, 207, 216);
const IRIS: Color = Color::Rgb(196, 167, 231);
const GOLD_DARK: Color = Color::Rgb(151, 92, 10);

/// Color configuration for the UI elements.
#[derive(Debug, Clone)]
pub struct Config {
    pub calendar_title: Color,
    pub calendar_box: Color,
    pub calendar_month_title: Color,
    pub calendar_month_box: Color,
    pub calendar_days_of_week: Color,
    pub calendar_days_of_week_bg: Color,
    pub calendar_day: Color,
    pub calendar_day_bg: Color,
    pub calendar_day_with_entry: Color,
    pub calendar_day_with_entry_bg: Color,
    pub calendar_day_selected: Color,
    pub calendar_day_selected_bg: Color,
    pub agenda_title: Color,
    pub agenda_box: Color,
    pub agenda_entry_title: Color,
    pub agenda_entry_box: Color,
    pub agenda_entry_full_day_event: Color,
    pub agenda_entry_timed_event: Color,
}

impl Default for Config {
    /// Default colorscheme uses Rose Pine Moon colors.
    fn default() -> Self {
        Config {
            calendar_title: PINE,
            calendar_box: FOAM,
            calendar_month_title: LOVE,
            calendar_month_box: PINE,
            calendar_days_of_week: ROSE,
            calendar_days_of_week_bg: MUTED,
            calendar_day: GOLD,
            calendar_day_bg: OVERLAY,
            calendar_day_with_entry: GOLD_DARK,
            calendar_day_with_entry_bg: IRIS,
            calendar_day_selected: GOLD_DARK,
            calendar_day_selected_bg: ROSE,
            agenda_title: PINE,
            agenda_box: FOAM,
            agenda_entry_title: LOVE,
            agenda_entry_box: PINE,
            agenda_entry_full_day_event: PINE,
            agenda_entry_timed_event: IRIS,
        }
    }
}
