use crate::kana_utils::{katakana_to_hiragana, contains_katakana};
use crate::{WordFrequency, calculate_frequency_score};
use std::collections::HashMap;

/// Enhanced find function that properly handles katakana input
pub fn find_with_katakana_support(word: &str) -> Vec<WordFrequency> {
    let mut homophones = Vec::new();
    let mut reading_to_words: HashMap<String, Vec<(String, u32, bool)>> = HashMap::new();
    
    // Convert katakana to hiragana for searching, since JMDict stores readings in hiragana
    let search_word = if contains_katakana(word) {
        katakana_to_hiragana(word)
    } else {
        word.to_string()
    };
    
    // Also keep the original word for matching text fields
    let original_word = word;
    let hiragana_word = search_word.as_str();
    
    // First pass: find all entries matching the input word and collect their readings
    let mut target_readings = Vec::new();
    
    // If input was katakana, we want to search for the hiragana reading
    if contains_katakana(original_word) {
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
                    
                    let freq_score = calculate_frequency_score(&reading.priority);
                    let key = reading.text.to_string();
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((kanji.text.to_string(), freq_score, reading.priority.is_common()));
                }
            }
        }
        
        // Check reading elements
        for reading in entry.reading_elements() {
            if reading.text == original_word || reading.text == hiragana_word {
                if !target_readings.contains(&reading.text.to_string()) {
                    target_readings.push(reading.text.to_string());
                }
                
                let freq_score = calculate_frequency_score(&reading.priority);
                let key = reading.text.to_string();
                
                // For kana-only entries
                if entry.kanji_elements().count() == 0 {
                    reading_to_words.entry(key)
                        .or_insert_with(Vec::new)
                        .push((reading.text.to_string(), freq_score, reading.priority.is_common()));
                } else {
                    // Add all kanji forms with this reading
                    for kanji in entry.kanji_elements() {
                        let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                        reading_to_words.entry(key.clone())
                            .or_insert_with(Vec::new)
                            .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common()));
                    }
                }
            }
        }
    }
    
    // Second pass: collect all words with the same readings as our target word
    // Now we need to normalize readings to handle long vowels properly
    if !target_readings.is_empty() {
        // Normalize target readings for comparison
        let normalized_targets: Vec<String> = target_readings.iter()
            .map(|r| katakana_to_hiragana(r))
            .collect();
        
        for entry in jmdict::entries() {
            for reading in entry.reading_elements() {
                // Normalize the reading for comparison
                let normalized_reading = katakana_to_hiragana(reading.text);
                
                // Check if this normalized reading matches any of our targets
                if normalized_targets.contains(&normalized_reading) {
                    let freq_score = calculate_frequency_score(&reading.priority);
                    // Use the original reading for storage
                    let key = reading.text.to_string();
                    
                    if entry.kanji_elements().count() == 0 {
                        // Kana-only entry
                        reading_to_words.entry(key)
                            .or_insert_with(Vec::new)
                            .push((reading.text.to_string(), freq_score, reading.priority.is_common()));
                    } else {
                        // Add all kanji forms
                        for kanji in entry.kanji_elements() {
                            let kanji_freq_score = calculate_frequency_score(&kanji.priority);
                            reading_to_words.entry(key.clone())
                                .or_insert_with(Vec::new)
                                .push((kanji.text.to_string(), kanji_freq_score, kanji.priority.is_common()));
                        }
                    }
                }
            }
        }
    }
    
    // If input was katakana, also include the katakana word itself
    if contains_katakana(original_word) {
        // Check if the katakana word exists in JMDict (like Χ for Chi)
        for entry in jmdict::entries() {
            for kanji in entry.kanji_elements() {
                if kanji.text == original_word {
                    // Found the katakana entry
                    let freq_score = calculate_frequency_score(&kanji.priority);
                    reading_to_words.entry(hiragana_word.to_string())
                        .or_insert_with(Vec::new)
                        .push((original_word.to_string(), freq_score, kanji.priority.is_common()));
                    break;
                }
            }
        }
        
        // Also add the katakana itself even if not in JMDict
        let has_katakana_entry = reading_to_words.get(hiragana_word)
            .map(|words| words.iter().any(|(text, _, _)| text == original_word))
            .unwrap_or(false);
            
        if !has_katakana_entry {
            reading_to_words.entry(hiragana_word.to_string())
                .or_insert_with(Vec::new)
                .push((original_word.to_string(), 0, false));
        }
    }
    
    // Convert to output format and deduplicate
    let mut seen = std::collections::HashSet::new();
    for (reading, words) in reading_to_words {
        for (text, freq_score, is_common) in words {
            let key = (text.clone(), reading.clone());
            if seen.insert(key) {
                homophones.push(WordFrequency {
                    text,
                    reading: reading.clone(),
                    frequency_score: freq_score,
                    is_common,
                });
            }
        }
    }
    
    // Sort by frequency score (higher is more common)
    homophones.sort_by(|a, b| b.frequency_score.cmp(&a.frequency_score));
    
    homophones
}