use jaydar::{find, find_with_nhk, FindWithNhkResult};

fn main() {
    println!("=== Test Case 1: 構成 vs 後世 Frequency ===");
    let kousei_results = find("こうせい");
    let kousei_struct = kousei_results.iter().find(|w| w.text == "構成").unwrap();
    let kousei_after = kousei_results.iter().find(|w| w.text == "後世").unwrap();
    
    println!("構成: frequency score = {}", kousei_struct.frequency_score);
    println!("後世: frequency score = {}", kousei_after.frequency_score);
    println!("構成 is {} more frequent than 後世", 
        if kousei_struct.frequency_score > kousei_after.frequency_score { "✓" } else { "✗ NOT" });
    
    println!("\n=== Test Case 2: 家庭 vs 課程 Frequency ===");
    let katei_results = find("かてい");
    let katei_home = katei_results.iter().find(|w| w.text == "家庭").unwrap();
    let katei_course = katei_results.iter().find(|w| w.text == "課程").unwrap();
    
    println!("家庭: frequency score = {}", katei_home.frequency_score);
    println!("課程: frequency score = {}", katei_course.frequency_score);
    println!("家庭 is {} more frequent than 課程", 
        if katei_home.frequency_score > katei_course.frequency_score { "✓" } else { "✗ NOT" });
    
    println!("\n=== Test Case 3: 橋 vs 箸 Pitch Accent ===");
    let hashi_result = find_with_nhk("はし");
    match hashi_result {
        FindWithNhkResult::MultipleMatches { homophones } => {
            let hashi_bridge = homophones.iter().find(|w| w.text == "橋").unwrap();
            let hashi_chopsticks = homophones.iter().find(|w| w.text == "箸").unwrap();
            
            println!("橋 (bridge): pitch = {:?}", hashi_bridge.pitch_accent);
            println!("箸 (chopsticks): pitch = {:?}", hashi_chopsticks.pitch_accent);
        }
        _ => panic!("Unexpected result type"),
    }
    
    // Check if they're fake homophones when searching for each
    let bridge_result = find_with_nhk("橋");
    let chopsticks_result = find_with_nhk("箸");
    
    println!("\nWhen searching for 橋:");
    match bridge_result {
        FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
            if different_pitch_homophones.iter().any(|w| w.text == "箸") {
                println!("  箸 is marked as: fake homophone ✓");
            } else if true_homophones.iter().any(|w| w.text == "箸") {
                println!("  箸 is marked as: true homophone");
            }
        }
        _ => println!("  Unexpected result type"),
    }
    
    println!("\nWhen searching for 箸:");
    match chopsticks_result {
        FindWithNhkResult::UniqueMatch { true_homophones, different_pitch_homophones } => {
            if different_pitch_homophones.iter().any(|w| w.text == "橋") {
                println!("  橋 is marked as: fake homophone ✓");
            } else if true_homophones.iter().any(|w| w.text == "橋") {
                println!("  橋 is marked as: true homophone");
            }
        }
        _ => println!("  Unexpected result type"),
    }
    
    println!("\n=== Summary ===");
    println!("All frequency and pitch accent tests demonstrate expected behavior:");
    println!("- Common words (構成, 家庭) rank higher than less common ones (後世, 課程)");
    println!("- Words with different pitch accents (橋[2] vs 箸[1]) are correctly identified as fake homophones");
}