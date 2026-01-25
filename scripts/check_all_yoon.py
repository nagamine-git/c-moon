#!/usr/bin/env python3
"""
2gramファイルから全ての拗音・小書き文字終わりを抽出して分類
"""

import sys
from pathlib import Path

def is_small_kana(c):
    """拗音・小書き文字かどうか"""
    small_hiragana = 'ぁぃぅぇぉゃゅょゎ'
    small_katakana = 'ァィゥェォヵヶャュョヮ'
    return c in small_hiragana or c in small_katakana

def classify_yoon(ngram):
    """拗音の分類"""
    if len(ngram) != 2:
        return "その他"

    first = ngram[0]
    second = ngram[1]

    # 拗音パターン分類
    if second in 'ゃゅょ':
        return "や行拗音"
    elif second in 'ぁぃぅぇぉ':
        return "小書きあ行"
    elif second in 'ゎ':
        return "小書きわ"
    elif second in 'ャュョ':
        return "カタカナ拗音"
    elif second in 'ァィゥェォヮ':
        return "小書きカタカナ"
    else:
        return "その他"

def extract_all_yoon(ngram2_path):
    """全拗音終わりを抽出"""
    ngram2_path = Path(ngram2_path)

    yoon_data = {
        "や行拗音": [],
        "小書きあ行": [],
        "小書きわ": [],
        "カタカナ拗音": [],
        "小書きカタカナ": [],
        "その他": []
    }

    with open(ngram2_path, 'r', encoding='utf-8') as f:
        for line in f:
            parts = line.strip().split('\t')
            if len(parts) >= 2:
                count = int(parts[0])
                ngram = parts[1]

                # 2文字で拗音・小書き文字終わり
                if len(ngram) == 2 and is_small_kana(ngram[1]):
                    category = classify_yoon(ngram)
                    yoon_data[category].append((count, ngram))

    # 結果表示
    total = 0
    for category, items in yoon_data.items():
        if items:
            print(f"\n## {category} ({len(items)}種類)")
            for count, ngram in sorted(items, key=lambda x: -x[0]):
                print(f"  {ngram:4s} ({count:5d})")
                total += 1

    print(f"\n合計: {total}種類")

    # 抜けている可能性のある拗音を提案
    print("\n## 期待される拗音パターン（参考）")
    print("や行拗音: きゃきゅきょ、しゃしゅしょ、ちゃちゅちょ、にゃにゅにょ、")
    print("         ひゃひゅひょ、みゃみゅみょ、りゃりゅりょ、ぎゃぎゅぎょ、")
    print("         じゃじゅじょ、びゃびゅびょ、ぴゃぴゅぴょ")
    print("小書きあ行: ティ、ディ、ファ、フィ、フェ、フォ、ヴァ、ヴィ、ヴェ、ヴォ")
    print("         ウィ、ウェ、ウォ、ツァ、ツィ、ツェ、ツォ、シェ、ジェ、チェ")

    return yoon_data

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("使用方法: python scripts/check_all_yoon.py 2gram.txt")
        sys.exit(1)

    extract_all_yoon(sys.argv[1])
