#![feature(box_patterns)]

use std::fmt;
use std::io;
use std::io::prelude::*;
use std::str::FromStr;

type Previous = Box<State>;

#[allow(non_camel_case_types)]
#[derive(Debug)]
enum State {
    CLOSING,
    CLOSED,
    OPENING,
    OPEN,
    STOPPED_WHILE_CLOSING,
    STOPPED_WHILE_OPENING,
    EMERGENCY_OPENING,
    Blocked(Previous),
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use State::*;
        match *self {
            CLOSING |
            CLOSED |
            OPENING |
            OPEN |
            STOPPED_WHILE_OPENING |
            STOPPED_WHILE_CLOSING |
            EMERGENCY_OPENING => write!(f, "Door: {:?}", self),
            Blocked(box ref state) => write!(f, "Door: BLOCKED_{:?}", state),
        }
    }
}

enum Event {
    BlockDetected,
    BlockCleared,
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
            "block_detected" => Ok(Event::BlockDetected),
            "block_cleared" => Ok(Event::BlockCleared),
            _ => Err(EventParseError {}),
        }
    }
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Event::*;
        match *self {
            ButtonClicked => write!(f, "> Button clicked."),
            CycleComplete => write!(f, "> Cycle complete."),
            BlockDetected => write!(f, "> Block detected!"),
            BlockCleared => write!(f, "> Block cleared."),
        }
    }
}

fn main() {
    use State::*;
    let stdin = io::stdin();
    let mut state = State::CLOSED;

    println!("{}", state);

    for line in stdin.lock().lines() {
        let event: Event = line.unwrap()
                               .trim()
                               .parse()
                               .expect("Incorrect input");
        println!("{:?}", event);

        state = match event {
            Event::ButtonClicked => {
                match state {
                    CLOSED |
                    STOPPED_WHILE_CLOSING => OPENING,
                    OPEN |
                    STOPPED_WHILE_OPENING => CLOSING,
                    CLOSING => STOPPED_WHILE_CLOSING,
                    OPENING => STOPPED_WHILE_OPENING,
                    other => other, // We are blocked, ignore clicks
                }
            }
            Event::CycleComplete => {
                match state {
                    OPENING => OPEN,
                    CLOSING => CLOSED,
                    EMERGENCY_OPENING => Blocked(Box::new(OPEN)),
                    other => other, // not moving right now
                }
            }
            Event::BlockDetected => {
                match state {
                    CLOSING | OPENING => EMERGENCY_OPENING,
                    other => Blocked(Box::new(other)), // already blocked
                }
            }
            Event::BlockCleared => {
                match state {
                    Blocked(box x) => x,
                    other => other, // not blocked, do nothing
                }
            }
        };
        println!("{}", state);
    }
}
