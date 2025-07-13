use jaydar::{find_with_nhk, FindWithNhkResult};

fn main() {
    let result = find_with_nhk("ソーセージ");
    
    println!("Results for ソーセージ:");
    match result {
        FindWithNhkResult::NoHomophones => {
            println!("  ソーセージ has no homophones");
        }
        FindWithNhkResult::UniqueMatch { true_homophones, .. } => {
            for word in &true_homophones {
                println!("  Text: {}, Reading: {}, Pitch: {:?}", 
                    word.text, word.reading, word.pitch_accent);
            }
        }
        FindWithNhkResult::MultipleMatches { homophones } => {
            for word in &homophones {
                println!("  Text: {}, Reading: {}, Pitch: {:?}", 
                    word.text, word.reading, word.pitch_accent);
            }
        }
    }
}