use jaydar::{find_with_nhk, FindWithNhkResult};

fn main() {
    println!("Testing 程度 (ていど) for multiple pitch accents:");
    
    let results = find_with_nhk("ていど");
    
    match &results {
        FindWithNhkResult::MultipleMatches { homophones } => {
            for word in homophones {
                if word.text == "程度" {
                    println!("\nFound 程度:");
                    println!("  Reading: {}", word.reading);
                    println!("  Pitch accents: {:?}", word.pitch_accent);
                    println!("  Frequency: {}", word.frequency_score);
                    println!("  Common: {}", word.is_common);
                    
                    if word.pitch_accent.len() > 1 {
                        println!("  ✓ Successfully has multiple pitch accents!");
                    } else {
                        println!("  ✗ Only has {} pitch accent(s)", word.pitch_accent.len());
                    }
                    
                    if word.pitch_accent.contains(&1) && word.pitch_accent.contains(&0) {
                        println!("  ✓ Has both pitch accents 1 and 0 as expected!");
                    }
                }
            }
            
            println!("\nAll homophones of ていど:");
            for (i, word) in homophones.iter().enumerate() {
                if i < 10 {
                    let pitch_str = if word.pitch_accent.is_empty() {
                        "?".to_string()
                    } else {
                        word.pitch_accent.iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    };
                    
                    println!("{:<10} ({:<10}) - pitch: {:<10} freq: {:<8} {}",
                        word.text,
                        word.reading,
                        pitch_str,
                        word.frequency_score,
                        if word.is_true_homophone { "✓" } else { "✗" }
                    );
                }
            }
        }
        FindWithNhkResult::UniqueMatch { .. } => {
            println!("Unexpected: Got UniqueMatch for reading search");
        }
    }
}