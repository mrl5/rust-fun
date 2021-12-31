// https://github.com/exercism/rust/tree/630a56517c9015341cdd96233632eda9ad8f44c4/exercises/concept/magazine-cutout
// https://exercism.org/tracks/rust/exercises/magazine-cutout/solutions/mrl5

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = map_magazine(magazine);
    evaluate_note(note, &mut map)
}

fn map_magazine<'a>(magazine: &'a [&str]) -> HashMap<&'a str, usize> {
    // https://doc.rust-lang.org/reference/lifetime-elision.html
    let mut map: HashMap<&str, usize> = HashMap::new();
    for word in magazine {
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

fn evaluate_note<'a>(note: &'a [&str], map: &mut HashMap<&'a str, usize>) -> bool {
    for word in note {
        let counter = map.entry(word).or_insert(0);
        if counter == &0 {
            return false;
        }
        *counter -= 1;
    }
    return true;
}
