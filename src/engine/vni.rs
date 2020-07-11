use super::processor::{
    Action, ToneMark, LetterModification,
    add_tone, remove_tone, modify_letter
};

fn is_number(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false
    }
}

/// Transform input buffer to vietnamese output
///
/// # Example
/// ```
/// transform_buffer(vec!['v', 'i', 'e', 't', '6', '5'])
/// // output: 'việt'
/// ```
pub fn transform_buffer(buffer: &Vec<char>) -> String {
    let mut content = String::new();
    let mut actions: Vec<Action> = Vec::new();
    for ch in buffer {
        if is_number(*ch) {
            // in vni, number denote an action like adding tone mark, remove
            // tone mark and changing letter to modified vietnamese letter.
            match ch {
                '1' => actions.push(Action::AddTone(ToneMark::Acute)),
                '2' => actions.push(Action::AddTone(ToneMark::Grave)),
                '3' => actions.push(Action::AddTone(ToneMark::HookAbove)),
                '4' => actions.push(Action::AddTone(ToneMark::Tilde)),
                '5' => actions.push(Action::AddTone(ToneMark::Underdot)),
                '6' => actions.push(Action::ModifyLetter(LetterModification::Circumflex)),
                '7' => actions.push(Action::ModifyLetter(LetterModification::Horn)),
                '8' => actions.push(Action::ModifyLetter(LetterModification::Breve)),
                '9' => actions.push(Action::ModifyLetter(LetterModification::Dyet)),
                '0' => actions.push(Action::RemoveTone),
                _ => {}
            }
        } else {
            content.push(*ch);
        }
    }

    for action in actions {
        match action {
            Action::AddTone(tone_mark) => {
                content = add_tone(&content, tone_mark)
            }
            _ => {}
        }
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_acute_tone_normal() {
        let input: Vec<char> = vec!['v', 'i', 't', '1'];
        let result = transform_buffer(&input);
        let expected = "vít".to_string();
        assert_eq!(result, expected);
    }
}
