use jaydar::{find, find_with_nhk};

fn main() {
    println!("=== Katakana Support Demo ===\n");
    
    // Test 1: カイ (katakana) finds kanji homophones
    println!("1. Searching for カイ (katakana chi from Greek χ):");
    let results = find("カイ");
    println!("Found {} homophones:", results.len());
    for (i, word) in results.iter().enumerate().take(10) {
        println!("   {}: {} ({}) - frequency: {}", 
            i+1, word.text, word.reading, word.frequency_score);
    }
    
    // Test 2: With pitch accent data
    println!("\n2. カイ with pitch accent data:");
    let results_pitch = find_with_nhk("カイ");
    println!("Top 5 results with pitch:");
    for word in results_pitch.iter().take(5) {
        println!("   {} - pitch: {:?}", word.text, word.pitch_accent);
    }
    
    // Test 3: Verify true/fake homophones
    println!("\n3. Searching from 会 (pitch 1) - checking fake homophones:");
    let results_kai = find_with_nhk("会");
    for word in results_kai.iter().filter(|w| ["会", "回", "階", "貝", "カイ"].contains(&w.text.as_str())) {
        println!("   {} - pitch: {:?}, true homophone: {}", 
            word.text, 
            word.pitch_accent,
            if word.is_true_homophone { "✓" } else { "✗ (different pitch)" });
    }
    
    // Test 4: Other katakana examples
    println!("\n4. Other katakana examples:");
    
    println!("\n   コウセイ (katakana):");
    let kousei_kata = find("コウセイ");
    for word in kousei_kata.iter().take(5) {
        println!("      {} ({})", word.text, word.reading);
    }
    
    println!("\n   ハシ (katakana):");
    let hashi_kata = find("ハシ");
    for word in hashi_kata.iter().take(5) {
        println!("      {} ({})", word.text, word.reading);
    }
}