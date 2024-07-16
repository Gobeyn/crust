// TODO: Add by week and by month movements while the UI is running.
// TODO: Let the key that needs to be pressed be handled by the configuration.

extern crate crossterm;
extern crate ratatui;

/// Descriptors for possible key events.
pub enum KeyEvents {
    NoEvent,
    Quit,
    Next,
    Previous,
}
/// Get key press event if special character is pressed.
///
/// Collect events via `crossterm`, determine if that event is a key press and return a
/// `KeyEvents` code for special characters:
/// - q: `Quit`,
/// - n: `Next`,
/// - p: `Previous`,
/// - _: `NoEvent`
/// If an error occurs when obtaining events, `NoEvent` is also returned.
pub fn get_key_event() -> KeyEvents {
    // Get events every 50 ms, return None if an error occurred.
    match crossterm::event::poll(std::time::Duration::from_millis(50)) {
        Ok(_) => {
            // Read the event that occurred.
            let event_read = match crossterm::event::read() {
                Ok(event) => event,
                Err(_) => {
                    return KeyEvents::NoEvent;
                }
            };
            // Check if the event was a key-related event and store that event in `key`.
            if let crossterm::event::Event::Key(key) = event_read {
                // Check that the key-related event was a key press.
                if key.kind == crossterm::event::KeyEventKind::Press {
                    // Return KeyEvents code for special key-presses.
                    if key.code == crossterm::event::KeyCode::Char('q') {
                        return KeyEvents::Quit;
                    } else if key.code == crossterm::event::KeyCode::Char('n') {
                        return KeyEvents::Next;
                    } else if key.code == crossterm::event::KeyCode::Char('p') {
                        return KeyEvents::Previous;
                    } else {
                        return KeyEvents::NoEvent;
                    }
                } else {
                    return KeyEvents::NoEvent;
                }
            } else {
                return KeyEvents::NoEvent;
            }
        }
        Err(_) => {
            return KeyEvents::NoEvent;
        }
    }
}
