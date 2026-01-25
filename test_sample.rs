// 簡易テスト: 配列サンプル表示
// コンパイル: rustc --edition 2021 test_sample.rs
// または: cargo script test_sample.rs

use std::collections::VecDeque;

const NUM_LAYERS: usize = 5;
const ROWS: usize = 3;
const COLS: usize = 10;

const HIRAGANA_FREQ_DEFAULT: &[&str] = &[
    "い", "う", "ん", "し", "か", "の", "と", "た", "て", "く",
    "な", "に", "き", "は", "こ", "る", "が", "で", "っ", "す",
    "ま", "じ", "り", "も", "つ", "お", "ら", "を", "さ", "あ",
    "れ", "だ", "ち", "せ", "け", "ー", "よ", "ど", "そ", "え",
    "わ", "み", "め", "ひ", "ば", "や", "ろ", "ほ", "しょ", "ふ",
    "ぶ", "ね", "ご", "ぎ", "じょ", "げ", "しゅ", "む", "きょ", "ず",
    "び", "しゃ", "ちょ", "ざ", "ぐ", "ぜ", "へ", "べ", "ゆ", "じゅ",
    "ぼ", "ぷ", "りょ", "ぞ", "ぱ", "きゅ", "ちゅ", "ぎょ", "ぽ", "にゅ",
    "ひょ", "づ", "じゃ", "ちゃ", "ぴ", "ぬ", "てぃ", "りゅ", "ぺ", "きゃ",
    "ふぁ", "でぃ", "しぇ", "びょ", "りゃ", "ふぃ", "ちぇ", "ぎゃ", "うぇ", "なぁ",
    "ふぇ", "ぴょ", "ぴゅ", "じぇ", "ふぉ", "ヴ", "びゅ", "ぢ", "みょ", "ひゃ",
    "みゅ", "ぎゅ", "みぃ", "ヴぁ", "うぃ", "にょ", "ねぇ", "まぁ", "ねぃ", "〓ぉ",
    "でゅ", "みゃ", "うぉ", "かぁ", "にゃ", "とぅ", "くぉ", "ひゅ", "はぁ", "へぇ",
    "りぃ", "ぎぃ", "だぁ", "おぉ", "しぃ", "どぅ", "ヴぃ", "てぇ", "ヴぇ", "あぁ",
];

fn main() {
    println!("=== 新月配列 v2.0 サンプル配列 ===\n");
    println!("140文字（1gram 74 + 拗音2gram 66）を5層に配置\n");

    // 簡易配列生成（固定位置を考慮）
    let mut layers: Vec<Vec<Vec<String>>> = (0..NUM_LAYERS)
        .map(|_| {
            (0..ROWS)
                .map(|_| vec!["　".to_string(); COLS])
                .collect()
        })
        .collect();

    // Layer 0: シフトキーと句読点
    layers[0][1][1] = "◆".to_string();  // 左薬指シフト
    layers[0][1][2] = "★".to_string();  // 左中指シフト
    layers[0][1][7] = "☆".to_string();  // 右中指シフト
    layers[0][1][8] = "◎".to_string();  // 右薬指シフト
    layers[0][2][7] = "、".to_string();
    layers[0][2][8] = "。".to_string();

    // Layer 1: 記号
    layers[1][2][7] = "；".to_string();
    layers[1][2][8] = "・".to_string();

    // Layer 2: Ver空白
    layers[2][0][2] = "　".to_string();
    layers[2][2][2] = "　".to_string();

    // 文字を順番に配置（簡易版）
    let mut chars: VecDeque<String> = HIRAGANA_FREQ_DEFAULT.iter().take(140).map(|s| s.to_string()).collect();

    for layer in 0..NUM_LAYERS {
        for row in 0..ROWS {
            for col in 0..COLS {
                // 固定位置をスキップ
                let is_fixed = (layer == 0 && row == 1 && (col == 1 || col == 2 || col == 7 || col == 8))
                    || (layer == 0 && row == 2 && (col == 7 || col == 8))
                    || (layer == 1 && row == 2 && (col == 7 || col == 8))
                    || (layer == 2 && col == 2 && (row == 0 || row == 2));

                if !is_fixed && !chars.is_empty() {
                    layers[layer][row][col] = chars.pop_front().unwrap();
                }
            }
        }
    }

    // 表示
    for layer in 0..NUM_LAYERS {
        let layer_name = match layer {
            0 => "Layer 0 (無シフト)",
            1 => "Layer 1 (☆シフト・右中指k)",
            2 => "Layer 2 (★シフト・左中指d)",
            3 => "Layer 3 (◎シフト・右薬指l)",
            4 => "Layer 4 (◆シフト・左薬指s)",
            _ => "Unknown",
        };

        println!("## {}", layer_name);
        println!("```");
        for row in 0..ROWS {
            print!("  ");
            for col in 0..COLS {
                let c = &layers[layer][row][col];
                // 2文字の拗音は少し広めに表示
                if c.chars().count() >= 2 {
                    print!("{:4}", c);
                } else {
                    print!("{:3}", c);
                }
            }
            println!();
        }
        println!("```\n");
    }

    // 統計
    let mut gram1 = 0;
    let mut gram2 = 0;
    for layer in 0..NUM_LAYERS {
        for row in 0..ROWS {
            for col in 0..COLS {
                let c = &layers[layer][row][col];
                if c != "　" && c != "★" && c != "☆" && c != "◎" && c != "◆" && c != "、" && c != "。" && c != "；" && c != "・" {
                    if c.chars().count() == 1 {
                        gram1 += 1;
                    } else if c.chars().count() == 2 {
                        gram2 += 1;
                    }
                }
            }
        }
    }

    println!("### 統計");
    println!("- 1gram文字: {}", gram1);
    println!("- 2gram文字（拗音）: {}", gram2);
    println!("- 合計: {}", gram1 + gram2);
}
