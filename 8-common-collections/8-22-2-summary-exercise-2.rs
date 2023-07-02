fn is_vowel(letter: &char) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let mut letter_is_vowel = false;
    for vowel in &vowels {
        if letter == vowel {
            letter_is_vowel = true;
            break;
        }
    }
    return letter_is_vowel;
}

fn main() {
    let paragraph = String::from("Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!");
    let mut pig_latin_words: Vec<String> = vec![];
    for word in paragraph.split_whitespace() {
        let mut first_letter_at = 0;
        let mut first_letter = word.chars().nth(first_letter_at).unwrap();
        let mut first_letter_is_uppercase = first_letter.is_uppercase();
        let mut first_letter_is_vowel = is_vowel(&first_letter);
        if !first_letter.is_alphabetic() {
            for index in 1..word.len() {
                let new_letter = word.chars().nth(index).unwrap();
                if !new_letter.is_alphabetic() { continue };
                first_letter = new_letter;
                first_letter_at = index;
                first_letter_is_uppercase = new_letter.is_uppercase();
                first_letter_is_vowel = is_vowel(&new_letter);
                break;
            }
        }

        let mut last_letter_at = word.len() - 1;
        let last_letter = word.chars().nth(last_letter_at).unwrap();
        if !last_letter.is_alphabetic() {
            for index in (0..word.len()).rev() {
                let new_letter = word.chars().nth(index).unwrap();
                if !new_letter.is_alphabetic() { continue };
                last_letter_at = index;
                break;
            }
        }

        let mut new_word = String::from(word);
        if first_letter_is_vowel {
            let suffix_string = "-hay";
            if last_letter_at == new_word.len() - 1 {
                new_word.push_str(&suffix_string);
            } else {
                let (first_part, last_part) = new_word.split_at(last_letter_at + 1);
                new_word = format!("{first_part}{suffix_string}{last_part}");
            }
        } else {
            let suffix_string = format!("-{0}ay", first_letter.to_lowercase());
            if last_letter_at == new_word.len() - 1 {
                new_word.push_str(&suffix_string);
            } else {
                let (first_part, last_part) = new_word.split_at(last_letter_at + 1);
                new_word = format!("{first_part}{suffix_string}{last_part}");
            }
            new_word.remove(first_letter_at);
            if first_letter_is_uppercase {
                let new_first_letter = new_word.remove(0).to_uppercase().collect::<Vec<_>>()[0];
                new_word.insert(0, new_first_letter);
            }
        }

        pig_latin_words.push(new_word);
    }
    let new_paragraph = pig_latin_words.join(" ");
    println!("\n{paragraph}");
    println!("\n--->\n");
    println!("{new_paragraph}\n");
}

