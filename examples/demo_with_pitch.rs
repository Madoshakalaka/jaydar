use jaydar::{find_with_nhk, FindWithNhkResult, WordFrequencyWithPitch};

fn main() {
    // Example 1: Show pitch accent differences in こうせい
    println!("Homophones for 'こうせい' with pitch accent:");
    let results = find_with_nhk("こうせい");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 2: Search for 構成 (pitch 0) - shows "fake homophones"
    println!("Homophones for '構成' (pitch 0) - marking fake homophones:");
    let results = find_with_nhk("構成");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 3: Search for 後世 (pitch 1) - different fake homophones
    println!("Homophones for '後世' (pitch 1) - marking fake homophones:");
    let results = find_with_nhk("後世");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 4: Another common word with pitch variations
    println!("Homophones for 'はし' with pitch accent:");
    let results = find_with_nhk("はし");
    print_results(&results);
}

fn print_results(results: &FindWithNhkResult) {
    match results {
        FindWithNhkResult::UniqueMatch { pitch_accent, true_homophones, different_pitch_homophones } => {
            println!("Type: Unique match (searched for specific word)");
            println!("Target word pitch accent: {:?}", pitch_accent);
            println!("\nTrue homophones (same pitch):");
            print_results_with_pitch(true_homophones);
            
            if !different_pitch_homophones.is_empty() {
                println!("\nFake homophones (different pitch):");
                print_results_with_pitch(different_pitch_homophones);
            }
        }
        FindWithNhkResult::MultipleMatches { homophones } => {
            println!("Type: Multiple matches (searched by reading)");
            print_results_with_pitch(homophones);
        }
    }
}

fn print_results_with_pitch(results: &[WordFrequencyWithPitch]) {
    if results.is_empty() {
        println!("No homophones found.");
        return;
    }
    
    println!("Found {} homophones:", results.len());
    println!("{:<12} {:<15} {:<10} {:<8} {:<10} {:<15}", 
        "Text", "Reading", "Frequency", "Common?", "Pitch", "True Homophone?");
    println!("{}", "-".repeat(70));
    
    for (i, word) in results.iter().enumerate() {
        if i < 15 {  // Show top 15
            let pitch_str = if word.pitch_accent.is_empty() {
                "?".to_string()
            } else {
                word.pitch_accent.iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            
            let homophone_str = if word.is_true_homophone {
                "✓"
            } else {
                "✗ (fake)"
            };
            
            println!(
                "{:<12} {:<15} {:<10} {:<8} {:<10} {:<15}",
                word.text,
                word.reading,
                word.frequency_score,
                if word.is_common { "Yes" } else { "No" },
                pitch_str,
                homophone_str
            );
        }
    }
    
    if results.len() > 15 {
        println!("... and {} more", results.len() - 15);
    }
    
    // Count true vs fake homophones
    let true_count = results.iter().filter(|w| w.is_true_homophone).count();
    let fake_count = results.len() - true_count;
    println!("\nSummary: {} true homophones, {} fake homophones (different pitch)", 
        true_count, fake_count);
}