use jaydar::{find_with_nhk, FindWithNhkResult};

fn main() {
    let results = find_with_nhk("にほんご");
    
    match &results {
        FindWithNhkResult::UniqueMatch { pitch_accent, true_homophones, different_pitch_homophones } => {
            println!("Got UniqueMatch for にほんご");
            println!("Pitch accent: {:?}", pitch_accent);
            println!("True homophones: {} words", true_homophones.len());
            println!("Different pitch homophones: {} words", different_pitch_homophones.len());
            
            for word in true_homophones {
                println!("  {} ({}) - pitch: {:?}", word.text, word.reading, word.pitch_accent);
            }
        }
        FindWithNhkResult::MultipleMatches { homophones } => {
            println!("Got MultipleMatches for にほんご");
            println!("Found {} homophones", homophones.len());
            for word in homophones {
                println!("  {} ({}) - pitch: {:?}", word.text, word.reading, word.pitch_accent);
            }
        }
    }
}