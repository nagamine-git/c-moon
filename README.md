# 新月配列 (Shingetsu Layout)

**月配列2-263をベースに、Colemak風の音素配置を取り入れた日本語かな配列**

遺伝的アルゴリズム（GA）による最適化で、打鍵効率と学習しやすさを両立。

## 特徴

- **月配列2-263ベース**: 実績ある月配列の前置シフト方式を継承
- **Colemak音素配置**: 英語キーボード配列Colemakの設計思想を日本語に応用
- **3層構造**: 無シフト / ☆シフト / ★シフト の前置シフト方式
- **GA最適化**: 同指連続回避、ホームポジション重視、左右交互打鍵などを数値評価

## 配列構造

| レイヤー | 発動方法 | 用途 |
|---------|---------|------|
| Layer 0 | そのまま打鍵 | 高頻度文字（1打鍵） |
| Layer 1 | ☆キー → 文字 | 中頻度文字（2打鍵） |
| Layer 2 | ★キー → 文字 | 低頻度文字（2打鍵） |

## インストール

### ビルド済みバイナリ

[Releases](https://github.com/nagamine-git/shingetsu-layout/releases) からダウンロード

### ソースからビルド

```bash
git clone https://github.com/nagamine-git/shingetsu-layout.git
cd shingetsu-layout
cargo build --release
```

## 使い方

### 配列の生成（GA最適化）

```bash
# N-gramファイルを使用（推奨）
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  --gram3 3gram.txt --gram4 4gram.txt \
  -p 500 -g 1000

# TUIモード（リアルタイム可視化）
./target/release/kana_layout_optimizer \
  --gram1 1gram.txt --gram2 2gram.txt \
  --gram3 3gram.txt --gram4 4gram.txt \
  -p 500 -g 500 --tui
```

### 出力形式

最適化完了時に以下の形式で自動エクスポート:

| ファイル | 用途 |
|---------|------|
| `***.json` | 配列データ（スコア含む） |
| `***_analyzer.json` | [keyboard_analyzer](https://github.com/eswai/keyboard_analyzer) 用 |
| `***-ansi.tsv` | hazkey用ローマ字テーブル（QWERTY） |
| `***-ansi-colemak.tsv` | hazkey用ローマ字テーブル（Colemak） |
| `***-karabiner.json` | Karabiner Elements用 |

## 評価メトリクス

### Core Metrics（必須・乗算評価）

| メトリクス | 説明 |
|-----------|------|
| 同指連続回避 | 同じ指での連続打鍵を最小化 |
| 段飛ばし回避 | 同指での段越えを最小化 |
| ホームポジション率 | 中段キーの使用率 |
| 位置別コスト | 高頻度文字を打ちやすい位置に配置 |
| 左右交互打鍵 | 左右の手の切り替え頻度 |
| 単打鍵率 | シフト不要な打鍵の割合 |
| Colemak類似度 | 音素配置のColemak一致度 |

### Bonus Metrics（奨励・加算評価）

| メトリクス | 説明 |
|-----------|------|
| 月配列類似度 | 月配列2-263との位置一致度 |
| ロール率 | 同手での滑らかな指の流れ |
| インロール | 外→内への指の動き |
| アルペジオ | 隣接指での連続打鍵 |
| リダイレクト回避 | 3連打での方向転換を回避 |

## オプション一覧

### 基本オプション

| オプション | デフォルト | 説明 |
|------------|-----------|------|
| `--gram1` | - | 1-gramファイル |
| `--gram2` | - | 2-gramファイル |
| `--gram3` | - | 3-gramファイル |
| `--gram4` | - | 4-gramファイル |
| `-c, --corpus` | `corpus.txt` | コーパスファイル |
| `-p, --population` | 500 | GA集団サイズ |
| `-g, --generations` | 1000 | GA世代数 |
| `-m, --mutation-rate` | 0.15 | 突然変異率 |
| `-s, --seed` | 42 | 乱数シード |
| `--tui` | false | TUIモード |
| `-o, --output` | `best_layout.json` | 出力ファイル |

### 評価重みオプション

Core Metrics:
- `--w-same-finger`, `--w-row-skip`, `--w-home-position`
- `--w-total-keystrokes`, `--w-alternating`, `--w-single-key`
- `--w-colemak-similarity`, `--w-position-cost`

Bonus Metrics:
- `--w-redirect-low`, `--w-tsuki-similarity`, `--w-roll`
- `--w-inroll`, `--w-arpeggio`, `--w-memorability`, `--w-shift-balance`

## 関連プロジェクト

- [月配列](https://jisx6004.client.jp/tsuki.html) - 本配列のベースとなった前置シフト方式のかな配列
- [Colemak](https://colemak.com/) - 音素配置の参考にした英語キーボード配列
- [keyboard_analyzer](https://github.com/eswai/keyboard_analyzer) - かな配列の評価・可視化ツール

## ライセンス

MIT

## Keywords

日本語入力, かな配列, キーボードレイアウト, 月配列, 月配列2-263, Colemak, 前置シフト, 遺伝的アルゴリズム, GA, Rust, Shingetsu, Japanese Input, Kana Layout, Keyboard Layout
