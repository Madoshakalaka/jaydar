use jaydar::{find, find_with_nhk};

fn main() {
    println!("=== Testing Difficult Homophone Cases ===\n");
    
    // Test 1: コック (cook) and 刻苦 (hard work)
    println!("1. Testing コック (cook) and 刻苦 (hard work):");
    
    // First check if コック exists
    let kokku_kata = find("コック");
    println!("   Found {} results for コック:", kokku_kata.len());
    for word in kokku_kata.iter().take(5) {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Check with hiragana
    let kokku_hira = find("こっく");
    println!("\n   Found {} results for こっく:", kokku_hira.len());
    for word in kokku_hira.iter().take(5) {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Check if 刻苦 exists
    let kokku_kanji = find("刻苦");
    println!("\n   Found {} results for 刻苦:", kokku_kanji.len());
    for word in kokku_kanji.iter() {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Test 2: ソーセージ (sausage) and 双生児 (twins)
    println!("\n2. Testing ソーセージ (sausage) and 双生児 (twins):");
    
    // First check if ソーセージ exists
    let sausage_kata = find("ソーセージ");
    println!("   Found {} results for ソーセージ:", sausage_kata.len());
    for word in sausage_kata.iter().take(5) {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Check with hiragana
    let sausage_hira = find("そーせーじ");
    println!("\n   Found {} results for そーせーじ:", sausage_hira.len());
    for word in sausage_hira.iter().take(5) {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Check if 双生児 exists
    let twins_kanji = find("双生児");
    println!("\n   Found {} results for 双生児:", twins_kanji.len());
    for word in twins_kanji.iter() {
        println!("      {} ({}) - score: {}", word.text, word.reading, word.frequency_score);
    }
    
    // Let's also check what reading 双生児 has
    println!("\n3. Checking readings in JMDict:");
    for entry in jmdict::entries() {
        for kanji in entry.kanji_elements() {
            if kanji.text == "双生児" {
                println!("   双生児 has readings:");
                for reading in entry.reading_elements() {
                    println!("      {}", reading.text);
                }
            }
            if kanji.text == "刻苦" {
                println!("   刻苦 has readings:");
                for reading in entry.reading_elements() {
                    println!("      {}", reading.text);
                }
            }
        }
    }
    
    // Check for katakana entries
    for entry in jmdict::entries() {
        for kanji in entry.kanji_elements() {
            if kanji.text == "コック" {
                println!("   Found コック entry with readings:");
                for reading in entry.reading_elements() {
                    println!("      {}", reading.text);
                }
            }
            if kanji.text == "ソーセージ" {
                println!("   Found ソーセージ entry with readings:");
                for reading in entry.reading_elements() {
                    println!("      {}", reading.text);
                }
            }
        }
    }
}