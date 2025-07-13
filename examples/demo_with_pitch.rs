use homophone_ranker::{find_with_nhk, WordFrequencyWithPitch};

fn main() {
    // Example 1: Show pitch accent differences in こうせい
    println!("Homophones for 'こうせい' with pitch accent:");
    let results = find_with_nhk("こうせい");
    print_results_with_pitch(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 2: Search for 構成 (pitch 0) - shows "fake homophones"
    println!("Homophones for '構成' (pitch 0) - marking fake homophones:");
    let results = find_with_nhk("構成");
    print_results_with_pitch(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 3: Search for 後世 (pitch 1) - different fake homophones
    println!("Homophones for '後世' (pitch 1) - marking fake homophones:");
    let results = find_with_nhk("後世");
    print_results_with_pitch(&results);
    
    println!("\n{}\n", "=".repeat(70));
    
    // Example 4: Another common word with pitch variations
    println!("Homophones for 'はし' with pitch accent:");
    let results = find_with_nhk("はし");
    print_results_with_pitch(&results);
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
            let pitch_str = match word.pitch_accent {
                Some(p) => format!("{}", p),
                None => "?".to_string(),
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