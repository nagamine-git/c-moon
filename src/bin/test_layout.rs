use kana_layout_optimizer::layout::Layout;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn main() {
    println!("=== 新月配列 v2.0 テスト ===\n");

    // シード固定でランダム配列を生成
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let layout = Layout::random(&mut rng);

    println!("生成された配列（5層）:\n");

    for layer in 0..5 {
        let layer_name = match layer {
            0 => "Layer 0 (無シフト)",
            1 => "Layer 1 (☆シフト・右中指)",
            2 => "Layer 2 (★シフト・左中指)",
            3 => "Layer 3 (◎シフト・右薬指)",
            4 => "Layer 4 (◆シフト・左薬指)",
            _ => "Unknown",
        };

        println!("## {}", layer_name);
        println!("```");
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
        println!("```\n");
    }

    // 統計情報
    let mut gram1_count = 0;
    let mut gram2_count = 0;

    for layer in 0..5 {
        for row in 0..3 {
            for col in 0..10 {
                let c = &layout.layers[layer][row][col];
                if c != "　" && c != "★" && c != "☆" && c != "◎" && c != "◆" && c != "、" && c != "。" && c != "；" && c != "・" {
                    if c.chars().count() == 1 {
                        gram1_count += 1;
                    } else if c.chars().count() == 2 {
                        gram2_count += 1;
                    }
                }
            }
        }
    }

    println!("統計:");
    println!("  1gram文字: {}", gram1_count);
    println!("  2gram文字（拗音）: {}", gram2_count);
    println!("  合計: {}", gram1_count + gram2_count);
}
