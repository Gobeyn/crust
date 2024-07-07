extern crate crossterm;
extern crate ratatui;

pub fn handle_events() -> std::io::Result<bool> {
    if crossterm::event::poll(std::time::Duration::from_millis(50))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press
                && key.code == crossterm::event::KeyCode::Char('q')
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
