use ratatui::{style::Style, widgets::Widget};

pub struct Scrollbar
{
    pub scrolled_amount: usize,
    pub total_amount: usize,
    pub style: Style,
}

impl Scrollbar
{
    pub fn new(scrolled_amount: usize, total_amount: usize, style: Style) -> Self
    {
        Self
        {
            scrolled_amount,
            total_amount,
            style,
        }
    }
}

impl Widget for Scrollbar
{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized 
    {
        if area.height == 0
        {
            return;
        }
        let handle_size = ((area.height as usize) / self.total_amount).clamp(1, area.height as usize);
        let handle_start = (self.scrolled_amount as isize * area.height as isize / self.total_amount as isize - handle_size as isize / 2)
            .clamp(0, area.height as isize - handle_size as isize) as usize;
        for y in area.top()..area.bottom()
        {
            if (y as usize) >= handle_start && (y as usize) < handle_start + handle_size
            {
                buf.get_mut(area.left(), y).set_char('█');
                buf.get_mut(area.left(), y).set_style(self.style);
            }
            else
            {
                buf.get_mut(area.left(), y).set_char(' ');
                buf.get_mut(area.left(), y).set_style(self.style);
            }
        }
    }
}