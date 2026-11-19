use jevlor::morphem::MorphemType;
use jevlor::word::Word;

/// Splits word into morphems in kinda readable format
pub fn format_word_morphems(word: &Word) -> String {
    let word_morphems = word.into_morphems_hashmap();
    let mut result: Vec<String> = Vec::new();
    if let Some(prefixes) = word_morphems.get(&MorphemType::Prefix)
        && !prefixes.is_empty()
    {
        result.push(format!("Prefixes:{:?}", prefixes));
    }
    if let Some(roots) = word_morphems.get(&MorphemType::Root)
        && !roots.is_empty()
    {
        result.push(format!("Roots:{:?}", roots));
    }
    if let Some(suffixes) = word_morphems.get(&MorphemType::Suffix)
        && !suffixes.is_empty()
    {
        result.push(format!("Suffixes:{:?}", suffixes));
    }
    if let Some(ending) = word_morphems.get(&MorphemType::Ending)
        && !ending.is_empty()
    {
        result.push(format!("Ending:\"{}\"", ending[0]));
    }
    if let Some(postfix) = word_morphems.get(&MorphemType::Postfix)
        && !postfix.is_empty()
    {
        result.push(format!("Postfix:\"{}\"", postfix[0]));
    }

    result.join(";")
}
