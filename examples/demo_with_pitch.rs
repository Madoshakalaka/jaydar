use jaydar::{find_with_nhk, FindWithNhkResult, WordFrequencyWithPitch};

fn main() {
    // Example 1: Show pitch accent differences in こうせい
    println!("Homophones for 'こうせい' with pitch accent:");
    let result = find_with_nhk("こうせい");
    print_results_with_pitch(result);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 2: Search for 構成 (pitch 0) - shows "fake homophones"
    println!("Homophones for '構成' (pitch 0) - marking fake homophones:");
    let result = find_with_nhk("構成");
    print_results_with_pitch(result);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 3: Search for 後世 (pitch 1) - different fake homophones
    println!("Homophones for '後世' (pitch 1) - marking fake homophones:");
    let result = find_with_nhk("後世");
    print_results_with_pitch(result);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 4: Another common word with pitch variations
    println!("Homophones for 'はし' with pitch accent:");
    let result = find_with_nhk("はし");
    print_results_with_pitch(result);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 5: Words with no homophones
    println!("Testing words with no homophones:");
    println!("'後始末':");
    let result = find_with_nhk("後始末");
    print_results_with_pitch(result);
    
    println!("\n'タピオカ':");
    let result = find_with_nhk("タピオカ");
    print_results_with_pitch(result);
}

fn print_results_with_pitch(result: FindWithNhkResult) {
    match result {
        FindWithNhkResult::NoHomophones => {
            println!("This word has no homophones.");
        }
        FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
            println!("Unique match found!");
            
            if !true_homophones.is_empty() {
                println!("\nTrue homophones (same pitch):");
                print_words_table(&true_homophones);
            }
            
            if !different_pitch_homophones.is_empty() {
                println!("\nFake homophones (different pitch):");
                print_words_table(&different_pitch_homophones);
            }
            
            println!("\nSummary: {} true homophones, {} fake homophones", 
                true_homophones.len(), different_pitch_homophones.len());
        }
        FindWithNhkResult::MultipleMatches { homophones } => {
            println!("Multiple matches found (searched by reading):");
            print_words_table(&homophones);
            println!("\nTotal homophones: {}", homophones.len());
        }
    }
}

fn print_words_table(words: &[WordFrequencyWithPitch]) {
    println!("{:<12} {:<15} {:<10} {:<8} {:<10}", 
        "Text", "Reading", "Frequency", "Common?", "Pitch");
    println!("{}", "-".repeat(55));
    
    for (i, word) in words.iter().enumerate() {
        if i < 15 {  // Show top 15
            let pitch_str = if word.pitch_accent.is_empty() {
                "?".to_string()
            } else {
                word.pitch_accent.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            
            println!(
                "{:<12} {:<15} {:<10} {:<8} {:<10}",
                word.text,
                word.reading,
                word.frequency_score,
                if word.is_common { "Yes" } else { "No" },
                pitch_str
            );
        }
    }
    
    if words.len() > 15 {
        println!("... and {} more", words.len() - 15);
    }
}