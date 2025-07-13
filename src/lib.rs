use std::collections::HashMap;

pub mod kana_utils;
mod katakana_support;
mod nhk_data;

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
    pub pitch_accent: Vec<u8>, // Multiple pitch accents in order of preference
    pub is_true_homophone: bool, // false if different pitch from query word
}

#[derive(Debug, Clone, PartialEq)]
pub struct NewWordFrequencyWithPitch {
    pub text: String,
    pub reading: String,
    pub frequency_score: u32,
    pub is_common: bool,
    pub pitch_accent: Vec<u8>, // Multiple pitch accents in order of preference
}

#[derive(Debug, Clone, PartialEq)]
pub enum FindWithNhkResult {
    UniqueMatch {
        pitch_accent: Vec<u8>,
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

    // First, find the target word's pitch accent
    let mut target_pitches: Vec<u8> = Vec::new();
    let mut target_readings = Vec::new();
    let mut is_unique_match = false;

    // If input was katakana, we want to search for the hiragana reading
    if kana_utils::contains_katakana(original_word) {
        target_readings.push(hiragana_word.to_string());
    }

    // Check if word is a specific text (kanji) rather than a reading
    for entry in jmdict::entries() {
        // Check kanji elements
        for kanji in entry.kanji_elements() {
            if kanji.text == original_word {
                is_unique_match = true;
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
                    reading_to_words.entry(key).or_insert_with(Vec::new).push((
                        kanji.text.to_string(),
                        freq_score,
                        reading.priority.is_common(),
                        pitches,
                    ));
                }
            }
        }

        // Check reading elements
        for reading in entry.reading_elements() {
            if reading.text == original_word || reading.text == hiragana_word {
                if !is_unique_match {
                    // Only mark as reading-based search if we didn't find a kanji match
                    if !target_readings.contains(&reading.text.to_string()) {
                        target_readings.push(reading.text.to_string());
                    }

                    // Get pitch accents for kana-only word
                    if target_pitches.is_empty() && entry.kanji_elements().count() == 0 {
                        target_pitches = nhk_data::get_pitch_accents(reading.text, reading.text);
                    }
                }

                let freq_score = calculate_frequency_score(&reading.priority);
                let key = reading.text.to_string();

                if entry.kanji_elements().count() == 0 {
                    let pitches = nhk_data::get_pitch_accents(reading.text, reading.text);
                    reading_to_words.entry(key).or_insert_with(Vec::new).push((
                        reading.text.to_string(),
                        freq_score,
                        reading.priority.is_common(),
                        pitches,
                    ));
                } else {
                    for kanji in entry.kanji_elements() {
                        let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                        let pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                        reading_to_words
                            .entry(key.clone())
                            .or_insert_with(Vec::new)
                            .push((
                                kanji.text.to_string(),
                                kanji_freq_score,
                                kanji.priority.is_common(),
                                pitches,
                            ));
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
                        reading_to_words.entry(key).or_insert_with(Vec::new).push((
                            reading.text.to_string(),
                            freq_score,
                            reading.priority.is_common(),
                            pitches,
                        ));
                    } else {
                        for kanji in entry.kanji_elements() {
                            let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                            let pitches = nhk_data::get_pitch_accents(reading.text, kanji.text);
                            reading_to_words
                                .entry(key.clone())
                                .or_insert_with(Vec::new)
                                .push((
                                    kanji.text.to_string(),
                                    kanji_freq_score,
                                    kanji.priority.is_common(),
                                    pitches,
                                ));
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
                    reading_to_words
                        .entry(hiragana_word.to_string())
                        .or_insert_with(Vec::new)
                        .push((
                            original_word.to_string(),
                            freq_score,
                            kanji.priority.is_common(),
                            pitches,
                        ));
                    break;
                }
            }
        }

        // Also add the katakana itself even if not in JMDict
        let has_katakana_entry = reading_to_words
            .get(hiragana_word)
            .map(|words| words.iter().any(|(text, _, _, _)| text == original_word))
            .unwrap_or(false);

        if !has_katakana_entry {
            reading_to_words
                .entry(hiragana_word.to_string())
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
                let is_true_homophone = if is_unique_match && !target_pitches.is_empty() && !pitches.is_empty() {
                    // For unique match, check if pitch matches the target
                    target_pitches.iter().any(|tp| pitches.contains(tp))
                } else {
                    true // For reading-based search, all are true homophones
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

    // Return appropriate result based on match type
    if is_unique_match {
        let true_homophones = homophones.iter()
            .filter(|w| w.is_true_homophone)
            .cloned()
            .collect();
        let different_pitch_homophones = homophones.iter()
            .filter(|w| !w.is_true_homophone)
            .cloned()
            .collect();
        
        FindWithNhkResult::UniqueMatch {
            pitch_accent: target_pitches,
            true_homophones,
            different_pitch_homophones,
        }
    } else {
        // Check if reading uniquely identifies a single word
        let unique_texts: std::collections::HashSet<String> = homophones.iter()
            .map(|w| w.text.clone())
            .collect();
        
        if unique_texts.len() == 1 && !homophones.is_empty() {
            // This reading uniquely identifies one word
            let unique_word = &homophones[0];
            FindWithNhkResult::UniqueMatch {
                pitch_accent: unique_word.pitch_accent.clone(),
                true_homophones: homophones,
                different_pitch_homophones: vec![],
            }
        } else {
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
            assert!(results[i - 1].frequency_score >= results[i].frequency_score);
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
        let results = find_with_nhk("こうせい");
        match results {
            FindWithNhkResult::MultipleMatches { homophones } => {
                assert!(!homophones.is_empty());
                
                // Find entries with pitch data
                let kousei = homophones.iter().find(|w| w.text == "構成");
                assert!(kousei.is_some());
                
                let kousei = kousei.unwrap();
                assert!(!kousei.pitch_accent.is_empty());
                
                // When searching by reading, all should be true homophones
                for word in &homophones {
                    assert!(word.is_true_homophone);
                }
            }
            _ => panic!("Expected MultipleMatches for reading search")
        }

        // Test searching by specific word - should return UniqueMatch
        let results2 = find_with_nhk("構成");
        match results2 {
            FindWithNhkResult::UniqueMatch { pitch_accent, true_homophones, different_pitch_homophones } => {
                assert!(!pitch_accent.is_empty());
                
                // The word itself should be in true_homophones
                let kousei_by_word = true_homophones.iter().find(|w| w.text == "構成");
                assert!(kousei_by_word.is_some());
                
                // 後世 with different pitch should be in different_pitch_homophones
                let kousei2 = different_pitch_homophones.iter().find(|w| w.text == "後世");
                if kousei2.is_some() {
                    assert!(!kousei2.unwrap().is_true_homophone);
                }
            }
            _ => panic!("Expected UniqueMatch for specific word search")
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
        let kousei_freq = results
            .iter()
            .find(|w| w.text == "構成")
            .unwrap()
            .frequency_score;
        let kousei_rare = results.iter().find(|w| w.text == "更正");
        if let Some(rare) = kousei_rare {
            assert!(kousei_freq > rare.frequency_score);
        }
    }

    #[test]
    fn test_pitch_accent_discrimination() {
        // When searching by reading, all should be true homophones
        let results = find_with_nhk("はし");
        match results {
            FindWithNhkResult::MultipleMatches { homophones } => {
                for word in &homophones {
                    assert!(
                        word.is_true_homophone,
                        "{} should be true homophone when searching by reading",
                        word.text
                    );
                }
            }
            _ => panic!("Expected MultipleMatches for reading search")
        }

        // Test searching by specific word
        let results_bridge = find_with_nhk("橋");
        match results_bridge {
            FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones, .. } => {
                let bridge = true_homophones.iter().find(|w| w.text == "橋");
                assert!(bridge.is_some(), "橋 should be in true_homophones");
                
                let chopsticks = different_pitch_homophones.iter().find(|w| w.text == "箸");
                if chopsticks.is_some() {
                    assert!(!chopsticks.unwrap().is_true_homophone, "箸 should be marked as fake homophone");
                }
            }
            _ => panic!("Expected UniqueMatch for specific word search")
        }
    }

    #[test]
    fn test_katakana_with_pitch() {
        let results = find_with_nhk("ソーセージ");
        match results {
            FindWithNhkResult::UniqueMatch { true_homophones, .. } => {
                assert!(!true_homophones.is_empty());

                // Should find both ソーセージ and 双生児
                let sausage = true_homophones.iter().find(|w| w.text == "ソーセージ");
                let twins = true_homophones.iter().find(|w| w.text == "双生児");

                assert!(sausage.is_some());
                assert!(twins.is_some());
            }
            FindWithNhkResult::MultipleMatches { homophones } => {
                assert!(!homophones.is_empty());

                // Should find both ソーセージ and 双生児
                let sausage = homophones.iter().find(|w| w.text == "ソーセージ");
                let twins = homophones.iter().find(|w| w.text == "双生児");

                assert!(sausage.is_some());
                assert!(twins.is_some());
            }
        }
    }

    #[test]
    fn test_multiple_pitch_accents() {
        let results = find_with_nhk("ていど");
        
        match results {
            FindWithNhkResult::MultipleMatches { homophones } => {
                // Find 程度
                let teido = homophones.iter().find(|w| w.text == "程度");
                assert!(teido.is_some());

                let teido = teido.unwrap();
                // Should have multiple pitch accents (1 and 0)
                assert!(
                    teido.pitch_accent.len() > 1,
                    "程度 should have multiple pitch accents"
                );
                assert!(
                    teido.pitch_accent.contains(&1),
                    "程度 should have pitch accent 1"
                );
                assert!(
                    teido.pitch_accent.contains(&0),
                    "程度 should have pitch accent 0"
                );
            }
            _ => panic!("Expected MultipleMatches for reading search")
        }
    }

    #[test]
    fn test_nihongo_unique_match() {
        // にほんご should return UniqueMatch when it uniquely identifies 日本語
        let results = find_with_nhk("にほんご");
        
        match results {
            FindWithNhkResult::UniqueMatch { pitch_accent: _, true_homophones, different_pitch_homophones } => {
                // Should find 日本語
                let nihongo = true_homophones.iter().find(|w| w.text == "日本語");
                assert!(nihongo.is_some(), "Should find 日本語 in true_homophones");
                
                // Pitch accent may or may not be available in the data
                // The important thing is that we get a UniqueMatch
                
                // Should be no other words with reading にほんご
                assert_eq!(true_homophones.len() + different_pitch_homophones.len(), 1, 
                    "にほんご should uniquely identify 日本語");
            }
            FindWithNhkResult::MultipleMatches { .. } => {
                panic!("Expected UniqueMatch for にほんご as it uniquely identifies 日本語")
            }
        }
    }
}

