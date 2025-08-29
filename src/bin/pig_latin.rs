fn parse_word(mut word: String) -> Result<String, String> {
    match word.chars().next().unwrap_or('☒') {
        '☒' => return Err(String::from("Something gone wrong")),
        'a' => {
            word.push_str("-hay");
            return Ok(word);
        }
        'e' => {
            word.push_str("-hay");
            return Ok(word);
        }
        'i' => {
            word.push_str("-hay");
            return Ok(word);
        }
        'o' => {
            word.push_str("-hay");
            return Ok(word);
        }
        'u' => {
            word.push_str("-hay");
            return Ok(word);
        }
        _ => (),
    }

    let first_char: char = word[..1].parse().unwrap();
    word = String::from(&word[1..]);
    word.push('-');
    word.push(first_char);
    word.push_str("ay");

    return Ok(word);
}

fn parse_sentence(sentence: String) -> String {
    let mut words: Vec<String> = Vec::new();
    for word in sentence.split_whitespace() {
        words.push(String::from(word));
    }

    for word in &mut words {
        *word = parse_word(word.clone()).unwrap_or(String::from("[¯\\_(ツ)_/¯]"));
    }

    let mut parsed_sentence = String::new();
    for word in &words {
        parsed_sentence.push_str(&word);
        parsed_sentence.push(' ');
    }

    return String::from(parsed_sentence.trim());
}

fn main() -> Result<(), String> {
    let txt = String::from("This is apple");
    dbg!(&txt);
    dbg!(parse_sentence(String::from(&txt)));

    return Ok(());
}
