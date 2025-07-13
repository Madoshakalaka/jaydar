use jaydar::find_with_nhk;

fn main() {
    let results = find_with_nhk("ソーセージ");
    
    println!("Results for ソーセージ:");
    for word in &results {
        println!("  Text: {}, Reading: {}, Pitch: {:?}", 
            word.text, word.reading, word.pitch_accent);
    }
}