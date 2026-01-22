# Repository Guidelines

## Project Overview
Rust製の日本語かな配列遺伝的アルゴリズム（GA）最適化ツール。3層構造（無シフト/☆シフト/★シフト）のかな配列を、複数の評価メトリクスに基づいて最適化する。

## Project Structure
```
src/
├── main.rs        # CLIエントリポイント、引数解析
├── layout.rs      # 配列データ構造（Layout, KeyPos, EvaluationScores）
├── corpus.rs      # N-gram読み込み、コーパス統計
├── evaluation.rs  # 評価メトリクス計算、フィットネス関数
├── ga.rs          # 遺伝的アルゴリズム（選択・交叉・突然変異）
└── tui.rs         # TUI可視化（ratatui）
```

## Build & Run Commands
```bash
# ビルド
cargo build --release

# 基本実行
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  --gram3 3gram.txt --gram4 4gram.txt \
  -p 500 -g 1000

# TUIモード
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  -p 500 -g 500 --tui

# マルチラン並列実行
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  -p 200 -g 200 --multi-run 32

# テスト
cargo test
```

## Coding Style & Conventions
- Rust 2021 Edition
- モジュール分離: データ構造 / コーパス / 評価 / GA / TUI
- 命名: snake_case（関数・変数）、CamelCase（型）
- ドキュメントコメント: `///` で公開API、`//!` でモジュール説明
- エラー処理: `Result<T, E>` を使用、`unwrap()` は避ける

## Key Data Structures

### Layout（配列）
```rust
pub struct Layout {
    pub layers: [[[char; 10]; 3]; 3],  // [layer][row][col]
    pub fitness: f64,
    pub scores: EvaluationScores,
}
```

### CorpusStats（コーパス統計）
```rust
pub struct CorpusStats {
    pub char_freq: HashMap<char, usize>,           // 1-gram
    pub bigram_freq: HashMap<(char, char), usize>, // 2-gram
    pub trigram_freq: HashMap<(char, char, char), usize>,
    pub fourgram_freq: HashMap<(char, char, char, char), usize>,
    pub hiragana_by_freq: Vec<char>,  // 頻度順ひらがな（1gramから生成）
}
```

### EvaluationWeights（評価重み）
- **Core Metrics（乗算・指数）**: same_finger, row_skip, home_position, total_keystrokes, alternating, single_key, colemak_similarity
- **Bonus Metrics（加算）**: redirect_low, tsuki_similarity, roll, inroll, arpeggio, memorability, shift_balance

## Evaluation Logic
フィットネス計算式:
```
fitness = core_multiplier × (1 + additive_bonus / bonus_scale)
```

### Core Multiplier
各メトリクスの重み付き幾何平均:
```
core = (metric1^w1 × metric2^w2 × ...)^(1/sum_weights) × 100
```

### Additive Bonus
各メトリクスの重み付き和:
```
bonus = Σ(metric_i × weight_i)
```

## Important Notes

### 前置シフトの特性
- Layer 1/2はどちらも2打鍵（シフト→文字）
- `shift_balance` メトリクスで☆/★の使用頻度を均等化
- 100% = 完全均等（50:50）、0% = 片方のみ使用

### Colemak類似度
- Layer 0: 100%の重み
- Layer 1/2: 80%の重み
- 「ん」と"n"は完全一致として計算

### 月配列類似度
- GA Layer 0 → 月 表面（layer 0）と比較
- GA Layer 1/2 → 月 裏面（layer 1）と比較

### ひらがな頻度順
- 1gramファイルから正確に読み込み（ハードコードではない）
- `hiragana_by_freq` に頻度降順で格納
- 句読点（、。）は除外

## Commit Guidelines
Conventional Commits形式:
- `feat:` 新機能
- `fix:` バグ修正
- `refactor:` リファクタリング
- `docs:` ドキュメント
- `perf:` パフォーマンス改善

## Dependencies
- `clap`: CLI引数解析
- `serde`, `serde_json`: シリアライゼーション
- `rand`, `rand_chacha`: 乱数生成
- `rayon`: 並列処理
- `indicatif`: プログレスバー
- `ratatui`, `crossterm`: TUI
- `atty`: TTY検出
- `num_cpus`: CPUコア数検出
