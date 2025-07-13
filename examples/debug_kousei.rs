use jaydar::{find_with_nhk, FindWithNhkResult};

fn main() {
    let result = find_with_nhk("こうせい");
    
    println!("Results for こうせい:");
    match result {
        FindWithNhkResult::MultipleMatches { homophones } => {
            for word in &homophones {
                println!("  Text: {}, Pitch: {:?}", 
                    word.text, word.pitch_accent);
            }
        }
        _ => println!("Unexpected result type"),
    }
    
    println!("\n\nResults for 構成:");
    let result2 = find_with_nhk("構成");
    match result2 {
        FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
            println!("  True homophones:");
            for word in &true_homophones {
                if word.text == "構成" || word.text == "後世" {
                    println!("    Text: {}, Pitch: {:?}", word.text, word.pitch_accent);
                }
            }
            println!("  Different pitch homophones:");
            for word in &different_pitch_homophones {
                if word.text == "構成" || word.text == "後世" {
                    println!("    Text: {}, Pitch: {:?}", word.text, word.pitch_accent);
                }
            }
        }
        _ => println!("Unexpected result type"),
    }
}