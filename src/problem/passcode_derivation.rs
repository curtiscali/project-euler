use std::collections::{HashMap, HashSet};
use super::Problem;

fn find_key_without_prevs(keymap: &HashMap<char, HashSet<char>>) -> Option<char> {
    for (c, prevs) in keymap {
        if prevs.len() == 0 {
            return Some(*c);
        }
    }

    None
}

fn get_all_references_to_key(keymap: &HashMap<char, HashSet<char>>, ch: char) -> Vec<char> {
    let mut references = vec![];

    for (c, prevs) in keymap {
        if prevs.contains(&ch) {
            references.push(*c);
        }
    }

    references
}

pub struct PasscodeDerivationProblem {}

impl Problem for PasscodeDerivationProblem {
    fn name(&self) -> String {
        String::from("Passcode Derivation")
    }

    fn number(&self) -> u16 {
        79
    }

    fn solve(&self) -> String {
        let bytes = include_bytes!("../data_files/0079_keylog.txt");
        let file_data = String::from_utf8_lossy(bytes);

        let mut keymap: HashMap<char, HashSet<char>> = HashMap::new();

        for line in file_data.lines() {
            let chars = line.chars().collect::<Vec<char>>();
            keymap.entry(chars[0]).or_insert(HashSet::new());

            for i in 1..chars.len() {
                let c = chars[i];
                let prev_c = chars[i - 1];

                keymap.entry(c)
                    .and_modify(|prevs| { prevs.insert(prev_c); })
                    .or_insert(HashSet::from([prev_c]));
            }
        }

        let mut passcode = String::from("");
        let mut processed_keys = 0;
        let keys_to_process = keymap.len();

        // ALGO:
        // find the first character with no predecessors
        // add it to the passcode
        // remove it from the mappings
        // remove it from all other chars prevs
        while processed_keys < keys_to_process {
            match find_key_without_prevs(&keymap) {
                Some(c) => {
                    passcode.push(c);

                    let references = get_all_references_to_key(&keymap, c);

                    for ch in references {
                        keymap.entry(ch).and_modify(|prevs| { prevs.remove(&c); });
                    }

                    keymap.remove(&c);
                },
                None => {}
            }

            processed_keys += 1;
        }

        format!("{}", passcode)
    }
}
