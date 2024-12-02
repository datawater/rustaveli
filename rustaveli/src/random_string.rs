use rand::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RandomString {
    pub value: String,
    amount_of_words: u8,
    amount_of_random_letters_min: u8,
}

impl RandomString {
    #[inline]
    fn words() -> [String; 17] {
        [
            "aes".into(),
            "decrypt".into(),
            "encode".into(),
            "windows".into(),
            "startup".into(),
            "shell".into(),
            "network".into(),
            "get".into(),
            "curl".into(),
            "decode".into(),
            "registry".into(),
            "update".into(),
            "send".into(),
            "request".into(),
            "process".into(),
            "spawn".into(),
            "task".into(),
        ]
    }

    pub fn new(amount_of_words: Option<u8>, amount_of_random_letters_min: Option<u8>) -> Self {
        let mut r = Self::default();
        let mut rng = thread_rng();
        r.amount_of_random_letters_min = amount_of_random_letters_min.unwrap_or(3);
        r.amount_of_words = amount_of_words.unwrap_or(2);

        let words_list = Self::words();
        let words = (0..r.amount_of_words)
            .map(|_| words_list[rng.gen_range(0..words_list.len()) as usize].clone())
            .collect::<Vec<_>>();

        let mut jw = words.join("_");

        if jw.len() == 1 {
            jw.pop();
        }

        let amount_of_letters = (jw.len() as f32 * 0.3) as i32
            + r.amount_of_random_letters_min as i32
            + rng.gen_range(0..r.amount_of_random_letters_min as i32 / 2);

        jw += &(0..amount_of_letters)
            .map(|_| String::from(rng.gen_range(b'a'..=b'z') as char))
            .collect::<Vec<_>>()
            .join("");

        r.value = jw;

        return r;
    }
}
