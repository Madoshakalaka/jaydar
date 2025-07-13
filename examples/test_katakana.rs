use homophone_ranker::find;

fn main() {
    // First, let's see what happens when we search for カイ
    println!("=== Searching for カイ (katakana) ===");
    let results_katakana = find("カイ");
    println!("Found {} results for カイ", results_katakana.len());
    for (i, word) in results_katakana.iter().enumerate().take(10) {
        println!("{}: {} ({}) - score: {}", 
            i+1, word.text, word.reading, word.frequency_score);
    }
    
    // Now search for かい (hiragana)
    println!("\n=== Searching for かい (hiragana) ===");
    let results_hiragana = find("かい");
    println!("Found {} results for かい", results_hiragana.len());
    for (i, word) in results_hiragana.iter().enumerate().take(10) {
        println!("{}: {} ({}) - score: {}", 
            i+1, word.text, word.reading, word.frequency_score);
    }
    
    // Check if we find the expected kanji
    let expected = ["会", "回", "階", "貝", "海"];
    println!("\n=== Checking for expected kanji ===");
    for kanji in &expected {
        let found_kata = results_katakana.iter().any(|w| w.text == *kanji);
        let found_hira = results_hiragana.iter().any(|w| w.text == *kanji);
        println!("{}: found with カイ={}, found with かい={}", 
            kanji, found_kata, found_hira);
    }
}