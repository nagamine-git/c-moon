#!/usr/bin/env python3
"""
1gramと拗音2gram（頻度でフィルタリング）をマージして最終頻度リストを作成
"""

import sys
from pathlib import Path

def is_small_kana(c):
    """拗音・小書き文字かどうか"""
    small_hiragana = 'ぁぃぅぇぉゃゅょゎ'
    small_katakana = 'ァィゥェォヵヶャュョヮ'
    return c in small_hiragana or c in small_katakana

def create_final_list(ngram1_path, ngram2_path, min_frequency=5):
    """最終リスト作成"""
    ngram1_path = Path(ngram1_path)
    ngram2_path = Path(ngram2_path)

    # 1gramを読み込み（拗音単体を除外）
    chars_1gram = []
    with open(ngram1_path, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split('\t')
            if len(parts) >= 2:
                count = int(parts[0])
                char = parts[1]
                # 拗音単体は除外（2gramで使うため）
                # ただし「ょ」「ゅ」「ゃ」は文末で単独使用されることがあるので含める可能性も
                # 今回は安全のため、小書き単体は除外
                if char not in 'ょゅゃぁぃぅぇぉゎァィゥェォヵヶャュョヮ':
                    chars_1gram.append((count, char, 1))

    print(f"1gram: {len(chars_1gram)}文字")

    # 拗音2gramを読み込み（頻度フィルタリング）
    chars_2gram = []
    with open(ngram2_path, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split('\t')
            if len(parts) >= 2:
                count = int(parts[0])
                ngram = parts[1]

                # 2文字で拗音終わり、かつ最小頻度以上
                if len(ngram) == 2 and is_small_kana(ngram[1]) and count >= min_frequency:
                    chars_2gram.append((count, ngram, 2))

    print(f"拗音2gram (頻度>={min_frequency}): {len(chars_2gram)}文字")

    # マージして頻度順にソート
    all_chars = chars_1gram + chars_2gram
    all_chars.sort(key=lambda x: -x[0])

    print(f"合計: {len(all_chars)}文字")

    # 出力
    output_path = ngram1_path.parent / f"final_freq_min{min_frequency}.txt"
    with open(output_path, 'w', encoding='utf-8') as f:
        for count, char, n in all_chars:
            f.write(f"{count}\t{char}\t{n}\n")

    print(f"出力: {output_path}")

    # トップ100を表示
    print("\nトップ100（頻度順）:")
    for i, (count, char, n) in enumerate(all_chars[:100], 1):
        char_type = "2g" if n == 2 else "1g"
        print(f"{i:3d}. {char:6s} ({count:6d}) [{char_type}]")

    # 統計情報
    gram1_count = sum(1 for _, _, n in all_chars if n == 1)
    gram2_count = sum(1 for _, _, n in all_chars if n == 2)
    print(f"\n統計: 1gram={gram1_count}, 2gram={gram2_count}")

    return all_chars

if __name__ == '__main__':
    if len(sys.argv) < 3:
        print("使用方法: python scripts/create_final_freq_list.py 1gram.txt 2gram.txt [min_freq]")
        sys.exit(1)

    min_freq = int(sys.argv[3]) if len(sys.argv) > 3 else 5
    create_final_list(sys.argv[1], sys.argv[2], min_freq)
