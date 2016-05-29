#[derive(Debug)]
pub enum Order {
    Alphabetical,
    ReverseAlphabetical,
}

pub fn is_in_order<F>(s: &str, order_function: F) -> bool
    where F: Fn(u8, u8) -> bool
{
    if s.is_empty() {
        return true;
    }

    let mut chars = s.chars().peekable();
    let mut prev = chars.peek().cloned().unwrap() as u8;

    for c in chars {
        let c = c as u8;
        if !order_function(prev, c) {
            return false;
        }
        prev = c;
    }
    true
}

pub fn classify(s: &str) -> Option<Order> {
    if is_in_order(s, |prev, cur| prev <= cur) {
        Some(Order::Alphabetical)
    } else if is_in_order(s, |prev, cur| prev >= cur) {
        Some(Order::ReverseAlphabetical)
    } else {
        None
    }
}

fn main() {
    for word in &["almost", "cereal", "billowy", "biopsy", "chinos", "defaced", "chintz",
                  "sponged", "bijoux", "abhors", "fiddle", "begins", "chimps", "wronged"] {
        if let Some(order) = classify(word) {
            println!("{} IN {:?} ORDER", word, order);
        } else {
            println!("{} NOT IN ORDER", word);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_in_order;

    #[test]
    fn works_for_given_cases() {
        for word in &["almost", "billowy", "biopsy", "chinos", "chintz", "bijoux", "abhors",
                      "begins", "chimps"] {
            assert!(is_in_order(word, |prev, cur| prev <= cur));
        }
        for word in &["sponged", "wronged"] {
            assert!(is_in_order(word, |prev, cur| prev >= cur));
        }
        for word in &["cereal", "defaced", "fiddle"] {
            assert!(!is_in_order(word, |prev, cur| prev <= cur));
            assert!(!is_in_order(word, |prev, cur| prev >= cur));
        }
    }
}
