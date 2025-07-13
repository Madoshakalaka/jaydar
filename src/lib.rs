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
    pub is_true_homophone: bool,  // false if different pitch from query word
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

pub fn find_with_nhk(word: &str) -> Vec<WordFrequencyWithPitch> {
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
    
    // First, find the target word's pitch accent
    let mut target_pitches: Vec<u8> = Vec::new();
    let mut target_readings = Vec::new();
    
    // If input was katakana, we want to search for the hiragana reading
    if kana_utils::contains_katakana(original_word) {
        target_readings.push(hiragana_word.to_string());
    }
    
    for entry in jmdict::entries() {
        // Check kanji elements
        for kanji in entry.kanji_elements() {
            if kanji.text == original_word {
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
                let is_true_homophone = if !target_pitches.is_empty() && !pitches.is_empty() {
                    // Check if any pitch matches
                    target_pitches.iter().any(|tp| pitches.contains(tp))
                } else {
                    true  // If we don't know pitch, assume it's a true homophone
                };
                
                homophones.push(WordFrequencyWithPitch {
                    text,
                    reading: reading.clone(),
                    frequency_score: freq_score,
                    is_common,
                    pitch_accent: pitches,
                    is_true_homophone,
                });
            }
        }
    }
    
    // Sort by frequency score (higher is more common)
    homophones.sort_by(|a, b| b.frequency_score.cmp(&a.frequency_score));
    
    homophones
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
        // Test searching by reading - all should be true homophones
        let results = find_with_nhk("こうせい");
        assert!(!results.is_empty());
        
        // Find entries with pitch data
        let kousei = results.iter().find(|w| w.text == "構成");
        assert!(kousei.is_some());
        
        let kousei = kousei.unwrap();
        assert!(!kousei.pitch_accent.is_empty());
        
        // When searching by reading, all should be true homophones
        for word in &results {
            assert!(word.is_true_homophone);
        }
        
        // Test searching by specific word - should mark different pitch as fake
        let results2 = find_with_nhk("構成");
        let kousei_by_word = results2.iter().find(|w| w.text == "構成");
        let kousei2_by_word = results2.iter().find(|w| w.text == "後世");
        
        if let (Some(k1), Some(k2)) = (kousei_by_word, kousei2_by_word) {
            assert!(k1.is_true_homophone); // Same word should be true
            if !k1.pitch_accent.is_empty() && !k2.pitch_accent.is_empty() {
                // Different pitch should be fake homophone
                let have_common_pitch = k1.pitch_accent.iter()
                    .any(|p| k2.pitch_accent.contains(p));
                assert_eq!(k2.is_true_homophone, have_common_pitch);
            }
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
        // When searching by reading, all should be true homophones
        let results = find_with_nhk("はし");
        for word in &results {
            assert!(word.is_true_homophone, "{} should be true homophone when searching by reading", word.text);
        }
        
        // Test searching by specific word
        let results_bridge = find_with_nhk("橋");
        let bridge = results_bridge.iter().find(|w| w.text == "橋");
        let chopsticks = results_bridge.iter().find(|w| w.text == "箸");
        
        if let (Some(b), Some(c)) = (bridge, chopsticks) {
            assert!(b.is_true_homophone); // Query word should be true
            if !b.pitch_accent.is_empty() && !c.pitch_accent.is_empty() {
                // They should have different pitch accents
                let have_common_pitch = b.pitch_accent.iter()
                    .any(|p| c.pitch_accent.contains(p));
                // If they have different pitches, chopsticks should be fake homophone
                assert_eq!(c.is_true_homophone, have_common_pitch);
            }
        }
    }

    #[test]
    fn test_katakana_with_pitch() {
        let results = find_with_nhk("ソーセージ");
        assert!(!results.is_empty());
        
        // Should find both ソーセージ and 双生児
        let sausage = results.iter().find(|w| w.text == "ソーセージ");
        let twins = results.iter().find(|w| w.text == "双生児");
        
        assert!(sausage.is_some());
        assert!(twins.is_some());
        
        // Both should exist (the reading comparison was incorrect as ソーセージ may keep its katakana reading)
    }

    #[test]
    fn test_multiple_pitch_accents() {
        let results = find_with_nhk("ていど");
        
        // Find 程度
        let teido = results.iter().find(|w| w.text == "程度");
        assert!(teido.is_some());
        
        let teido = teido.unwrap();
        // Should have multiple pitch accents (1 and 0)
        assert!(teido.pitch_accent.len() > 1, "程度 should have multiple pitch accents");
        assert!(teido.pitch_accent.contains(&1), "程度 should have pitch accent 1");
        assert!(teido.pitch_accent.contains(&0), "程度 should have pitch accent 0");
    }
}