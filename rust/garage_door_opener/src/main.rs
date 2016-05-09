use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum State {
    CLOSING,
    CLOSED,
    OPENING,
    OPEN,
    STOPPED_WHILE_CLOSING,
    STOPPED_WHILE_OPENING,
}

enum Event {
    ButtonClicked,
    CycleComplete,
}

#[derive(Debug)]
struct EventParseError {}

impl FromStr for Event {
    type Err = EventParseError;
    fn from_str(s: &str) -> Result<Self, EventParseError> {
        match s {
            "button_clicked" => Ok(Event::ButtonClicked),
            "cycle_complete" => Ok(Event::CycleComplete),
            _ => Err(EventParseError {}),
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Event::ButtonClicked => write!(f, "> Button clicked."),
            Event::CycleComplete => write!(f, "> Cycle complete."),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut state = State::CLOSED;

    let print_state = |state: &State| {
        println!("Door: {:?}", state);
    };

    print_state(&state);

    for line in stdin.lock().lines() {
        let event: Event = line.unwrap()
                               .trim()
                               .parse()
                               .expect("Incorrect input");
        println!("{:?}", event);

        state = match event {
            Event::ButtonClicked => {
                match state {
                    State::CLOSED |
                    State::STOPPED_WHILE_CLOSING => State::OPENING,
                    State::OPEN |
                    State::STOPPED_WHILE_OPENING => State::CLOSING,
                    State::CLOSING => State::STOPPED_WHILE_CLOSING,
                    State::OPENING => State::STOPPED_WHILE_OPENING,
                }
            }
            Event::CycleComplete => {
                match state {
                    State::OPENING => State::OPEN,
                    State::CLOSING => State::CLOSED,
                    other => other,
                }
            }
        };

        print_state(&state);
    }
}
