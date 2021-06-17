/*
 * @Date: 2021-06-17 07:34:47
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-06-17 08:06:12
 */

fn is_number(s: String) -> bool {
    for ch in s.chars() {
        if ch.is_alphabetic() && (ch != 'e' && ch != 'E') {
            return false;
        }
    }
    match s.parse::<f32>().ok() {
        Some(_f) => true,
        _ => false,
    }
}

fn is_number_auto(s: String) -> bool {
    s.chars()
        .try_fold(State::new(), State::handle)
        .as_ref()
        .map_or(false, State::is_valid)
}

type Result = std::result::Result<State, ()>;

enum State {
    Start,
    Sign,
    Integer,
    Dot,
    EmptyDot,
    Decimal,
    E,
    ExpSign,
    Exponent,
    End,
}

impl State {
    pub fn new() -> Self {
        State::Start
    }

    pub fn is_valid(&self) -> bool {
        use State::*;
        match self {
            Start | Sign | E | ExpSign | EmptyDot => false,
            _ => true,
        }
    }

    pub fn handle(self, c: char) -> Result {
        use State::*;
        match self {
            Start => match c {
                ' ' => Ok(Start),
                '+' | '-' => Ok(Sign),
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            },
            Sign => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(EmptyDot),
                _ => Err(()),
            },
            Integer => match c {
                '0'..='9' => Ok(Integer),
                '.' => Ok(Dot),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            },
            EmptyDot => match c {
                '0'..='9' => Ok(Decimal), // "  .1" or "  +.1"
                _ => Err(()),
            },
            Dot => match c {
                '0'..='9' => Ok(Decimal),
                'e' | 'E' => Ok(E), // "46.e3"
                ' ' => Ok(End),
                _ => Err(()),
            },
            Decimal => match c {
                '0'..='9' => Ok(Decimal),
                'e' | 'E' => Ok(E),
                ' ' => Ok(End),
                _ => Err(()),
            },
            E => match c {
                '+' | '-' => Ok(ExpSign),
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            },
            ExpSign => match c {
                '0'..='9' => Ok(Exponent),
                _ => Err(()),
            },
            Exponent => match c {
                '0'..='9' => Ok(Exponent),
                ' ' => Ok(End),
                _ => Err(()),
            },
            End => match c {
                ' ' => Ok(End),
                _ => Err(()),
            },
        }
    }
}

fn main() {
    assert!(is_number("0".to_string()));
    assert!(!is_number("e".to_string()));
    assert!(!is_number(".".to_string()));
    assert!(is_number(".1".to_string()));
    assert!(!is_number("inf".to_string()));
    assert!(is_number("1E9".to_string()));
    assert!(is_number_auto("0".to_string()));
    assert!(!is_number_auto("e".to_string()));
    assert!(!is_number_auto(".".to_string()));
    assert!(is_number_auto(".1".to_string()));
    assert!(!is_number_auto("inf".to_string()));
    assert!(is_number_auto("1E9".to_string()));
}
