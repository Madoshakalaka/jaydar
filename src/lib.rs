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
    pub pitch_accent: Option<u8>,
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
    let mut reading_to_words: HashMap<String, Vec<(String, u32, bool, Option<u8>)>> = HashMap::new();
    
    // First, find the target word's pitch accent
    let mut target_pitch: Option<u8> = None;
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
                    
                    // Get pitch accent for this word
                    if target_pitch.is_none() {
                        target_pitch = nhk_data::get_pitch_accent(reading.text, kanji.text);
                    }
                    
                    let freq_score = calculate_frequency_score(&reading.priority);
                    let pitch = nhk_data::get_pitch_accent(reading.text, kanji.text);
                    let key = reading.text.to_string();
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((kanji.text.to_string(), freq_score, reading.priority.is_common(), pitch));
                }
            }
        }
        
        // Check reading elements
        for reading in entry.reading_elements() {
            if reading.text == original_word || reading.text == hiragana_word {
                if !target_readings.contains(&reading.text.to_string()) {
                    target_readings.push(reading.text.to_string());
                }
                
                // Get pitch accent for kana-only word
                if target_pitch.is_none() {
                    target_pitch = nhk_data::get_pitch_accent(reading.text, reading.text);
                }
                
                let freq_score = calculate_frequency_score(&reading.priority);
                let key = reading.text.to_string();
                
                if entry.kanji_elements().count() == 0 {
                    let pitch = nhk_data::get_pitch_accent(reading.text, reading.text);
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((reading.text.to_string(), freq_score, reading.priority.is_common(), pitch));
                } else {
                    for kanji in entry.kanji_elements() {
                        let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                        let pitch = nhk_data::get_pitch_accent(reading.text, kanji.text);
                        reading_to_words.entry(key.clone())
                            .or_insert_with(Vec::new)
                            .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common(), pitch));
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
                        let pitch = nhk_data::get_pitch_accent(reading.text, reading.text);
                        reading_to_words.entry(key)
                            .or_insert_with(Vec::new)
                            .push((reading.text.to_string(), freq_score, reading.priority.is_common(), pitch));
                    } else {
                        for kanji in entry.kanji_elements() {
                            let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                            let pitch = nhk_data::get_pitch_accent(reading.text, kanji.text);
                            reading_to_words.entry(key.clone())
                                .or_insert_with(Vec::new)
                                .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common(), pitch));
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
                    let pitch = nhk_data::get_pitch_accent(hiragana_word, original_word);
                    reading_to_words.entry(hiragana_word.to_string())
                        .or_insert_with(Vec::new)
                        .push((original_word.to_string(), freq_score, kanji.priority.is_common(), pitch));
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
                .push((original_word.to_string(), 0, false, None));
        }
    }
    
    // Convert to output format and deduplicate
    let mut seen = std::collections::HashSet::new();
    for (reading, words) in reading_to_words {
        for (text, freq_score, is_common, pitch) in words {
            let key = (text.clone(), reading.clone());
            if seen.insert(key) {
                let is_true_homophone = match (target_pitch, pitch) {
                    (Some(tp), Some(p)) => tp == p,
                    _ => true,  // If we don't know pitch, assume it's a true homophone
                };
                
                homophones.push(WordFrequencyWithPitch {
                    text,
                    reading: reading.clone(),
                    frequency_score: freq_score,
                    is_common,
                    pitch_accent: pitch,
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
    fn test_kanji_input() {
        let results = find("買う");
        assert!(!results.is_empty());
        
        // Should find the word itself
        assert!(results.iter().any(|w| w.text == "買う"));
        
        // And its homophones
        assert!(results.iter().any(|w| w.text == "飼う"));
    }
    
    #[test]
    fn test_common_word() {
        let results = find("きく");
        assert!(!results.is_empty());
        
        // 聞く should be marked as common
        let kiku = results.iter().find(|w| w.text == "聞く");
        assert!(kiku.is_some());
        assert!(kiku.unwrap().is_common);
    }
    
    #[test]
    fn test_kousei_homophones() {
        let results = find("こうせい");
        assert!(!results.is_empty());
        
        // Should find multiple homophones
        let texts: Vec<&str> = results.iter().map(|w| w.text.as_str()).collect();
        assert!(texts.contains(&"構成"));
        assert!(texts.contains(&"攻勢"));
        assert!(texts.contains(&"後世"));
        assert!(texts.contains(&"公正"));
        
        // Print for debugging
        println!("Found {} homophones for こうせい:", results.len());
        for (i, word) in results.iter().enumerate().take(10) {
            println!("{}: {} - score: {}", i+1, word.text, word.frequency_score);
        }
    }
    
    #[test]
    fn test_find_with_nhk() {
        let results = find_with_nhk("こうせい");
        assert!(!results.is_empty());
        
        // Find specific words and check their pitch
        let kousei_0 = results.iter().find(|w| w.text == "構成");
        let kousei_1 = results.iter().find(|w| w.text == "後世");
        
        assert!(kousei_0.is_some());
        assert!(kousei_1.is_some());
        
        let kousei_0 = kousei_0.unwrap();
        let kousei_1 = kousei_1.unwrap();
        
        // Check pitch accents
        assert_eq!(kousei_0.pitch_accent, Some(0));
        assert_eq!(kousei_1.pitch_accent, Some(1));
        
        // If we search for 構成, words with pitch 0 should be true homophones
        let results_kousei = find_with_nhk("構成");
        for word in &results_kousei {
            if let Some(pitch) = word.pitch_accent {
                assert_eq!(word.is_true_homophone, pitch == 0);
            }
        }
        
        println!("\nこうせい homophones with pitch:");
        for word in results.iter().take(10) {
            println!("  {} - pitch: {:?}, true homophone: {}", 
                word.text, word.pitch_accent, word.is_true_homophone);
        }
    }
    
    #[test]
    fn test_fake_homophones() {
        // Test with 後世 (pitch 1) - should mark pitch 0 words as fake homophones
        let results = find_with_nhk("後世");
        
        let kousei_struct = results.iter().find(|w| w.text == "構成");
        let kousei_attack = results.iter().find(|w| w.text == "攻勢");
        let kousei_after = results.iter().find(|w| w.text == "後世");
        
        // 構成 and 攻勢 have pitch 0, so they're fake homophones of 後世 (pitch 1)
        if let Some(w) = kousei_struct {
            assert!(!w.is_true_homophone);
        }
        if let Some(w) = kousei_attack {
            assert!(!w.is_true_homophone);
        }
        // 後世 itself should be a true homophone
        if let Some(w) = kousei_after {
            assert!(w.is_true_homophone);
        }
    }
    
    #[test]
    fn test_frequency_ranking_kousei() {
        // Test 1: Ensure 構成 is more frequent than 後世
        let results = find("こうせい");
        
        let kousei_struct = results.iter().find(|w| w.text == "構成").expect("構成 not found");
        let kousei_after = results.iter().find(|w| w.text == "後世").expect("後世 not found");
        
        assert!(kousei_struct.frequency_score > kousei_after.frequency_score,
            "構成 ({}) should be more frequent than 後世 ({})",
            kousei_struct.frequency_score, kousei_after.frequency_score);
    }
    
    #[test]
    fn test_frequency_ranking_katei() {
        // Test 2: Ensure 家庭 is more frequent than 課程
        let results = find("かてい");
        
        let katei_home = results.iter().find(|w| w.text == "家庭").expect("家庭 not found");
        let katei_course = results.iter().find(|w| w.text == "課程").expect("課程 not found");
        
        assert!(katei_home.frequency_score > katei_course.frequency_score,
            "家庭 ({}) should be more frequent than 課程 ({})",
            katei_home.frequency_score, katei_course.frequency_score);
    }
    
    #[test]
    fn test_hashi_fake_homophones() {
        // Test 3: 橋 and 箸 are fake homophones (different pitch accent)
        let results = find_with_nhk("はし");
        
        let hashi_bridge = results.iter().find(|w| w.text == "橋").expect("橋 not found");
        let hashi_chopsticks = results.iter().find(|w| w.text == "箸").expect("箸 not found");
        
        // Get their pitch accents
        println!("橋 pitch: {:?}", hashi_bridge.pitch_accent);
        println!("箸 pitch: {:?}", hashi_chopsticks.pitch_accent);
        
        // They should have different pitch accents
        if let (Some(bridge_pitch), Some(chopsticks_pitch)) = 
            (hashi_bridge.pitch_accent, hashi_chopsticks.pitch_accent) {
            assert_ne!(bridge_pitch, chopsticks_pitch,
                "橋 and 箸 should have different pitch accents");
        }
        
        // When searching for 橋, 箸 should be marked as fake homophone
        let results_bridge = find_with_nhk("橋");
        let chopsticks_from_bridge = results_bridge.iter()
            .find(|w| w.text == "箸")
            .expect("箸 not found when searching from 橋");
        assert!(!chopsticks_from_bridge.is_true_homophone,
            "箸 should be a fake homophone of 橋");
        
        // When searching for 箸, 橋 should be marked as fake homophone
        let results_chopsticks = find_with_nhk("箸");
        let bridge_from_chopsticks = results_chopsticks.iter()
            .find(|w| w.text == "橋")
            .expect("橋 not found when searching from 箸");
        assert!(!bridge_from_chopsticks.is_true_homophone,
            "橋 should be a fake homophone of 箸");
    }
    
    #[test]
    fn test_katakana_kai() {
        // Test case: カイ (katakana) and its kanji homophones
        let results = find("カイ");
        
        // Should find katakana カイ itself
        let kai_katakana = results.iter().find(|w| w.text == "カイ");
        assert!(kai_katakana.is_some(), "Should find カイ in katakana");
        
        // Should also find kanji homophones with reading かい
        let expected_kanji = vec!["会", "回", "階", "貝", "買い", "下位"];
        let found_kanji: Vec<&str> = results.iter()
            .filter(|w| expected_kanji.contains(&w.text.as_str()))
            .map(|w| w.text.as_str())
            .collect();
        
        println!("Found kanji homophones for カイ: {:?}", found_kanji);
        
        // Check that we found at least some of the expected kanji
        assert!(found_kanji.contains(&"会"), "Should find 会 (meeting)");
        assert!(found_kanji.contains(&"回"), "Should find 回 (times)");
        assert!(found_kanji.contains(&"階"), "Should find 階 (floor)");
        assert!(found_kanji.contains(&"貝"), "Should find 貝 (shell)");
        assert!(found_kanji.contains(&"買い"), "Should find 買い (buy)");
        
        // Print frequency ranking
        println!("\nカイ homophones by frequency:");
        for (i, word) in results.iter().enumerate().take(10) {
            println!("{}: {} ({}) - score: {}", 
                i+1, word.text, word.reading, word.frequency_score);
        }
    }
    
    #[test]
    fn test_katakana_with_pitch() {
        // Test katakana with pitch accent data
        let results = find_with_nhk("カイ");
        
        // Find specific entries
        let kai_katakana = results.iter().find(|w| w.text == "カイ");
        let kai_meeting = results.iter().find(|w| w.text == "会");
        let kai_shell = results.iter().find(|w| w.text == "貝");
        
        if let Some(katakana) = kai_katakana {
            println!("カイ pitch: {:?}", katakana.pitch_accent);
        }
        if let Some(meeting) = kai_meeting {
            println!("会 pitch: {:?}", meeting.pitch_accent);
        }
        if let Some(shell) = kai_shell {
            println!("貝 pitch: {:?}", shell.pitch_accent);
        }
        
        // Check if any have different pitch accents (making them fake homophones)
        let pitch_accents: Vec<Option<u8>> = results.iter()
            .filter_map(|w| {
                if ["カイ", "会", "回", "階", "貝", "買い"].contains(&w.text.as_str()) {
                    Some(w.pitch_accent)
                } else {
                    None
                }
            })
            .collect();
        
        println!("\nPitch accents found: {:?}", pitch_accents);
    }
    
    #[test]
    fn test_difficult_cases() {
        // Test case 1: コック and 刻苦 are homophones (both read こっく)
        let kokku_results = find("コック");
        
        // Should find both コック and 刻苦
        let kokku_kata = kokku_results.iter().find(|w| w.text == "コック");
        let kokku_kanji = kokku_results.iter().find(|w| w.text == "刻苦");
        
        assert!(kokku_kata.is_some(), "Should find コック");
        assert!(kokku_kanji.is_some(), "Should find 刻苦 as homophone of コック");
        
        // Verify they are homophones - コック might keep katakana reading
        if let Some(kata) = kokku_kata {
            // コック may have reading コック or こっく
            assert!(kata.reading == "コック" || kata.reading == "こっく",
                "コック should have reading コック or こっく, found: {}", kata.reading);
        }
        if let Some(kanji) = kokku_kanji {
            assert_eq!(kanji.reading, "こっく");
        }
        
        // Test case 2: ソーセージ and 双生児 ARE homophones (both normalize to そうせいじ)
        let sausage_results = find("ソーセージ");
        let twins_found = sausage_results.iter().find(|w| w.text == "双生児");
        
        // Should find 双生児 when searching for ソーセージ due to long vowel normalization
        assert!(twins_found.is_some(), "双生児 should be a homophone of ソーセージ (both そうせいじ)");
        
        // Verify their readings
        let twins_results = find("双生児");
        if let Some(twins) = twins_results.iter().find(|w| w.text == "双生児") {
            assert_eq!(twins.reading, "そうせいじ", "双生児 should read そうせいじ");
        }
        
        // Print debug info
        println!("\nソーセージ homophones:");
        for word in sausage_results.iter().take(10) {
            println!("  {} ({})", word.text, word.reading);
        }
    }
}