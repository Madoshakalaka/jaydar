use homophone_ranker::{find, find_with_nhk};

fn main() {
    println!("=== Checking ソーセージ readings ===\n");
    
    // Check ソーセージ in JMDict
    for entry in jmdict::entries() {
        for kanji in entry.kanji_elements() {
            if kanji.text == "ソーセージ" {
                println!("Found ソーセージ with readings:");
                for reading in entry.reading_elements() {
                    println!("  {} (raw text)", reading.text);
                }
            }
        }
    }
    
    // Test katakana to hiragana conversion
    // Import from the crate root first
    use homophone_ranker::kana_utils;
    
    println!("\nKatakana to hiragana conversion:");
    println!("  ソーセージ -> {}", kana_utils::katakana_to_hiragana("ソーセージ"));
    
    // Check exact readings
    println!("\nChecking readings for そうせいじ:");
    let results = find("そうせいじ");
    for word in results.iter().take(10) {
        println!("  {} ({})", word.text, word.reading);
    }
    
    println!("\nChecking readings for そうせえじ:");
    let results = find("そうせえじ");
    for word in results.iter().take(10) {
        println!("  {} ({})", word.text, word.reading);
    }
}