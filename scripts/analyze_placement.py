#!/usr/bin/env python3
"""
136文字の配置を頻度順に分析
ホームポジションと各層への配置が適切かチェック
"""

# 頻度リストから読み込み
with open('../final_136chars.txt', 'r') as f:
    lines = [line.strip().split('\t') for line in f if line.strip()]
    all_chars = [(char, int(freq)) for freq, char, _ in lines]

# 頻度順にソート（頻度0を除く）
chars_with_freq = [(c, f) for c, f in all_chars if f > 0]
chars_sorted = sorted(chars_with_freq, key=lambda x: x[1], reverse=True)

print("=== 136文字の頻度分布分析 ===\n")

# 層ごとの推奨ポジション数
# Layer 0: 24 (30 - 6固定)
# Layer 1: 28 (30 - 2固定)
# Layer 2: 28 (30 - 2Ver)
# Layer 3: 28 (30 - 2Ver)
# Layer 4: 28 (30 - 2Ver)

layer_capacity = {
    0: 24,
    1: 28,
    2: 28,
    3: 28,
    4: 28
}

# 層ごとのコスト
layer_weights = {
    0: 1.0,
    1: 2.0,
    2: 2.0,
    3: 2.3,
    4: 2.3
}

print("## 層ごとの推奨配置\n")

total_assigned = 0
for layer in range(5):
    capacity = layer_capacity[layer]
    start = total_assigned
    end = total_assigned + capacity

    layer_chars = chars_sorted[start:end]

    print(f"### Layer {layer} (コスト: {layer_weights[layer]}, 容量: {capacity}枠)")

    if layer_chars:
        freq_range = f"{layer_chars[0][1]:,} 〜 {layer_chars[-1][1]:,}"
        avg_freq = sum(f for _, f in layer_chars) / len(layer_chars)
        total_freq = sum(f for _, f in layer_chars)

        print(f"頻度範囲: {freq_range}")
        print(f"平均頻度: {avg_freq:,.0f}")
        print(f"合計頻度: {total_freq:,}")

        # 1gramと2gramの内訳
        gram1 = [(c, f) for c, f in layer_chars if len(c) == 1]
        gram2 = [(c, f) for c, f in layer_chars if len(c) == 2]

        print(f"内訳: 1gram={len(gram1)}個, 2gram={len(gram2)}個")

        # 上位5個と下位5個を表示
        print(f"上位5個:")
        for i, (c, f) in enumerate(layer_chars[:5], 1):
            char_type = "1g" if len(c) == 1 else "2g"
            print(f"  {i}. {c:4s} ({f:6,}) [{char_type}]")

        if len(layer_chars) > 10:
            print(f"下位5個:")
            for i, (c, f) in enumerate(layer_chars[-5:], len(layer_chars)-4):
                char_type = "1g" if len(c) == 1 else "2g"
                print(f"  {i}. {c:4s} ({f:6,}) [{char_type}]")

    print()
    total_assigned = end

# 小書き文字（頻度0）の確認
small_kana = [(c, f) for c, f in all_chars if c in 'ぁぃぅぇぉ']
print(f"\n### 小書き文字（ぁぃぅぇぉ）: {len(small_kana)}個")
print("これらは頻度0なので最下層（Layer 4）に配置推奨")
for c, f in small_kana:
    print(f"  {c}: {f}")

print("\n\n=== 層別頻度割合 ===\n")

total_freq_all = sum(f for _, f in chars_with_freq)
print(f"総頻度: {total_freq_all:,}\n")

total_assigned = 0
for layer in range(5):
    capacity = layer_capacity[layer]
    start = total_assigned
    end = total_assigned + capacity

    layer_chars = chars_sorted[start:end]
    layer_total = sum(f for _, f in layer_chars)
    percentage = layer_total / total_freq_all * 100

    print(f"Layer {layer}: {layer_total:10,} ({percentage:5.2f}%) - コスト{layer_weights[layer]}")

    total_assigned = end

print("\n\n=== や行拗音（ゃゅょ終わり）の配置 ===\n")

ya_yoon_all = [(c, f) for c, f in chars_sorted if len(c) == 2 and c[1] in 'ゃゅょ']
print(f"や行拗音: {len(ya_yoon_all)}個\n")

# 頻度帯で分類
ultra_high = [(c, f) for c, f in ya_yoon_all if f >= 1000]
high = [(c, f) for c, f in ya_yoon_all if 300 <= f < 1000]
mid = [(c, f) for c, f in ya_yoon_all if 100 <= f < 300]
low = [(c, f) for c, f in ya_yoon_all if f < 100]

print(f"超高頻度（>=1000）: {len(ultra_high)}個 → Layer 1推奨")
for c, f in ultra_high:
    print(f"  {c}: {f:,}")

print(f"\n高頻度（300-999）: {len(high)}個 → Layer 1-2推奨")
for c, f in high:
    print(f"  {c}: {f:,}")

print(f"\n中頻度（100-299）: {len(mid)}個 → Layer 2-3推奨")
for c, f in mid:
    print(f"  {c}: {f:,}")

print(f"\n低頻度（<100）: {len(low)}個 → Layer 3-4推奨")
for c, f in low:
    print(f"  {c}: {f:,}")

print("\n\n=== ホームポジション配置の重要性 ===\n")
print("ホームポジション（中段）は最も打ちやすい位置")
print("各層で上位の文字をホームポジションに配置すべき")
print()
print("推奨配置優先度:")
print("  1. Layer 0 ホームポジション（最重要）")
print("  2. Layer 1,2 ホームポジション")
print("  3. Layer 0 上段/下段")
print("  4. Layer 3,4 ホームポジション")
print("  5. Layer 1,2 上段/下段")
print("  6. Layer 3,4 上段/下段（最低優先度）")
