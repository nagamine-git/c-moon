# かな配列遺伝的アルゴリズム最適化ツール

日本語かな配列を遺伝的アルゴリズム（GA）で最適化するRustツール。

## ビルド

```bash
cargo build --release
```

## 実行方法

### 基本実行

```bash
# N-gramファイルを使用（推奨）
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  --gram3 3gram.txt --gram4 4gram.txt \
  -p 500 -g 1000

# コーパステキストを使用
./target/release/kana_layout_optimizer -c corpus.txt -p 500 -g 1000
```

### マルチラン実行

```bash
# CPUコア数を最大活用して並列実行
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  --gram3 3gram.txt --gram4 4gram.txt \
  -p 500 -g 1000 \
  --multi-run 16
```

## オプション

| オプション | デフォルト | 説明 |
|------------|-----------|------|
| `--gram1` | - | 1-gramファイル |
| `--gram2` | - | 2-gramファイル |
| `--gram3` | - | 3-gramファイル |
| `--gram4` | - | 4-gramファイル |
| `-c, --corpus` | `corpus.txt` | コーパスファイル（N-gram未指定時） |
| `-p, --population` | 500 | 集団サイズ |
| `-g, --generations` | 1000 | 世代数 |
| `-m, --mutation-rate` | 0.15 | 突然変異率 |
| `-e, --elite` | 10 | エリート保持数 |
| `-s, --seed` | 42 | 乱数シード |
| `--multi-run` | 0 | 並列実行数（0=単一実行） |
| `-o, --output` | `best_layout.json` | 出力ファイル |

### 評価重みオプション

**Core Metrics（乗算・指数）:**
- `--w-same-finger` (1.8): 同指連続率の低さ
- `--w-row-skip` (1.55): 段越えの少なさ
- `--w-home-position` (1.3): ホームポジション率
- `--w-total-keystrokes` (1.05): 総打鍵コスト
- `--w-alternating` (0.8): 左右交互打鍵率
- `--w-single-key` (0.7): 単打鍵率
- `--w-colemak-similarity` (0.6): Colemak類似度

**Bonus Metrics（加算）:**
- `--w-redirect-low` (5.0): リダイレクト少
- `--w-tsuki-similarity` (4.0): 月配列類似度
- `--w-roll` (5.0): ロール率
- `--w-inroll` (5.0): インロール率
- `--w-arpeggio` (5.0): アルペジオ率
- `--w-memorability` (2.0): 覚えやすさ
- `--w-shift-balance` (3.0): シフトバランス（☆★均等化）

## 配列構造

3層構造のかな配列（前置シフト対応）：

- **Layer 0（無シフト）**: 高頻度文字（1打鍵）
- **Layer 1（☆シフト）**: シフト+文字（2打鍵・dキー前置）
- **Layer 2（★シフト）**: シフト+文字（2打鍵・kキー前置）

## 評価メトリクス

### Core Metrics（乗算・必須）

低スコアは致命的。全メトリクスの重み付き幾何平均を計算。

| メトリクス | 説明 |
|-----------|------|
| 同指連続低 | 同じ指での連続打鍵を回避 |
| 段飛ばし少 | 同指での段越えを回避 |
| ホームポジ率 | 中段の使用率 |
| 総打鍵コスト | 指の移動距離負担 |
| 左右交互 | 左右の手の切り替え率 |
| 単打鍵率 | シフト不要な打鍵率 |
| Colemak類似 | 音素配置のColemak一致度 |

### Bonus Metrics（加算・奨励）

高スコアで加点。低くてもペナルティは小さい。

| メトリクス | 説明 |
|-----------|------|
| リダイレクト少 | 3-gramでの方向転換回避 |
| 月配列類似 | 月配列との位置一致度 |
| ロール率 | 同手での滑らかな指の流れ |
| インロール | 外→内への指の動き |
| アルペジオ | 隣接指での連続打鍵 |
| 覚えやすさ | レイヤー間の母音/子音一貫性 |
| シフトバランス | ☆/★シフトの使用頻度均等化 |

## プロジェクト構造

```
src/
├── main.rs        # CLIエントリポイント
├── layout.rs      # 配列データ構造
├── corpus.rs      # N-gram読み込み
├── evaluation.rs  # 評価スコア計算
└── ga.rs          # 遺伝的アルゴリズム
```

## ライセンス

MIT
