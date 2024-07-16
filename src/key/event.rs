// TODO: Add by week and by month movements while the UI is running.

extern crate crossterm;
extern crate ratatui;

// Local files
use crate::configuration::config;

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
/// `KeyEvents` code for special characters determined by the configuration. If a key is pressed
/// with no attached functionality, or an error occurs `NoEvent` is returned.
pub fn get_key_event(conf: &config::Config) -> KeyEvents {
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
                    if key.code == crossterm::event::KeyCode::Char(conf.key_quit) {
                        return KeyEvents::Quit;
                    } else if key.code == crossterm::event::KeyCode::Char(conf.key_next) {
                        return KeyEvents::Next;
                    } else if key.code == crossterm::event::KeyCode::Char(conf.key_previous) {
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
