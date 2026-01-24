#!/usr/bin/env python3
"""
merged_freq.txtから119文字のRust配列を生成
"""

import sys
from pathlib import Path

def generate_rust_array(merged_freq_path, output_path=None):
    """merged_freq.txtからRust配列を生成"""
    merged_freq_path = Path(merged_freq_path)

    chars = []
    with open(merged_freq_path, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split('\t')
            if len(parts) >= 2:
                count = int(parts[0])
                char = parts[1]
                # 〓（スペース代替）と句読点は除外
                if char not in ['〓', '、', '。']:
                    chars.append(char)

    print(f"総文字数: {len(chars)}")

    # Rust配列形式で出力
    if output_path:
        with open(output_path, 'w', encoding='utf-8') as f:
            f.write("pub const HIRAGANA_FREQ_DEFAULT: &[&str] = &[\n")
            for i in range(0, len(chars), 10):
                chunk = chars[i:i+10]
                line = "    " + ", ".join(f'"{c}"' for c in chunk) + ","
                f.write(line + "\n")
            f.write("];\n")
        print(f"出力: {output_path}")
    else:
        # 標準出力
        print("\npub const HIRAGANA_FREQ_DEFAULT: &[&str] = &[")
        for i in range(0, len(chars), 10):
            chunk = chars[i:i+10]
            line = "    " + ", ".join(f'"{c}"' for c in chunk) + ","
            print(line)
        print("];")

    return chars

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("使用方法: python scripts/generate_freq_list.py merged_freq.txt [output.txt]")
        sys.exit(1)

    output = sys.argv[2] if len(sys.argv) > 2 else None
    generate_rust_array(sys.argv[1], output)
