extern crate crossterm;
extern crate ratatui;

pub struct KeyEvents {
    pub quit: bool,
    pub next_entry: bool,
    pub prev_entry: bool,
}

impl Default for KeyEvents {
    fn default() -> Self {
        KeyEvents {
            quit: false,
            next_entry: false,
            prev_entry: false,
        }
    }
}

pub fn get_key_events() -> std::io::Result<KeyEvents> {
    let mut key_events = KeyEvents::default();

    if crossterm::event::poll(std::time::Duration::from_millis(50))? {
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                if key.code == crossterm::event::KeyCode::Char('q') {
                    key_events.quit = true;
                } else if key.code == crossterm::event::KeyCode::Char('n') {
                    key_events.next_entry = true;
                } else if key.code == crossterm::event::KeyCode::Char('p') {
                    key_events.prev_entry = true;
                }
            }
        }
    }
    Ok(key_events)
}
