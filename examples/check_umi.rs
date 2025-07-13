use jaydar::find;

fn main() {
    // Check if 海 has reading かい
    println!("Searching for 海:");
    let results = find("海");
    for word in results.iter().take(5) {
        println!("  {} ({})", word.text, word.reading);
    }
    
    // Check readings for うみ
    println!("\nSearching for うみ:");
    let results = find("うみ");
    for word in results.iter().take(5) {
        println!("  {} ({})", word.text, word.reading);
    }
    
    // Check specific entry for 海
    for entry in jmdict::entries() {
        for kanji in entry.kanji_elements() {
            if kanji.text == "海" {
                println!("\nFound 海 with readings:");
                for reading in entry.reading_elements() {
                    println!("  {}", reading.text);
                }
                break;
            }
        }
    }
}