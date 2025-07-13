use jaydar::find;

fn main() {
    // Example 1: Find homophones for a kana word
    println!("Homophones for 'かう':");
    let results = find("かう");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(50));
    
    // Example 2: Find homophones for a kanji word
    println!("Homophones for '聞く':");
    let results = find("聞く");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(50));
    
    // Example 3: Find homophones for another common word
    println!("Homophones for 'はし':");
    let results = find("はし");
    print_results(&results);
    
    println!("\n{}\n", "=".repeat(50));
    
    // Example 4: Find homophones for 'こうせい' (many homophones)
    println!("Homophones for 'こうせい':");
    let results = find("こうせい");
    print_results(&results);
}

fn print_results(results: &[jaydar::WordFrequency]) {
    if results.is_empty() {
        println!("No homophones found.");
        return;
    }
    
    println!("Found {} homophones:", results.len());
    println!("{:<10} {:<15} {:<15} {:<10}", "Text", "Reading", "Frequency", "Common?");
    println!("{}", "-".repeat(50));
    
    for (i, word) in results.iter().enumerate() {
        if i < 10 {  // Show top 10
            println!(
                "{:<10} {:<15} {:<15} {:<10}",
                word.text,
                word.reading,
                word.frequency_score,
                if word.is_common { "Yes" } else { "No" }
            );
        }
    }
    
    if results.len() > 10 {
        println!("... and {} more", results.len() - 10);
    }
}