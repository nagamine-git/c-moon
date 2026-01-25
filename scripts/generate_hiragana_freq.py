#!/usr/bin/env python3
"""
final_freq_min5.txt から Rust の &[&str] リストを生成
"""

import sys
from pathlib import Path

def generate_rust_str_array(freq_file, max_chars=140):
    """Rust の &[&str] 配列を生成"""
    freq_file = Path(freq_file)

    chars = []
    with open(freq_file, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split('\t')
            if len(parts) >= 2:
                count = int(parts[0])
                char = parts[1]
                # 〓（スペース代替）、、。は除外
                if char not in ['〓', '、', '。']:
                    chars.append((count, char))

    # 上位max_chars文字を使用
    chars = chars[:max_chars]

    print(f"使用する文字数: {len(chars)}")
    print(f"\n// {len(chars)}文字（1gram + 拗音2gram）")
    print("pub const HIRAGANA_FREQ_DEFAULT: &[&str] = &[")

    # 10文字ずつ改行
    for i in range(0, len(chars), 10):
        chunk = chars[i:i+10]
        line = "    " + ", ".join(f'"{c}"' for _, c in chunk) + ","
        print(line)

    print("];")

    # 統計情報
    gram1 = sum(1 for _, c in chars if len(c) == 1)
    gram2 = sum(1 for _, c in chars if len(c) == 2)
    print(f"\n// 統計: 1gram={gram1}, 2gram={gram2}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("使用方法: python scripts/generate_hiragana_freq.py final_freq_min5.txt [max_chars]")
        sys.exit(1)

    max_chars = int(sys.argv[2]) if len(sys.argv) > 2 else 140
    generate_rust_str_array(sys.argv[1], max_chars)
