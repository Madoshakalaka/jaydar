#!/usr/bin/env python3
"""Analyze NHK pitch accent data structure."""

import json
from collections import defaultdict

def analyze_structure():
    with open('nhk_16_entries.json', 'r') as f:
        data = json.load(f)
    
    print(f"Total entries: {len(data)}")
    
    # Analyze structure
    print("\nSample entry structure:")
    sample = data[0]
    for key in sample.keys():
        value = sample[key]
        if isinstance(value, list) and len(value) > 0:
            print(f"  {key}: {type(value).__name__} of {type(value[0]).__name__} (length: {len(value)})")
        else:
            print(f"  {key}: {type(value).__name__}")
    
    # Analyze pitch patterns
    pitch_counts = defaultdict(int)
    multi_pitch_count = 0
    
    for entry in data:
        if entry['accents']:
            if len(entry['accents']) > 1:
                multi_pitch_count += 1
            for accent_group in entry['accents']:
                for accent in accent_group['accent']:
                    pitch_counts[accent['pitchAccent']] += 1
    
    print(f"\nPitch accent distribution:")
    for pitch, count in sorted(pitch_counts.items()):
        print(f"  Pitch {pitch}: {count} entries")
    print(f"\nEntries with multiple accent patterns: {multi_pitch_count}")
    
    # Find some homophones
    kana_to_entries = defaultdict(list)
    for entry in data:
        kana = entry['kana']
        kana_to_entries[kana].append(entry)
    
    # Find kana with most homophones
    homophone_counts = [(kana, len(entries)) for kana, entries in kana_to_entries.items() if len(entries) > 1]
    homophone_counts.sort(key=lambda x: x[1], reverse=True)
    
    print(f"\nTop 10 readings with most homophones:")
    for kana, count in homophone_counts[:10]:
        print(f"  {kana}: {count} different words")
        # Show examples
        entries = kana_to_entries[kana][:3]
        for entry in entries:
            kanji = entry.get('kanji', ['(kana only)'])
            pitch = entry['accents'][0]['accent'][0]['pitchAccent'] if entry['accents'] else 'N/A'
            print(f"    - {kanji[0] if kanji else '(none)'} (pitch: {pitch})")

def analyze_kousei():
    """Specifically analyze こうせい entries."""
    with open('nhk_16_entries.json', 'r') as f:
        data = json.load(f)
    
    kousei_entries = [entry for entry in data if entry['kana'] == 'こうせい']
    
    print(f"\n\nAnalysis of こうせい ({len(kousei_entries)} entries):")
    print("-" * 50)
    
    for entry in kousei_entries:
        kanji = entry.get('kanji', [])
        if entry['accents']:
            pitch = entry['accents'][0]['accent'][0]['pitchAccent']
            pronunciation = entry['accents'][0]['accent'][0]['pronunciation']
        else:
            pitch = 'N/A'
            pronunciation = 'N/A'
        
        print(f"\n{kanji[0] if kanji else '(kana only)'}")
        print(f"  ID: {entry['id']}")
        print(f"  Pitch: {pitch}")
        print(f"  Pronunciation: {pronunciation}")
        if entry.get('usage'):
            print(f"  Usage: {entry['usage']}")
        if entry.get('category'):
            print(f"  Category: {entry['category']}")

if __name__ == "__main__":
    analyze_structure()
    analyze_kousei()