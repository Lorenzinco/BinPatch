use ratatui::style::{Color, Modifier, Style};

use super::App;

pub struct ColorSettings
{
    pub address_selected: Style,
    pub address_default: Style,

    pub hex_selected: Style,
    pub hex_null: Style,
    pub hex_alphanumeric: Style,
    pub hex_symbol: Style,
    pub hex_end_of_line: Style,
    pub hex_whitespace: Style,
    pub hex_default: Style,
    pub current_instruction: Style,

    pub text_selected: Style,

    pub assembly_selected: Style,
    pub assembly_address: Style,
    pub assembly_nop: Style,
    pub assembly_bad: Style,
    pub assembly_operands: Style,
    pub assembly_default: Style,

    pub help_command: Style,
    pub hep_description: Style,

    pub yes: Style,
    pub yes_selected: Style,
    pub no: Style,
    pub no_selected: Style,
    pub ok: Style,
    pub ok_selected: Style,
}

impl Default for ColorSettings
{
    fn default() -> Self
    {
        Self
        {
            address_selected: Style::default().fg(Color::Black).bg(Color::White),
            address_default: Style::default().fg(Color::DarkGray),

            hex_selected: Style::default().fg(Color::Black).bg(Color::White),
            hex_null: Style::default().fg(Color::DarkGray),
            hex_alphanumeric: Style::default().fg(Color::Rgb(204, 152, 113)),
            hex_symbol: Style::default().fg(Color::Rgb(204, 152, 113)).add_modifier(Modifier::DIM),
            hex_end_of_line: Style::default().fg(Color::LightRed),
            hex_whitespace: Style::default().fg(Color::Rgb(244, 202, 183)),
            hex_default: Style::default(),
            current_instruction: Style::default().bg(Color::LightGreen),

            text_selected: Style::default().fg(Color::Black).bg(Color::White),

            assembly_selected: Style::default().fg(Color::Rgb(255, 100, 113)).bg(Color::White),
            assembly_address: Style::default().fg(Color::DarkGray),
            assembly_nop: Style::default().fg(Color::DarkGray),
            assembly_bad: Style::default().fg(Color::LightRed),
            assembly_default: Style::default().fg(Color::Rgb(204, 152, 113)),
            assembly_operands: Style::default(),

            help_command: Style::default().fg(Color::LightGreen),
            hep_description: Style::default().fg(Color::Gray),

            yes: Style::default().fg(Color::Green),
            yes_selected: Style::default().fg(Color::Black).bg(Color::Green),
            no: Style::default().fg(Color::Red),
            no_selected: Style::default().fg(Color::Black).bg(Color::Red),
            ok: Style::default().fg(Color::White),
            ok_selected: Style::default().fg(Color::Black).bg(Color::White),
        }
    }
}

impl <'a> App<'a>
{
    pub(super) fn get_style_for_byte(color_settings: &ColorSettings, byte: u8) -> Style
    {
        match byte
        {
            // null
            0x00 => color_settings.hex_null,
            // newline
            0x0A | 0x0C | 0x0D => color_settings.hex_end_of_line,
            // whitespace
            0x20 | 0x09 | 0x0B => color_settings.hex_whitespace,
            // numbers
            0x30..=0x39 |
            // uppercase
            0x41..=0x5A |
            // lowercase
            0x61..=0x7A => color_settings.hex_alphanumeric,
            // special characters
            0x20..=0x7E => color_settings.hex_symbol,
            _ => color_settings.hex_default
        }
    }
}
