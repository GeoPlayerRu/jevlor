# Todo list of something to do

## Morphems
- [x] Generate differently based on morphem type
- [ ] Figure out prefix generation
- [ ] Figure out suffix generation
- [ ] Figure out ending generation
## Syllables
- [x] Make roots generate from syllables
## Words
- [ ] Add word types with different morphems
- [ ] Make generators that make words using pre-existing morphems
- [~] Make generators that combine statistical and pre-existing approach
    - [x] Make wordgen that can use different morphemgens
    - [ ] Make wordgen with easily replacable morphemgens
## Generation Misc.
- [ ] (possibly) Create grammatical rules to generate sentences.
## Command-line interface
- [x] Split into different crate main.rs
- [x] Utilize clap to make easy to use interface
- [x] Lookup and default lang
- [ ] Advanced word creation
- [ ] Passing stdout to stdin
- [ ] Morphem generation
- [ ] Advanced morphem generation
## Tools
- [ ] Remaking existing words by taking syllables at the beginning and the end.
- [ ] Chaining commands (e.g "jelvor-cli en morphems | jelvor-cli en load-morphems - words" should generate morphems and use them in pre-determent word generation)
## Abstractions
- [ ] Ultimately this must generate words in any language with words
- [ ] Generate sounds instead of words
- [x] Shuffle structs a bit to make things easier and evade using Option
