#!/usr/bin/env python3
"""
新戦略で136文字の配置リストを生成

戦略:
1. 既存1gram: 77個
2. 小書き追加（ぁぃぅぇぉ）: 5個
3. や行拗音2gram（ゃゅょ終わり）: 34個
4. 小書きあ行2gram（ぁぃぅぇぉ終わり、頻度>=16）: 20個
合計: 136個
"""

# final_freq_min5.txtから読み込み
with open('../final_freq_min5.txt', 'r') as f:
    lines = [line.strip().split('\t') for line in f if line.strip()]
    all_chars = [(char, int(freq)) for freq, char, _ in lines]

# 1gramと2gramを分離（固定位置の文字を除外）
onegrams = [(c, f) for c, f in all_chars if len(c) == 1 and c not in ['、', '。', '〓', '；', '・']]
twograms = [(c, f) for c, f in all_chars if len(c) == 2]

# 2gramを分類
ya_yoon = [(c, f) for c, f in twograms if c[1] in 'ゃゅょ']
small_a_2gram = [(c, f) for c, f in twograms if c[1] in 'ぁぃぅぇぉ']

# 必要な小書きあ行2gram数を計算
target_total = 136
base_count = len(onegrams) + 5 + len(ya_yoon)  # 1gram + 小書き5個 + や行拗音
needed_small_a = target_total - base_count

# 小書きあ行2gramから必要な数だけ取得
small_a_sorted = sorted(small_a_2gram, key=lambda x: x[1], reverse=True)
small_a_selected = small_a_sorted[:needed_small_a]

print(f"1gram: {len(onegrams)}")
print(f"小書き追加: 5")
print(f"や行拗音2gram: {len(ya_yoon)}")
print(f"小書きあ行2gram（上位{needed_small_a}個）: {len(small_a_selected)}")
print(f"合計: {len(onegrams) + 5 + len(ya_yoon) + len(small_a_selected)}")

# 最終リスト作成（頻度順にソート）
final_list = []

# 1. 既存1gram
final_list.extend(onegrams)

# 2. 小書き文字追加（頻度0）
for c in ['ぁ', 'ぃ', 'ぅ', 'ぇ', 'ぉ']:
    final_list.append((c, 0))

# 3. や行拗音2gram
final_list.extend(ya_yoon)

# 4. 小書きあ行2gram
final_list.extend(small_a_selected)

# 頻度順にソート（0は最後に）
final_list_sorted = sorted(final_list, key=lambda x: (x[1] == 0, -x[1]))

# ファイル出力
with open('../final_136chars.txt', 'w') as f:
    for char, freq in final_list_sorted:
        f.write(f"{freq}\t{char}\t1\n")

print(f"\n生成完了: final_136chars.txt ({len(final_list)}文字)")

# 統計情報
print("\n=== 統計 ===")
print(f"1gram: {len([c for c, f in final_list if len(c) == 1])}個")
print(f"2gram: {len([c for c, f in final_list if len(c) == 2])}個")
print(f"  - ゃゅょ終わり: {len([c for c, f in final_list if len(c) == 2 and c[1] in 'ゃゅょ'])}個")
print(f"  - ぁぃぅぇぉ終わり: {len([c for c, f in final_list if len(c) == 2 and c[1] in 'ぁぃぅぇぉ'])}個")

print("\n小書きあ行2gram（採用）:")
for c, f in small_a_selected:
    print(f"  {c}: {f:,}")
