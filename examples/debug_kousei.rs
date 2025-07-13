use jaydar::{find_with_nhk, FindWithNhkResult};

fn main() {
    let results = find_with_nhk("こうせい");
    
    println!("Results for こうせい:");
    match &results {
        FindWithNhkResult::MultipleMatches { homophones } => {
            for word in homophones {
                println!("  Text: {}, Pitch: {:?}, True homophone: {}", 
                    word.text, word.pitch_accent, word.is_true_homophone);
            }
        }
        FindWithNhkResult::UniqueMatch { .. } => {
            println!("  Unexpected UniqueMatch for reading search");
        }
    }
    
    println!("\n\nResults for 構成:");
    let results2 = find_with_nhk("構成");
    match &results2 {
        FindWithNhkResult::UniqueMatch { pitch_accent, true_homophones, different_pitch_homophones } => {
            println!("  Target pitch: {:?}", pitch_accent);
            println!("  True homophones:");
            for word in true_homophones {
                if word.text == "構成" {
                    println!("    Text: {}, Pitch: {:?}", word.text, word.pitch_accent);
                }
            }
            println!("  Fake homophones:");
            for word in different_pitch_homophones {
                if word.text == "後世" {
                    println!("    Text: {}, Pitch: {:?}", word.text, word.pitch_accent);
                }
            }
        }
        FindWithNhkResult::MultipleMatches { .. } => {
            println!("  Unexpected MultipleMatches for specific word search");
        }
    }
}