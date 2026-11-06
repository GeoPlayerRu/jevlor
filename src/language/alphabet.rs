/// Structure that contains information about letters in language.
pub struct Alphabet {
    /// All leters that alphabet can provide.
    pub letters: Vec<char>,
    vowels: Vec<char>,
    consonants: Vec<char>,
}

impl Alphabet {
    pub fn new(letters: Vec<char>) -> Self {
        Self {
            letters,
            vowels: vec![],
            consonants: vec![],
        }
    }

    pub fn define_vowels(&mut self, vowels: Vec<char>) {
        self.vowels = vowels;
    }

    pub fn define_consonants(&mut self, consonants: Vec<char>) {
        self.consonants = consonants;
    }

    /// Get vowels as vec of [char].
    pub fn get_vowels(&self) -> Vec<char> {
        self.vowels.iter().copied().collect()
    }

    /// Get consonants as vec of [UlPair].
    pub fn get_consonants(&self) -> Vec<char> {
        self.consonants.iter().copied().collect()
    }
}
