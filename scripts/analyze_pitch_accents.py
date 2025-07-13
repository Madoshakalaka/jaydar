#!/usr/bin/env python3
"""Script to analyze NHK pitch accent data structure and find words with multiple pitch accents."""

import json
import sys
from collections import defaultdict

def analyze_nhk_data(filename):
    """Analyze NHK data to understand the pitch accent structure."""
    
    with open(filename, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    # Statistics
    total_entries = len(data)
    entries_with_multiple_accents = 0
    entries_with_multiple_pitches_in_accent = 0
    max_accents = 0
    max_pitches_in_accent = 0
    
    # Examples
    examples_multiple_accents = []
    examples_multiple_pitches = []
    
    # Check for 程度
    teido_entries = []
    
    for entry in data:
        word = entry.get('kana', '')
        kanji_list = entry.get('kanji', [])
        
        # Check for 程度
        if word == 'ていど' or '程度' in kanji_list:
            teido_entries.append(entry)
        
        accents = entry.get('accents', [])
        
        if len(accents) > 1:
            entries_with_multiple_accents += 1
            if len(examples_multiple_accents) < 5:
                examples_multiple_accents.append({
                    'kana': word,
                    'kanji': kanji_list,
                    'accents': accents
                })
        
        max_accents = max(max_accents, len(accents))
        
        # Check pitch patterns within each accent
        for accent in accents:
            accent_list = accent.get('accent', [])
            if len(accent_list) > 1:
                entries_with_multiple_pitches_in_accent += 1
                if len(examples_multiple_pitches) < 5:
                    examples_multiple_pitches.append({
                        'kana': word,
                        'kanji': kanji_list,
                        'accent': accent_list
                    })
            max_pitches_in_accent = max(max_pitches_in_accent, len(accent_list))
    
    print(f"Total entries: {total_entries}")
    print(f"Entries with multiple accent groups: {entries_with_multiple_accents}")
    print(f"Entries with multiple pitches in a single accent: {entries_with_multiple_pitches_in_accent}")
    print(f"Max accent groups in an entry: {max_accents}")
    print(f"Max pitch patterns in an accent: {max_pitches_in_accent}")
    
    print("\n=== Examples of entries with multiple accent groups ===")
    for ex in examples_multiple_accents[:3]:
        print(f"\nWord: {ex['kana']} ({', '.join(ex['kanji'])})")
        for i, accent in enumerate(ex['accents']):
            print(f"  Accent group {i+1}:")
            for a in accent.get('accent', []):
                print(f"    - Pitch: {a.get('pitchAccent')}, Pronunciation: {a.get('pronunciation')}")
    
    print("\n=== Examples of entries with multiple pitch patterns in single accent ===")
    for ex in examples_multiple_pitches[:3]:
        print(f"\nWord: {ex['kana']} ({', '.join(ex['kanji'])})")
        print(f"  Pitch patterns:")
        for a in ex['accent']:
            print(f"    - Pitch: {a.get('pitchAccent')}, Pronunciation: {a.get('pronunciation')}")
    
    print("\n=== 程度 (ていど) entries ===")
    for entry in teido_entries:
        print(f"\nWord: {entry['kana']} ({', '.join(entry.get('kanji', []))})")
        for i, accent in enumerate(entry.get('accents', [])):
            print(f"  Accent group {i+1}:")
            for a in accent.get('accent', []):
                print(f"    - Pitch: {a.get('pitchAccent')}, Pronunciation: {a.get('pronunciation')}")

if __name__ == "__main__":
    analyze_nhk_data("nhk_16_entries.json")