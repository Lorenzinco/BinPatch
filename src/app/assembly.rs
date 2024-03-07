use iced_x86::Instruction;
use ratatui::text::{Line, Span, Text};

use super::{app::App, color_settings::ColorSettings, cursor_position::CursorPosition, elf::{Elf, ElfHeader}};

impl <'a> App<'a>
{
    fn instruction_to_line(color_settings: &ColorSettings, instruction: &Instruction, selected: bool) -> Line<'a>
    {
        let mut line = Line::default();
        line.spans.push(Span::styled(format!("{:16X}",instruction.ip()), color_settings.assembly_address));
        line.spans.push(Span::raw(" "));
        let instruction_string = instruction.to_string();
        let mut instruction_pieces = instruction_string.split_whitespace();
        let mnemonic = instruction_pieces.next().unwrap().to_string();
        let args = instruction_pieces.collect::<Vec<&str>>().join(" ");
        let mnemonic_style =
        match instruction.mnemonic() {
            iced_x86::Mnemonic::Nop => color_settings.assembly_nop,
            iced_x86::Mnemonic::INVALID => color_settings.assembly_bad,
            _ => color_settings.assembly_default,
        };

        line.spans.push(Span::styled(mnemonic,
            if selected
            {
                color_settings.assembly_selected
            }
            else
            {
                mnemonic_style
            }
        ));
        line.spans.push(Span::styled(" ",
            if selected
            {
                color_settings.assembly_selected
            }
            else
            {
                color_settings.assembly_default
        }));
        line.spans.push(Span::styled(args,
            if selected
            {
                color_settings.assembly_selected
            }
            else
            {
                color_settings.assembly_operands
            }));
        line
    }

    pub(super) fn assembly_from_bytes(color_settings: &ColorSettings, bytes: &[u8]) -> (Text<'a>, Vec<usize>, Vec<Instruction>)
    {
        let mut output = Text::default();
        let mut line_offsets = vec![0; bytes.len()];

        //let elf : Elf = Elf::parse_elf(bytes); TODO
        let header = ElfHeader::parse_header(bytes).unwrap_or(ElfHeader::default());
        let mut instructions = Vec::new();

        let decoder = iced_x86::Decoder::new(header.bitness.to_num_bits(), bytes, iced_x86::DecoderOptions::NONE);
        let mut byte_index = 0;
        let mut line_index = 0;
        for instruction in decoder {

            instructions.push(instruction);
            let line = Self::instruction_to_line(color_settings, &instruction, line_index == 0);

            for _ in 0..instruction.len() {
                line_offsets[byte_index] = line_index;
                byte_index += 1;
            }
            line_index += 1;
            output.lines.push(line);
        }
        (output, line_offsets, instructions)
    }

    pub(super) fn update_assembly_scroll(&mut self)
    {
        let cursor_position = self.get_cursor_position();
        let current_ip = cursor_position.global_byte_index.min(self.assembly_offsets.len() - 1);
        let current_scroll = self.assembly_offsets[current_ip];

        self.assembly_view.lines[self.assembly_scroll].spans[0].style = self.color_settings.assembly_address;
        self.assembly_view.lines[self.assembly_scroll].spans[1].style = self.color_settings.assembly_default;
        self.assembly_view.lines[self.assembly_scroll].spans[2].style = self.color_settings.assembly_default;

        for i in 3..self.assembly_view.lines[self.assembly_scroll].spans.len(){
            self.assembly_view.lines[self.assembly_scroll].spans[i].style = self.color_settings.assembly_operands;
        }

        for i in 2..self.assembly_view.lines[current_scroll].spans.len(){
            self.assembly_view.lines[current_scroll].spans[i].style = self.color_settings.assembly_selected;
        }
        self.assembly_scroll = current_scroll;
    }

    pub(super) fn get_assembly_view_scroll(&self) -> usize
    {
        let visible_lines = self.screen_size.1 - 3;
        let center_of_view = visible_lines / 2;
        let view_scroll = (self.assembly_scroll as isize - center_of_view as isize).clamp(0, (self.assembly_view.lines.len() as isize - visible_lines as isize).max(0));

        view_scroll as usize
    }

    pub(super) fn edit_assembly(&mut self)
    {
        (self.assembly_view, self.assembly_offsets, self.instructions) = Self::assembly_from_bytes(&self.color_settings, &self.data);
        self.assembly_scroll = 0;
        self.update_assembly_scroll();
    }



}
