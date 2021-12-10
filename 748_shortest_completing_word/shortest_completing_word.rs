/*
 * @Date: 2021-12-10 09:28:57
 * @Author: Mengsen Wang
 * @LastEditors: Mengsen Wang
 * @LastEditTime: 2021-12-10 09:48:45
 */

pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
    let to_state = |t: &String| {
        t.to_lowercase()
            .bytes()
            .filter(|&c| c.is_ascii_alphabetic())
            .fold(vec![0; 26], |mut s, c| {
                s[(c - b'a') as usize] += 1;
                s
            })
    };
    let lp_state = to_state(&license_plate);
    words.iter().fold(words.join(" "), |mut ans, word| {
        let word_state = to_state(word);
        if lp_state.iter().zip(&word_state).all(|(c1, c2)| c1 <= c2) && word.len() < ans.len() {
            ans = word.clone();
        }
        ans
    })
}

fn main() {
    assert_eq!(
        shortest_completing_word(
            String::from("1s3 PSt"),
            vec!["step", "steps", "stripe", "stepple"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "steps"
    );
    assert_eq!(
        shortest_completing_word(
            String::from("1s3 456"),
            vec!["looks", "pest", "stew", "show"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        "pest"
    );
    assert_eq!(
        shortest_completing_word(
            String::from("Ah71752"),
            vec![
                "suggest",
                "letter",
                "of",
                "husband",
                "easy",
                "education",
                "drug",
                "prevent",
                "writer",
                "old"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        "husband"
    );

    assert_eq!(
        shortest_completing_word(
            String::from("OgEu755"),
            vec![
                "enough", "these", "play", "wide", "wonder", "box", "arrive", "money", "tax",
                "thus"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        "enough"
    );

    assert_eq!(
        shortest_completing_word(
            String::from("iMSlpe4"),
            vec![
                "claim", "consumer", "student", "camera", "public", "never", "wonder", "simple",
                "thought", "use"
            ]
            .iter()
            .map(|s| s.to_string())
            .collect()
        ),
        "simple"
    );
}
