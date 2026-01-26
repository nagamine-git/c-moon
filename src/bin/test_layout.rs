use kana_layout_optimizer::layout::{Layout, HIRAGANA_FREQ_DEFAULT};

fn main() {
    println!("=== 新月配列 v2.0 テスト ===\n");

    // 初期配列を表示
    let layout = Layout::improved_custom();

    println!("初期配列（5層）:\n");

    for layer in 0..5 {
        let layer_name = match layer {
            0 => "Layer 0 (No Shift)",
            1 => "Layer 1 (A shift)",
            2 => "Layer 2 (B shift)",
            3 => "Layer 3 (C shift)",
            4 => "Layer 4 (D shift)",
            _ => "Unknown",
        };

        println!("## {}", layer_name);
        for row in 0..3 {
            print!("  ");
            for col in 0..10 {
                let c = &layout.layers[layer][row][col];
                // 1文字なら1文字分、2文字なら2文字分表示
                if c.chars().count() == 1 {
                    print!("{:3}", c);
                } else {
                    print!("{:4}", c);
                }
            }
            println!();
        }
        println!();
    }

    // 統計情報
    let mut gram1_count = 0;
    let mut gram2_count = 0;
    let mut blank_count = 0;
    let mut fixed_count = 0;

    let fixed_chars = ["☆", "★", "◎", "◆", "、", "。", "・", "ー", ";"];

    for layer in 0..5 {
        for row in 0..3 {
            for col in 0..10 {
                let c = &layout.layers[layer][row][col];
                if c == "　" {
                    blank_count += 1;
                } else if fixed_chars.contains(&c.as_str()) {
                    fixed_count += 1;
                } else if c.chars().count() == 1 {
                    gram1_count += 1;
                } else if c.chars().count() == 2 {
                    gram2_count += 1;
                }
            }
        }
    }

    println!("=== 統計 ===");
    println!("  1gram文字: {}", gram1_count);
    println!("  2gram文字（拗音）: {}", gram2_count);
    println!("  配置文字合計: {}", gram1_count + gram2_count);
    println!("  空白: {}", blank_count);
    println!("  固定文字: {} (◆,★,☆,◎,、,。,・,ー,;)", fixed_count);
    println!("  総計: {} (=150)", gram1_count + gram2_count + blank_count + fixed_count);

    println!("\n=== HIRAGANA_FREQ_DEFAULT ===");
    println!("  文字数: {}", HIRAGANA_FREQ_DEFAULT.len());

    // 検証
    let result = layout.validate(HIRAGANA_FREQ_DEFAULT);
    result.print_report();
}
