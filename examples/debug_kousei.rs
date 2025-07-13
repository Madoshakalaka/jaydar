use jaydar::find_with_nhk;

fn main() {
    let results = find_with_nhk("こうせい");
    
    println!("Results for こうせい:");
    for word in &results {
        println!("  Text: {}, Pitch: {:?}, True homophone: {}", 
            word.text, word.pitch_accent, word.is_true_homophone);
    }
    
    println!("\n\nResults for 構成:");
    let results2 = find_with_nhk("構成");
    for word in &results2 {
        if word.text == "構成" || word.text == "後世" {
            println!("  Text: {}, Pitch: {:?}, True homophone: {}", 
                word.text, word.pitch_accent, word.is_true_homophone);
        }
    }
}