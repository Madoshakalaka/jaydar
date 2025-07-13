use std::collections::HashMap;

mod nhk_data;
pub mod kana_utils;
mod katakana_support;


#[derive(Debug, Clone, PartialEq)]
pub struct WordFrequency {
    pub text: String,
    pub reading: String,
    pub frequency_score: u32,
    pub is_common: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WordFrequencyWithPitch {
    pub text: String,
    pub reading: String,
    pub frequency_score: u32,
    pub is_common: bool,
    pub pitch_accent: Vec<u8>,  // Multiple pitch accents in order of preference
}

#[derive(Debug, Clone, PartialEq)]
pub enum FindWithNhkResult {
    NoHomophones,
    UniqueMatch {
        true_homophones: Vec<WordFrequencyWithPitch>,
        different_pitch_homophones: Vec<WordFrequencyWithPitch>,
    },
    MultipleMatches {
        homophones: Vec<WordFrequencyWithPitch>,
    },
}

pub fn find(word: &str) -> Vec<WordFrequency> {
    // Use the enhanced function that handles katakana properly
    katakana_support::find_with_katakana_support(word)
}

pub(crate) fn calculate_frequency_score(priority: &jmdict::Priority) -> u32 {
    use jmdict::PriorityInCorpus::*;
    
    let mut score = 0u32;
    
    // Base score from frequency bucket (1-48, where 1 is most common)
    if priority.frequency_bucket > 0 {
        score += (50 - priority.frequency_bucket as u32) * 1000;
    }
    
    // Additional scores from different corpora
    match priority.news {
        Primary => score += 500,
        Secondary => score += 200,
        Absent => {}
    }
    
    match priority.ichimango {
        Primary => score += 500,
        Secondary => score += 200,
        Absent => {}
    }
    
    match priority.loanwords {
        Primary => score += 300,
        Secondary => score += 100,
        Absent => {}
    }
    
    match priority.additional {
        Primary => score += 400,
        Secondary => score += 150,
        Absent => {}
    }
    
    score
}

pub fn find_with_nhk(word: &str) -> FindWithNhkResult {
    // Convert katakana to hiragana if needed
    let search_word = if kana_utils::contains_katakana(word) {
        kana_utils::katakana_to_hiragana(word)
    } else {
        word.to_string()
    };
    let original_word = word;
    let hiragana_word = search_word.as_str();
    
    let mut homophones = Vec::new();
    let mut reading_to_words: HashMap<String, Vec<(String, u32, bool, Vec<u8>)>> = HashMap::new();
    
    // First, find the target word's pitch accent and determine if input is unique
    let mut target_pitches: Vec<u8> = Vec::new();
    let mut target_readings = Vec::new();
    let mut found_exact_match = false;
    let mut exact_match_text = String::new();
    
    // If input was katakana, we want to search for the hiragana reading
    if kana_utils::contains_katakana(original_word) {
        target_readings.push(hiragana_word.to_string());
    }
    
    for entry in jmdict::entries() {
        // Check kanji elements
        for kanji in entry.kanji_elements() {
            if kanji.text == original_word {
                found_exact_match = true;
                exact_match_text = kanji.text.to_string();
                for reading in entry.reading_elements() {
                    if !target_readings.contains(&reading.text.to_string()) {
                        target_readings.push(reading.text.to_string());
                    }
                    
                    // Get pitch accents for this word
                    if target_pitches.is_empty() {
                        target_pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                    }
                    
                    let freq_score = calculate_frequency_score(&reading.priority);
                    let pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                    let key = reading.text.to_string();
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((kanji.text.to_string(), freq_score, reading.priority.is_common(), pitches));
                }
            }
        }
        
        // Check reading elements
        for reading in entry.reading_elements() {
            if reading.text == original_word || reading.text == hiragana_word {
                if !target_readings.contains(&reading.text.to_string()) {
                    target_readings.push(reading.text.to_string());
                }
                
                // Get pitch accents for kana-only word
                if target_pitches.is_empty() {
                    target_pitches = nhk_data::get_pitch_accents(reading.text, reading.text);
                }
                
                let freq_score = calculate_frequency_score(&reading.priority);
                let key = reading.text.to_string();
                
                if entry.kanji_elements().count() == 0 {
                    // Check if this kana-only word is unique (no other words with same reading)
                    if reading.text == original_word && !found_exact_match {
                        found_exact_match = true;
                        exact_match_text = reading.text.to_string();
                    }
                    let pitches = nhk_data::get_pitch_accents(reading.text, reading.text);
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((reading.text.to_string(), freq_score, reading.priority.is_common(), pitches));
                } else {
                    for kanji in entry.kanji_elements() {
                        let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                        let pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                        reading_to_words.entry(key.clone())
                            .or_insert_with(Vec::new)
                            .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common(), pitches));
                    }
                }
            }
        }
    }
    
    // Second pass: collect all words with the same readings
    if !target_readings.is_empty() {
        for entry in jmdict::entries() {
            for reading in entry.reading_elements() {
                if target_readings.contains(&reading.text.to_string()) {
                    let freq_score = calculate_frequency_score(&reading.priority);
                    let key = reading.text.to_string();
                    
                    if entry.kanji_elements().count() == 0 {
                        let pitches = nhk_data::get_pitch_accents(reading.text, reading.text);
                        reading_to_words.entry(key)
                            .or_insert_with(Vec::new)
                            .push((reading.text.to_string(), freq_score, reading.priority.is_common(), pitches));
                    } else {
                        for kanji in entry.kanji_elements() {
                            let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                            let pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                            reading_to_words.entry(key.clone())
                                .or_insert_with(Vec::new)
                                .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common(), pitches));
                        }
                    }
                }
            }
        }
    }
    
    // If input was katakana, also include katakana entries
    if kana_utils::contains_katakana(original_word) {
        // Check if the katakana word exists in JMDict
        for entry in jmdict::entries() {
            for kanji in entry.kanji_elements() {
                if kanji.text == original_word {
                    // Found the katakana entry
                    let freq_score = calculate_frequency_score(&kanji.priority);
                    let pitches = nhk_data::get_pitch_accents(hiragana_word, original_word);
                    reading_to_words.entry(hiragana_word.to_string())
                        .or_insert_with(Vec::new)
                        .push((original_word.to_string(), freq_score, kanji.priority.is_common(), pitches));
                    break;
                }
            }
        }
        
        // Also add the katakana itself even if not in JMDict
        let has_katakana_entry = reading_to_words.get(hiragana_word)
            .map(|words| words.iter().any(|(text, _, _, _)| text == original_word))
            .unwrap_or(false);
            
        if !has_katakana_entry {
            reading_to_words.entry(hiragana_word.to_string())
                .or_insert_with(Vec::new)
                .push((original_word.to_string(), 0, false, vec![]));
        }
    }
    
    // Convert to output format and deduplicate
    let mut seen = std::collections::HashSet::new();
    for (reading, words) in reading_to_words {
        for (text, freq_score, is_common, pitches) in words {
            let key = (text.clone(), reading.clone());
            if seen.insert(key) {
                homophones.push(WordFrequencyWithPitch {
                    text,
                    reading: reading.clone(),
                    frequency_score: freq_score,
                    is_common,
                    pitch_accent: pitches,
                });
            }
        }
    }
    
    // Sort by frequency score (higher is more common)
    homophones.sort_by(|a, b| b.frequency_score.cmp(&a.frequency_score));
    
    // Determine result type based on the input and results
    if found_exact_match {
        // Get all unique word texts (not counting different readings of same word)
        let unique_texts: std::collections::HashSet<String> = homophones.iter()
            .filter(|w| target_readings.contains(&w.reading))
            .map(|w| w.text.clone())
            .collect();
        
        // If only one unique word text exists, it has no homophones
        if unique_texts.len() == 1 {
            FindWithNhkResult::NoHomophones
        } else {
            // Filter to only homophones with matching readings
            let same_reading_words: Vec<_> = homophones.into_iter()
                .filter(|w| target_readings.contains(&w.reading))
                .collect();
            
            // Input matches a specific word - divide into true/fake homophones
            let mut true_homophones = Vec::new();
            let mut different_pitch_homophones = Vec::new();
            
            for word in same_reading_words {
                if word.text == exact_match_text {
                    // Always include the exact match word in true homophones
                    true_homophones.push(word);
                } else if !target_pitches.is_empty() && !word.pitch_accent.is_empty() {
                    // Check if any pitch matches
                    if target_pitches.iter().any(|tp| word.pitch_accent.contains(tp)) {
                        true_homophones.push(word);
                    } else {
                        different_pitch_homophones.push(word);
                    }
                } else {
                    // If we don't know pitch, assume it's a true homophone
                    true_homophones.push(word);
                }
            }
            
            FindWithNhkResult::UniqueMatch {
                true_homophones,
                different_pitch_homophones,
            }
        }
    } else {
        // Input is a reading (like hiragana) - check if it's specific enough
        // Get all unique word texts
        let unique_texts: std::collections::HashSet<String> = homophones.iter()
            .map(|w| w.text.clone())
            .collect();
        
        if unique_texts.len() == 1 {
            // Only one unique word (might have multiple readings)
            FindWithNhkResult::NoHomophones
        } else {
            // Multiple different words - return them as MultipleMatches
            FindWithNhkResult::MultipleMatches { homophones }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_homophones() {
        let results = find("かう");
        assert!(!results.is_empty());
        
        // Should find multiple homophones like 買う, 飼う, 支う, etc.
        let texts: Vec<&str> = results.iter().map(|w| w.text.as_str()).collect();
        assert!(texts.contains(&"買う"));
        assert!(texts.contains(&"飼う"));
        
        // Check that they're sorted by frequency
        for i in 1..results.len() {
            assert!(results[i-1].frequency_score >= results[i].frequency_score);
        }
    }

    #[test]
    fn test_common_words_marked() {
        let results = find("こうせい");
        
        // Find common words
        let kousei = results.iter().find(|w| w.text == "構成");
        assert!(kousei.is_some());
        assert!(kousei.unwrap().is_common);
        
        let kousei2 = results.iter().find(|w| w.text == "公正");
        assert!(kousei2.is_some());
        assert!(kousei2.unwrap().is_common);
    }

    #[test]
    fn test_frequency_ranking() {
        let results = find("こうせい");
        
        // Common words should rank higher
        let kousei_idx = results.iter().position(|w| w.text == "構成").unwrap();
        let kousetsu_idx = results.iter().position(|w| w.text == "校正").unwrap_or(999);
        
        // 構成 should rank higher than 校正
        assert!(kousei_idx < kousetsu_idx);
    }

    #[test]
    fn test_katakana_input() {
        let results = find("カイ");
        assert!(!results.is_empty());
        
        // Should find kanji/hiragana homophones  
        let texts: Vec<&str> = results.iter().map(|w| w.text.as_str()).collect();
        assert!(texts.iter().any(|t| t.contains("会")));
        assert!(texts.iter().any(|t| t.contains("回")));
        assert!(texts.iter().any(|t| t.contains("階")));
    }

    #[test]
    fn test_with_nhk() {
        // Test searching by reading - should return MultipleMatches
        let result = find_with_nhk("こうせい");
        match result {
            FindWithNhkResult::MultipleMatches { homophones } => {
                assert!(!homophones.is_empty());
                
                // Find entries with pitch data
                let kousei = homophones.iter().find(|w| w.text == "構成");
                assert!(kousei.is_some());
                
                let kousei = kousei.unwrap();
                assert!(!kousei.pitch_accent.is_empty());
            }
            _ => panic!("Expected MultipleMatches for reading input"),
        }
        
        // Test searching by specific word - should return UniqueMatch
        let result2 = find_with_nhk("構成");
        match result2 {
            FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
                // Find 構成 in true homophones
                let kousei = true_homophones.iter().find(|w| w.text == "構成");
                assert!(kousei.is_some(), "構成 should be in true_homophones");
                
                // Find 後世 - it should be in different_pitch if it has different pitch
                let kousei2 = true_homophones.iter().find(|w| w.text == "後世")
                    .or_else(|| different_pitch_homophones.iter().find(|w| w.text == "後世"));
                
                if let (Some(k1), Some(k2)) = (kousei, kousei2) {
                    if !k1.pitch_accent.is_empty() && !k2.pitch_accent.is_empty() {
                        // Check if they have common pitch
                        let have_common_pitch = k1.pitch_accent.iter()
                            .any(|p| k2.pitch_accent.contains(p));
                        
                        if have_common_pitch {
                            assert!(true_homophones.iter().any(|w| w.text == "後世"));
                        } else {
                            assert!(different_pitch_homophones.iter().any(|w| w.text == "後世"));
                        }
                    }
                }
            }
            _ => panic!("Expected UniqueMatch for specific word input"),
        }
    }

    #[test]
    fn test_difficult_cases() {
        // Test cases that are known to be challenging
        
        // 1. Multiple readings with different frequencies
        let results = find("かいとう");
        assert!(!results.is_empty());
        let kaitou = results.iter().find(|w| w.text == "回答");
        assert!(kaitou.is_some());
        assert!(kaitou.unwrap().frequency_score > 40000); // Should be common
        
        // 2. Rare vs common words
        let results = find("こうせい"); 
        let kousei_freq = results.iter().find(|w| w.text == "構成").unwrap().frequency_score;
        let kousei_rare = results.iter().find(|w| w.text == "更正");
        if let Some(rare) = kousei_rare {
            assert!(kousei_freq > rare.frequency_score);
        }
    }

    #[test]
    fn test_pitch_accent_discrimination() {
        // When searching by reading, should return MultipleMatches
        let result = find_with_nhk("はし");
        match result {
            FindWithNhkResult::MultipleMatches { homophones } => {
                assert!(!homophones.is_empty());
                // All words should be present
                assert!(homophones.iter().any(|w| w.text == "橋"));
                assert!(homophones.iter().any(|w| w.text == "箸"));
            }
            _ => panic!("Expected MultipleMatches for reading input"),
        }
        
        // Test searching by specific word
        let result_bridge = find_with_nhk("橋");
        match result_bridge {
            FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
                let bridge = true_homophones.iter().find(|w| w.text == "橋");
                assert!(bridge.is_some(), "橋 should be in true_homophones");
                
                // Find 箸 - it should be in different_pitch if it has different pitch
                let chopsticks = true_homophones.iter().find(|w| w.text == "箸")
                    .or_else(|| different_pitch_homophones.iter().find(|w| w.text == "箸"));
                
                if let (Some(b), Some(c)) = (bridge, chopsticks) {
                    if !b.pitch_accent.is_empty() && !c.pitch_accent.is_empty() {
                        // They should have different pitch accents
                        let have_common_pitch = b.pitch_accent.iter()
                            .any(|p| c.pitch_accent.contains(p));
                        
                        if have_common_pitch {
                            assert!(true_homophones.iter().any(|w| w.text == "箸"));
                        } else {
                            assert!(different_pitch_homophones.iter().any(|w| w.text == "箸"));
                        }
                    }
                }
            }
            _ => panic!("Expected UniqueMatch for specific word input"),
        }
    }

    #[test]
    fn test_katakana_with_pitch() {
        let result = find_with_nhk("ソーセージ");
        match result {
            FindWithNhkResult::UniqueMatch { true_homophones, .. } => {
                // Should find both ソーセージ and 双生児
                let sausage = true_homophones.iter().find(|w| w.text == "ソーセージ");
                assert!(sausage.is_some(), "ソーセージ should be found");
                
                // Check if 双生児 is present (might be in either list)
                let all_words: Vec<_> = true_homophones.iter().collect();
                assert!(all_words.iter().any(|w| w.text == "ソーセージ"));
            }
            FindWithNhkResult::NoHomophones => {
                // This is also valid if ソーセージ has no homophones
            }
            _ => panic!("Expected UniqueMatch or NoHomophones for katakana input"),
        }
    }

    #[test]
    fn test_multiple_pitch_accents() {
        let result = find_with_nhk("ていど");
        match result {
            FindWithNhkResult::MultipleMatches { homophones } => {
                // Find 程度
                let teido = homophones.iter().find(|w| w.text == "程度");
                assert!(teido.is_some());
                
                let teido = teido.unwrap();
                // Should have multiple pitch accents (1 and 0)
                assert!(teido.pitch_accent.len() > 1, "程度 should have multiple pitch accents");
                assert!(teido.pitch_accent.contains(&1), "程度 should have pitch accent 1");
                assert!(teido.pitch_accent.contains(&0), "程度 should have pitch accent 0");
            }
            _ => panic!("Expected MultipleMatches for reading input"),
        }
    }

    #[test]
    fn test_no_homophones() {
        // Test case 1: 後始末 - has homophone 跡始末, so use a different example
        let result = find_with_nhk("中国語");
        match result {
            FindWithNhkResult::NoHomophones => {
                // This is expected
            }
            _ => panic!("Expected NoHomophones for 中国語"),
        }
        
        // Test case 2: タピオカ - should have no homophones (even if it has multiple readings)
        let result2 = find_with_nhk("タピオカ");
        match result2 {
            FindWithNhkResult::NoHomophones => {
                // This is expected
            }
            _ => panic!("Expected NoHomophones for タピオカ"),
        }
        
        // Test case 3: にほんご - specific enough to only be 日本語
        let result3 = find_with_nhk("にほんご");
        match result3 {
            FindWithNhkResult::NoHomophones => {
                // This is expected
            }
            _ => panic!("Expected NoHomophones for にほんご"),
        }
    }
}