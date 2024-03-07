use super::App;

pub struct CursorPosition
{
    pub local_x: usize,
    pub local_byte_index: usize,
    pub block_index: usize,
    pub local_block_index: usize,
    pub line_index: usize,
    pub line_byte_index: usize,
    pub global_byte_index: usize,
    pub high_byte: bool,
}

impl CursorPosition
{
    pub fn get_high_byte_offset(&self) -> usize
    {
        match &self.high_byte {
            true => 0,
            false => 1,
        }
    }
}

impl <'a> App<'a>
{
    pub(super) fn get_cursor_position(&self) -> CursorPosition
    {
        let local_x = self.cursor.0 as usize % (self.block_size * 3 + 1);
        let high_byte = local_x % 3 == 0;
        let local_byte_index = local_x / 3;
        let block_index = self.cursor.0 as usize / (self.block_size * 3 + 1) + (self.scroll + self.cursor.1 as usize) * self.blocks_per_row;
        let local_block_index = block_index % self.blocks_per_row;
        let line_index = block_index / self.blocks_per_row;
        let line_byte_index = local_byte_index + self.block_size * local_block_index;
        let global_byte_index = line_byte_index + line_index * self.block_size * self.blocks_per_row;

        CursorPosition {
            local_x,
            local_byte_index,
            block_index,
            local_block_index,
            line_index,
            line_byte_index,
            global_byte_index,
            high_byte,
        }
    }

    pub(super) fn move_cursor(&mut self, dx: isize, dy: isize)
    {
        // TODO: check that the cursor does not overflow the data
        let (x, y) = self.cursor;
        let mut x = x as isize + dx;
        let mut y = y as isize + dy;

        let view_size_y = self.screen_size.1 - 3;

        let viewed_block_size = (self.block_size * 3 + 1) as isize;
        let viewed_line_size = viewed_block_size * self.blocks_per_row as isize + self.blocks_per_row as isize - 3;

        let block_count = x / viewed_block_size;
        let local_x = x - block_count * viewed_block_size;
        if local_x % 3 == 2
        {
            x += dx.signum();
        }
        while x % viewed_block_size == viewed_block_size - 1 || x % viewed_block_size == viewed_block_size - 2
        {
            x += dx.signum();
        }

        if x < 0
        {
            if self.scroll > 0 || y > 0
            {
                x = viewed_line_size - self.blocks_per_row as isize;
                y -= 1;
            }
            else
            {
                x = 0;
            }
        }
        else if x > viewed_line_size - self.blocks_per_row as isize
        {
            x = 0;
            y += 1;
        }
        if y >= (self.hex_view.lines.len() as isize - 1)
        {
            y = self.hex_view.lines.len() as isize - 1;
        }
        if y < 0
        {
            y = 0;
            if self.scroll > 0
            {
                self.scroll -= 1;
            }
        }
        else if y >= view_size_y as isize
        {
            y = view_size_y as isize - 1;
            if self.scroll < self.hex_view.lines.len() - view_size_y as usize
            {
                self.scroll += 1;
            }
        }

        let data_len = self.data.len() as isize;
        let bytes_per_row = self.block_size as isize * self.blocks_per_row as isize;
        let characters_in_last_row = (data_len % bytes_per_row) * 3 + (data_len % bytes_per_row) / self.block_size as isize - 2;
        if y + self.scroll as isize == data_len / bytes_per_row
        {
            x = x.min(characters_in_last_row);
        }

        self.cursor = (x as u16, y as u16);

        self.update_cursors();
    }

    pub(super) fn print_cursor_position(&mut self)
    {
        let cursor_position = self.get_cursor_position();
        self.output = format!("{:16X} {}", cursor_position.global_byte_index, if cursor_position.high_byte { "H" } else { "L" });
    }

    pub(super) fn move_cursor_page_up(&mut self)
    {
        if self.scroll == 0
        {
            self.cursor.1 = 0;
        }
        self.scroll = self.scroll.saturating_sub((self.screen_size.1 - 3) as usize);
        self.update_cursors();
    }

    pub(super) fn move_cursor_page_down(&mut self)
    {
        if self.scroll == self.hex_view.lines.len() - (self.screen_size.1 - 3) as usize
        {
            self.cursor.1 = (self.hex_view.lines.len() % self.screen_size.1 as usize - 3) as u16;
        }
        self.scroll = (self.scroll + (self.screen_size.1 - 3) as usize).min(self.hex_view.lines.len() - (self.screen_size.1 - 3) as usize);
        self.update_cursors();
    }

    pub(super) fn move_cursor_to_end(&mut self)
    {
        self.scroll = (self.hex_view.lines.len() as isize- (self.screen_size.1 - 3) as isize).max(0) as usize;
        let x = self.blocks_per_row as u16 * 3 * self.block_size as u16 + self.blocks_per_row as u16 - 3;
        let y = (self.screen_size.1 - 4).min(self.hex_view.lines.len() as u16 - 1);
        self.cursor = (x, y);
        self.update_cursors();
    }

    pub(super) fn move_cursor_to_start(&mut self)
    {
        self.cursor = (0, 0);
        self.scroll = 0;
        self.update_cursors();
    }

    pub(super) fn update_cursors(&mut self)
    {
        self.print_cursor_position();
        self.update_address_cursor();
        self.update_hex_cursor();
        self.update_text_cursor();
        self.update_assembly_scroll();
    }
}
