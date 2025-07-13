/// Convert katakana to hiragana with proper long vowel normalization
pub fn katakana_to_hiragana(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    
    for (i, &c) in chars.iter().enumerate() {
        match c {
            'ア'..='ン' => {
                // Convert katakana to hiragana by subtracting the offset
                // Katakana ア (U+30A2) -> Hiragana あ (U+3042)
                result.push(char::from_u32((c as u32) - 0x60).unwrap_or(c));
            }
            'ヴ' => result.push('ゔ'), // Special case for vu
            'ァ'..='ヶ' => {
                // Small katakana: ァ (U+30A1) -> ぁ (U+3041)
                result.push(char::from_u32((c as u32) - 0x60).unwrap_or(c));
            }
            'ー' => {
                // Long vowel mark - replace based on previous character's vowel
                if i > 0 {
                    let prev = chars[i - 1];
                    let long_vowel = match prev {
                        // あ-row: ア カ ガ サ ザ タ ダ ナ ハ バ パ マ ヤ ラ ワ
                        'ア' | 'カ' | 'ガ' | 'サ' | 'ザ' | 'タ' | 'ダ' | 'ナ' | 'ハ' | 'バ' | 'パ' | 'マ' | 'ヤ' | 'ラ' | 'ワ' => 'あ',
                        // い-row: イ キ ギ シ ジ チ ヂ ニ ヒ ビ ピ ミ リ
                        'イ' | 'キ' | 'ギ' | 'シ' | 'ジ' | 'チ' | 'ヂ' | 'ニ' | 'ヒ' | 'ビ' | 'ピ' | 'ミ' | 'リ' => 'い',
                        // う-row: ウ ク グ ス ズ ツ ヅ ヌ フ ブ プ ム ユ ル
                        'ウ' | 'ク' | 'グ' | 'ス' | 'ズ' | 'ツ' | 'ヅ' | 'ヌ' | 'フ' | 'ブ' | 'プ' | 'ム' | 'ユ' | 'ル' => 'う',
                        // え-row: エ ケ ゲ セ ゼ テ デ ネ ヘ ベ ペ メ レ
                        'エ' | 'ケ' | 'ゲ' | 'セ' | 'ゼ' | 'テ' | 'デ' | 'ネ' | 'ヘ' | 'ベ' | 'ペ' | 'メ' | 'レ' => 'い',
                        // お-row: オ コ ゴ ソ ゾ ト ド ノ ホ ボ ポ モ ヨ ロ ヲ
                        'オ' | 'コ' | 'ゴ' | 'ソ' | 'ゾ' | 'ト' | 'ド' | 'ノ' | 'ホ' | 'ボ' | 'ポ' | 'モ' | 'ヨ' | 'ロ' | 'ヲ' => 'う',
                        // ん doesn't take long vowels
                        'ン' => 'ー', // Keep as is
                        // Already hiragana? Check hiragana too
                        'あ' | 'か' | 'が' | 'さ' | 'ざ' | 'た' | 'だ' | 'な' | 'は' | 'ば' | 'ぱ' | 'ま' | 'や' | 'ら' | 'わ' => 'あ',
                        'い' | 'き' | 'ぎ' | 'し' | 'じ' | 'ち' | 'ぢ' | 'に' | 'ひ' | 'び' | 'ぴ' | 'み' | 'り' => 'い',
                        'う' | 'く' | 'ぐ' | 'す' | 'ず' | 'つ' | 'づ' | 'ぬ' | 'ふ' | 'ぶ' | 'ぷ' | 'む' | 'ゆ' | 'る' => 'う',
                        'え' | 'け' | 'げ' | 'せ' | 'ぜ' | 'て' | 'で' | 'ね' | 'へ' | 'べ' | 'ぺ' | 'め' | 'れ' => 'い',
                        'お' | 'こ' | 'ご' | 'そ' | 'ぞ' | 'と' | 'ど' | 'の' | 'ほ' | 'ぼ' | 'ぽ' | 'も' | 'よ' | 'ろ' | 'を' => 'う',
                        _ => 'ー', // Keep as is if unknown
                    };
                    result.push(long_vowel);
                } else {
                    result.push('ー'); // Keep as is if at start
                }
            }
            _ => result.push(c),
        }
    }
    
    result
}

/// Check if a string contains katakana
pub fn contains_katakana(s: &str) -> bool {
    s.chars().any(|c| matches!(c, 'ア'..='ン' | 'ヴ' | 'ァ'..='ヶ'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_katakana_to_hiragana() {
        assert_eq!(katakana_to_hiragana("カイ"), "かい");
        assert_eq!(katakana_to_hiragana("コウセイ"), "こうせい");
        assert_eq!(katakana_to_hiragana("ハシ"), "はし");
        assert_eq!(katakana_to_hiragana("カタカナ"), "かたかな");
        assert_eq!(katakana_to_hiragana("ヴァイオリン"), "ゔぁいおりん");
        
        // Mixed text should leave non-katakana unchanged
        assert_eq!(katakana_to_hiragana("カイ会"), "かい会");
        assert_eq!(katakana_to_hiragana("hello カイ"), "hello かい");
        
        // Test long vowel normalization
        assert_eq!(katakana_to_hiragana("ソーセージ"), "そうせいじ"); // セー → せい
        assert_eq!(katakana_to_hiragana("コーヒー"), "こうひい"); // コー → こう, ヒー → ひい
        assert_eq!(katakana_to_hiragana("スーパー"), "すうぱあ"); // スー → すう, パー → ぱあ
        assert_eq!(katakana_to_hiragana("ケーキ"), "けいき"); // ケー → けい
        assert_eq!(katakana_to_hiragana("ボール"), "ぼうる"); // ボー → ぼう
        assert_eq!(katakana_to_hiragana("メール"), "めいる"); // メー → めい
        assert_eq!(katakana_to_hiragana("エレベーター"), "えれべいたあ"); // ベー → べい, ター → たあ
    }
    
    #[test]
    fn test_contains_katakana() {
        assert!(contains_katakana("カイ"));
        assert!(contains_katakana("カタカナ"));
        assert!(contains_katakana("hello カイ"));
        assert!(!contains_katakana("かい"));
        assert!(!contains_katakana("漢字"));
        assert!(!contains_katakana("hello"));
    }
}