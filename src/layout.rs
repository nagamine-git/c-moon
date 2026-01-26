//! 配列データ構造モジュール
//! 
//! キーボード配列を表現するためのデータ構造を提供する。

use rand::seq::SliceRandom;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// 定数
// ============================================================================

/// キーボードの行数
pub const ROWS: usize = 3;

/// キーボードの列数
pub const COLS: usize = 10;

/// レイヤー数（無シフト、Aシフト、Bシフト、Cシフト、Dシフト）
pub const NUM_LAYERS: usize = 5;

/// 1レイヤーあたりのキー数
pub const KEYS_PER_LAYER: usize = ROWS * COLS;

/// キー配置ペナルティ（位置コスト）
/// 0.0 は固定位置または空白位置
pub const POSITION_COSTS: [[[f64; COLS]; ROWS]; NUM_LAYERS] = [
    // Layer 0 (No Shift)
    [
        [3.7, 2.0, 2.0, 2.4, 3.5, 3.9, 2.4, 2.0, 2.0, 3.7],  // row 0 (全て配置可能)
        [1.5, 0.0, 0.0, 1.0, 2.4, 2.4, 1.0, 0.0, 0.0, 1.5],  // row 1 (cols 1,2,7,8 = ◆,★,☆,◎)
        [3.7, 2.8, 2.4, 2.0, 3.9, 3.0, 2.0, 0.0, 0.0, 3.7],  // row 2 (cols 7,8 = 、,。)
    ],
    // Layer 1 (A shift) - ー,・ は row2 cols 7,8
    [
        [6.0, 3.2, 3.2, 3.9, 5.5, 6.2, 3.9, 0.0, 0.0, 0.0],  // row 0 (cols 7-9 blank)
        [2.4, 1.6, 1.6, 1.6, 3.9, 3.9, 1.6, 2.9, 0.0, 0.0],  // row 1 (cols 8-9 blank)
        [6.0, 4.5, 3.9, 3.2, 6.2, 4.8, 3.2, 0.0, 0.0, 0.0],  // row 2 (cols 7,8 = ー,・, col 9 blank)
    ],
    // Layer 2 (B shift)
    [
        [0.0, 0.0, 0.0, 3.9, 5.5, 6.2, 3.9, 3.2, 3.2, 0.0],  // row 0 (cols 0-2,9 blank)
        [0.0, 0.0, 2.9, 1.6, 3.9, 3.9, 1.6, 1.6, 1.6, 2.4],  // row 1 (cols 0-1 blank)
        [0.0, 0.0, 0.0, 3.2, 6.2, 4.8, 3.2, 3.9, 4.5, 0.0],  // row 2 (cols 0-2,9 blank)
    ],
    // Layer 3 (C shift) - ; は row2 col8
    [
        [6.4, 3.4, 3.4, 4.2, 5.9, 6.6, 4.2, 3.4, 0.0, 0.0],  // row 0 (cols 8-9 blank)
        [2.6, 1.7, 1.7, 1.7, 4.2, 4.2, 1.7, 1.7, 2.9, 0.0],  // row 1 (col 9 blank)
        [6.4, 4.8, 4.2, 3.4, 6.6, 5.1, 3.4, 4.2, 0.0, 0.0],  // row 2 (col 8 = ;, col 9 blank)
    ],
    // Layer 4 (D shift)
    [
        [0.0, 0.0, 3.4, 4.2, 5.9, 6.6, 4.2, 3.4, 3.4, 0.0],  // row 0 (cols 0,1,9 blank)
        [0.0, 2.9, 1.7, 1.7, 4.2, 4.2, 1.7, 1.7, 1.7, 2.6],  // row 1 (col 0 blank)
        [0.0, 0.0, 4.2, 3.4, 6.6, 5.1, 3.4, 4.2, 4.8, 0.0],  // row 2 (cols 0,1,9 blank)
    ],
];

/// ひらがな文字のデフォルト頻度順リスト（フォールバック用）
/// 114文字: 1gram(73) + 小書き(7:ぁぃぅぇぉゃょ) + ゃゅょ終わり2gram(34)
/// ぁぃぅぇぉで終わる2gramは除外（うぃ、ふぁ等は小書き1gramで組み合わせ可能）
/// 固定文字(9個): 、, 。, ・, ー, ;, ◆, ★, ☆, ◎
pub const HIRAGANA_FREQ_DEFAULT: &[&str] = &[
    // 1gram (73文字)
    "い", "う", "ん", "し", "か", "の", "と", "た", "て", "く",
    "な", "に", "き", "は", "こ", "る", "が", "で", "っ", "す",
    "ま", "じ", "り", "も", "つ", "お", "ら", "を", "さ", "あ",
    "れ", "だ", "ち", "せ", "け", "よ", "ど", "そ", "え", "わ",
    "み", "め", "ひ", "ば", "や", "ろ", "ほ", "ふ", "ぶ", "ね",
    "ご", "ぎ", "げ", "む", "ず", "び", "ざ", "ぐ", "ぜ", "へ",
    "べ", "ゆ", "ぼ", "ぷ", "ぞ", "ぱ", "ぽ", "づ", "ぴ", "ぬ",
    "ぺ", "ヴ", "ぢ",
    // 小書き (7文字)
    "ぁ", "ぃ", "ぅ", "ぇ", "ぉ", "ゃ", "ょ",
    // ゃゅょ終わり2gram (34文字) - ぁぃぅぇぉ終わりは除外
    "しょ", "じょ", "しゅ", "きょ", "しゃ", "ちょ", "じゅ", "りょ",
    "きゅ", "ちゅ", "ぎょ", "にゅ", "ひょ", "じゃ", "ちゃ", "りゅ",
    "きゃ", "びょ", "りゃ", "ぎゃ", "ぴょ", "ぴゅ", "びゅ", "みょ",
    "ひゃ", "みゅ", "にょ", "みゃ", "にゃ", "ひゅ", "びゃ", "ぴゃ",
    "ぎゅ", "ぢゃ",
];

// ============================================================================
// キー位置
// ============================================================================

/// キーの位置を表す構造体
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyPos {
    /// レイヤー（0: 無シフト, 1: ☆シフト, 2: ★シフト）
    pub layer: usize,
    /// 行（0: 上段, 1: 中段, 2: 下段）
    pub row: usize,
    /// 列（0-9）
    pub col: usize,
}

impl KeyPos {
    /// 新しいキー位置を作成
    pub fn new(layer: usize, row: usize, col: usize) -> Self {
        Self { layer, row, col }
    }

    /// ホームポジションかどうか（中段）
    pub fn is_home(&self) -> bool {
        self.row == 1
    }

    /// 左手かどうか（列0-4）
    pub fn is_left_hand(&self) -> bool {
        self.col < 5
    }

    /// 指のインデックス（0: 小指, 1: 薬指, 2: 中指, 3: 人差し指）
    pub fn finger(&self) -> usize {
        match self.col {
            0 | 9 => 0, // 小指
            1 | 8 => 1, // 薬指
            2 | 7 => 2, // 中指
            3 | 4 | 5 | 6 => 3, // 人差し指
            _ => 0,
        }
    }

    /// キーの打鍵コスト（距離ベース）
    pub fn weight(&self) -> f64 {
        // 基本重み: ホームポジション = 1.0
        let row_weight = match self.row {
            1 => 1.0,  // ホーム
            0 => 1.3,  // 上段
            2 => 1.2,  // 下段
            _ => 2.0,
        };

        // 列の重み（中央が低い）
        let col_weight = match self.col {
            3 | 4 | 5 | 6 => 1.0,  // 人差し指
            2 | 7 => 1.1,          // 中指
            1 | 8 => 1.2,          // 薬指
            0 | 9 => 1.4,          // 小指
            _ => 1.5,
        };

        // シフトレイヤーのペナルティ
        let layer_weight = match self.layer {
            0 => 1.0,       // 無シフト
            1 => 2.0,       // 中指シフト（☆）
            2 => 2.2,       // 中指シフト（★）
            3 => 2.2,       // 薬指シフト（◎）
            4 => 2.3,       // 薬指シフト（◆）
            _ => 3.0,
        };

        row_weight * col_weight * layer_weight
    }
}

// ============================================================================
// 配列
// ============================================================================

/// キーボード配列を表す構造体
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layout {
    /// 5層の配列データ [layer][row][col] - String型（1文字 or 2文字の拗音対応）
    pub layers: Vec<Vec<Vec<String>>>,

    /// 評価フィットネス値
    #[serde(default)]
    pub fitness: f64,

    /// 詳細スコア
    #[serde(default)]
    pub scores: EvaluationScores,
}

/// 評価スコアの詳細
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EvaluationScores {
    /// 段飛ばしの少なさ（高いほど良い）
    pub row_skip: f64,
    /// ホームポジション率
    pub home_position: f64,
    /// 総打鍵コストの低さ
    pub total_keystrokes: f64,
    /// 同指連続の少なさ
    pub same_finger: f64,
    /// 単打鍵率（シフト無し）
    pub single_key: f64,
    /// Colemak類似度
    pub colemak_similarity: f64,
    /// 位置別コスト（ベースコスト×シフト係数）
    pub position_cost: f64,
    /// 月配列類似度
    pub tsuki_similarity: f64,
    /// 覚えやすさ
    pub memorability: f64,
    /// 左右交互打鍵率
    pub alternating: f64,
    /// ロール率
    pub roll: f64,
    /// リダイレクト少なさ
    pub redirect_low: f64,
    /// インロール率
    pub inroll: f64,
    /// アルペジオ率
    pub arpeggio: f64,
    /// シフトバランス（☆★均等）
    pub shift_balance: f64,
}

impl Default for Layout {
    fn default() -> Self {
        let layers = (0..NUM_LAYERS)
            .map(|_| {
                (0..ROWS)
                    .map(|_| {
                        (0..COLS)
                            .map(|_| "　".to_string())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self {
            layers,
            fitness: 0.0,
            scores: EvaluationScores::default(),
        }
    }
}

impl Layout {
    /// 改善版カスタムレイアウト（初期配置として使用）
    /// シフトキー: ◆(col1) → Layer4, ★(col2) → Layer2, ☆(col7) → Layer1, ◎(col8) → Layer3
    /// 固定文字(9個): 、, 。, ・, ー, ;, ◆, ★, ☆, ◎
    pub fn improved_custom() -> Self {
        let mut layers: Vec<Vec<Vec<String>>> = (0..NUM_LAYERS)
            .map(|_| {
                (0..ROWS)
                    .map(|_| vec!["　".to_string(); COLS])
                    .collect()
            })
            .collect();

        // Layer 0 (No Shift)
        layers[0][0] = vec!["ぐ", "も", "ら", "れ", "ちょ", "ゆ", "だ", "り", "を", "ぜ"].iter().map(|s| s.to_string()).collect();
        layers[0][1] = vec!["し", "◆", "★", "う", "ち", "け", "い", "☆", "◎", "ん"].iter().map(|s| s.to_string()).collect();
        layers[0][2] = vec!["ざ", "そ", "せ", "お", "べ", "ひ", "つ", "、", "。", "へ"].iter().map(|s| s.to_string()).collect();

        // Layer 1 (☆シフト) - col=7右中指前置で発動
        // 右側(cols 7,8,9)は row0全て、row1-2は8,9が空白
        layers[1][0] = vec!["ぎゅ", "ろ", "ぶ", "ぼ", "びゅ", "にゃ", "ぷ", "　", "　", "　"].iter().map(|s| s.to_string()).collect();
        layers[1][1] = vec!["あ", "な", "の", "と", "ぱ", "ぎょ", "く", "え", "　", "　"].iter().map(|s| s.to_string()).collect();
        layers[1][2] = vec!["みゅ", "きゃ", "きゅ", "や", "にょ", "りゃ", "しょ", "ー", "・", "　"].iter().map(|s| s.to_string()).collect();

        // Layer 2 (★シフト) - col=2左中指前置で発動
        // 左側(cols 0,1,2,9)は row0で空白、row1は0,1、row2は0,1,2,9が空白
        layers[2][0] = vec!["　", "　", "　", "じゅ", "ぢ", "みゃ", "りょ", "ば", "ね", "　"].iter().map(|s| s.to_string()).collect();
        layers[2][1] = vec!["　", "　", "わ", "た", "ぞ", "ぃ", "て", "か", "に", "さ"].iter().map(|s| s.to_string()).collect();
        layers[2][2] = vec!["　", "　", "　", "ほ", "ぅ", "ぉ", "ふ", "ちゅ", "びょ", "　"].iter().map(|s| s.to_string()).collect();

        // Layer 3 (◎シフト) - col=8右薬指前置で発動
        // 右側(cols 8,9)は row0,2で空白、row1はcol9のみ空白
        layers[3][0] = vec!["びゃ", "じょ", "び", "にゅ", "みょ", "ゃ", "ひょ", "ぎ", "　", "　"].iter().map(|s| s.to_string()).collect();
        layers[3][1] = vec!["ど", "ま", "る", "っ", "づ", "りゅ", "す", "き", "め", "　"].iter().map(|s| s.to_string()).collect();
        layers[3][2] = vec!["ひゅ", "ぎゃ", "ちゃ", "しゅ", "ぴゃ", "ヴ", "きょ", "ぬ", ";", "　"].iter().map(|s| s.to_string()).collect();

        // Layer 4 (◆シフト) - col=1左薬指前置で発動
        // 左側(cols 0,1)と右端(col9)がrow0,2で空白、row1はcol0のみ空白
        layers[4][0] = vec!["　", "　", "ず", "ぽ", "ひゃ", "ょ", "ぇ", "ご", "しゃ", "　"].iter().map(|s| s.to_string()).collect();
        layers[4][1] = vec!["　", "み", "こ", "が", "じゃ", "ぺ", "で", "は", "じ", "よ"].iter().map(|s| s.to_string()).collect();
        layers[4][2] = vec!["　", "　", "ぴ", "げ", "ぢゃ", "ぴゅ", "む", "ぁ", "ぴょ", "　"].iter().map(|s| s.to_string()).collect();

        Self {
            layers,
            fitness: 0.0,
            scores: EvaluationScores::default(),
        }
    }
    
    /// ランダムな配列を生成（デフォルト頻度リスト使用）
    pub fn random(rng: &mut ChaCha8Rng) -> Self {
        Self::random_with_chars(rng, HIRAGANA_FREQ_DEFAULT)
    }
    
    /// 指定した文字リストからランダムな配列を生成
    /// コーパスの1gramから取得した頻度順リストを使用可能（1文字 or 2文字の拗音対応）
    pub fn random_with_chars(rng: &mut ChaCha8Rng, hiragana_chars: &[&str]) -> Self {
        let mut chars: Vec<String> = hiragana_chars.iter().map(|s| s.to_string()).collect();

        // 空白位置を事前に計算（シフトキーと同手で押せない位置）
        // Layer 1 (☆シフト, col=7右中指): 右側制限
        //   - Row 0: cols 7,8,9 = 3 blanks
        //   - Row 1: cols 8,9 = 2 blanks
        //   - Row 2: col 9 = 1 blank (cols 7,8は ー,・ 固定)
        // Layer 2 (★シフト, col=2左中指): 左側制限
        //   - Row 0: cols 0,1,2,9 = 4 blanks
        //   - Row 1: cols 0,1 = 2 blanks
        //   - Row 2: cols 0,1,2,9 = 4 blanks
        // Layer 3 (◎シフト, col=8右薬指): 右側制限
        //   - Row 0: cols 8,9 = 2 blanks
        //   - Row 1: col 9 = 1 blank
        //   - Row 2: col 9 = 1 blank (col 8は ; 固定)
        // Layer 4 (◆シフト, col=1左薬指): 左側+右端制限
        //   - Row 0: cols 0,1,9 = 3 blanks
        //   - Row 1: col 0 = 1 blank
        //   - Row 2: cols 0,1,9 = 3 blanks
        //
        // 固定位置 (9):
        //   Layer 0: ◆,★,☆,◎ (4) + 、,。 (2) = 6
        //   Layer 1: ー,・ (2) = 2
        //   Layer 3: ; (1) = 1
        // シフト制限空白 (27):
        //   Layer 1: 6, Layer 2: 10, Layer 3: 4, Layer 4: 7
        // 配置可能: 150 - 9 - 27 = 114ポジション
        const FIXED_COUNT: usize = 9;
        const SHIFT_BLANK_COUNT: usize = 27;
        let total_positions = KEYS_PER_LAYER * NUM_LAYERS - FIXED_COUNT - SHIFT_BLANK_COUNT;

        // 119個分の文字を用意（足りなければ空白で埋める）
        while chars.len() < total_positions {
            chars.push("　".to_string());
        }
        // 多すぎる場合は切り詰め
        chars.truncate(total_positions);

        chars.shuffle(rng);

        let mut layers: Vec<Vec<Vec<String>>> = (0..NUM_LAYERS)
            .map(|_| {
                (0..ROWS)
                    .map(|_| vec!["　".to_string(); COLS])
                    .collect()
            })
            .collect();

        // 固定文字の配置
        // Layer 0: シフトキーと句読点
        layers[0][1][1] = "◆".to_string();  // Layer 4
        layers[0][1][2] = "★".to_string();  // Layer 2
        layers[0][1][7] = "☆".to_string();  // Layer 1
        layers[0][1][8] = "◎".to_string();  // Layer 3
        layers[0][2][7] = "、".to_string();
        layers[0][2][8] = "。".to_string();
        // Layer 1: ー,・
        layers[1][2][7] = "ー".to_string();
        layers[1][2][8] = "・".to_string();
        // Layer 3: セミコロン
        layers[3][2][8] = ";".to_string();

        // シャッフルした文字を配置（固定位置と空白位置を除く119ポジション）
        let mut char_idx = 0;
        for layer in 0..NUM_LAYERS {
            for row in 0..ROWS {
                for col in 0..COLS {
                    // 固定位置と空白位置をスキップ
                    if !Self::is_fixed_position(layer, row, col) && !Self::is_blank_position(layer, row, col) {
                        if char_idx < chars.len() {
                            layers[layer][row][col] = chars[char_idx].clone();
                            char_idx += 1;
                        }
                    }
                }
            }
        }

        // デバッグ確認
        debug_assert_eq!(char_idx, chars.len(),
            "配置した文字数({})と用意した文字数({})が不一致", char_idx, chars.len());

        Self {
            layers,
            fitness: 0.0,
            scores: EvaluationScores::default(),
        }
    }

    /// シフト制限による空白位置かどうかを判定
    pub fn is_blank_position(layer: usize, row: usize, col: usize) -> bool {
        match layer {
            // Layer 1 (☆シフト): 右側制限
            1 => {
                if row == 0 && col >= 7 { return true; }  // row 0: cols 7,8,9
                if row == 1 && col >= 8 { return true; }  // row 1: cols 8,9
                if row == 2 && col == 9 { return true; }  // row 2: col 9 (cols 7,8は ー,・ 固定)
                false
            }
            // Layer 2 (★シフト): 左側制限
            2 => {
                if row == 0 && (col <= 2 || col == 9) { return true; }  // row 0: cols 0,1,2,9
                if row == 1 && col <= 1 { return true; }  // row 1: cols 0,1
                if row == 2 && (col <= 2 || col == 9) { return true; }  // row 2: cols 0,1,2,9
                false
            }
            // Layer 3 (◎シフト): 右側制限
            3 => {
                if row == 0 && col >= 8 { return true; }  // row 0: cols 8,9
                if row == 1 && col == 9 { return true; }  // row 1: col 9
                if row == 2 && col == 9 { return true; }  // row 2: col 9 (col 8は ; 固定)
                false
            }
            // Layer 4 (◆シフト): 左側+右端制限
            4 => {
                if row == 0 && (col <= 1 || col == 9) { return true; }  // row 0: cols 0,1,9
                if row == 1 && col == 0 { return true; }  // row 1: col 0
                if row == 2 && (col <= 1 || col == 9) { return true; }  // row 2: cols 0,1,9
                false
            }
            _ => false,
        }
    }

    /// 固定位置かどうかを判定
    /// 固定文字(9個): ◆, ★, ☆, ◎ (シフトキー) + 、, 。 (Layer0) + ー, ・ (Layer1) + ; (Layer3)
    pub fn is_fixed_position(layer: usize, row: usize, col: usize) -> bool {
        // Layer 0：シフトキー位置（◆=col1, ★=col2, ☆=col7, ◎=col8）
        if layer == 0 && row == 1 && (col == 1 || col == 2 || col == 7 || col == 8) {
            return true;
        }
        // Layer 0：句読点（、, 。）
        if layer == 0 && row == 2 && (col == 7 || col == 8) {
            return true;
        }
        // Layer 1：ー, ・
        if layer == 1 && row == 2 && (col == 7 || col == 8) {
            return true;
        }
        // Layer 3：セミコロン（;）
        if layer == 3 && row == 2 && col == 8 {
            return true;
        }
        false
    }

    /// 文字の位置を検索
    pub fn find_char(&self, c: &str) -> Option<KeyPos> {
        for layer in 0..NUM_LAYERS {
            for row in 0..ROWS {
                for col in 0..COLS {
                    if self.layers[layer][row][col] == c {
                        return Some(KeyPos::new(layer, row, col));
                    }
                }
            }
        }
        None
    }

    /// 文字→位置のマップを構築
    /// 2gram文字列の場合は最初の文字のみをキーとして使用
    pub fn build_char_map(&self) -> HashMap<char, KeyPos> {
        let mut map = HashMap::new();
        for layer in 0..NUM_LAYERS {
            for row in 0..ROWS {
                for col in 0..COLS {
                    let s = &self.layers[layer][row][col];
                    if let Some(c) = s.chars().next() {
                        if c != '　' && c != '\0' {
                            map.entry(c).or_insert(KeyPos::new(layer, row, col));
                        }
                    }
                }
            }
        }
        map
    }

    /// 配列を整形して文字列で返す
    pub fn format(&self) -> String {
        let mut result = String::new();

        for layer in 0..NUM_LAYERS {
            let label = match layer {
                0 => "Layer 0 (無シフト)",
                1 => "Layer 1 (Aシフト)",
                2 => "Layer 2 (Bシフト)",
                3 => "Layer 3 (Cシフト)",
                4 => "Layer 4 (Dシフト)",
                _ => "Unknown",
            };
            result.push_str(&format!("{}:\n", label));

            for row in 0..ROWS {
                result.push_str("  ");
                for col in 0..COLS {
                    let s = &self.layers[layer][row][col];
                    result.push_str(s);
                    result.push(' ');
                }
                result.push('\n');
            }
            result.push('\n');
        }

        result
    }

    /// 配列の検証（重複・不足チェック）
    /// Returns: (duplicates, missing, extra) - 問題があれば該当文字のリスト
    pub fn validate(&self, expected_chars: &[&str]) -> ValidationResult {
        use std::collections::{HashMap as StdHashMap, HashSet};

        let mut char_counts: StdHashMap<String, Vec<(usize, usize, usize)>> = StdHashMap::new();
        let mut found_chars: HashSet<String> = HashSet::new();

        // 全ポジションをスキャン
        for layer in 0..NUM_LAYERS {
            for row in 0..ROWS {
                for col in 0..COLS {
                    // 固定位置と空白位置はスキップ
                    if Self::is_fixed_position(layer, row, col) {
                        continue;
                    }
                    if Self::is_blank_position(layer, row, col) {
                        continue;
                    }

                    let s = &self.layers[layer][row][col];
                    if s != "　" && !s.is_empty() {
                        char_counts.entry(s.clone()).or_default().push((layer, row, col));
                        found_chars.insert(s.clone());
                    }
                }
            }
        }

        // 重複チェック
        let duplicates: Vec<(String, Vec<(usize, usize, usize)>)> = char_counts
            .iter()
            .filter(|(_, positions)| positions.len() > 1)
            .map(|(c, positions)| (c.clone(), positions.clone()))
            .collect();

        // 期待文字セット
        let expected: HashSet<String> = expected_chars.iter().map(|s| s.to_string()).collect();

        // 不足チェック（期待されているが見つからない）
        let missing: Vec<String> = expected
            .difference(&found_chars)
            .cloned()
            .collect();

        // 余分チェック（見つかったが期待されていない）
        let extra: Vec<String> = found_chars
            .difference(&expected)
            .cloned()
            .collect();

        ValidationResult {
            duplicates,
            missing,
            extra,
            total_found: found_chars.len(),
            total_expected: expected.len(),
        }
    }
}

/// 配列検証結果
#[derive(Debug)]
pub struct ValidationResult {
    /// 重複している文字と位置
    pub duplicates: Vec<(String, Vec<(usize, usize, usize)>)>,
    /// 不足している文字
    pub missing: Vec<String>,
    /// 余分な文字（期待リストにない）
    pub extra: Vec<String>,
    /// 見つかった文字数
    pub total_found: usize,
    /// 期待される文字数
    pub total_expected: usize,
}

impl ValidationResult {
    /// 重複がないかどうか（メインの検証基準）
    pub fn is_valid(&self) -> bool {
        self.duplicates.is_empty()
    }

    pub fn print_report(&self) {
        println!("\n=== 配列検証結果 ===");
        println!("配置文字数: {} (配置可能: 112)", self.total_found);

        if self.duplicates.is_empty() {
            println!("✓ 重複なし");
        } else {
            println!("✗ 重複あり ({} 件):", self.duplicates.len());
            for (c, positions) in &self.duplicates {
                let pos_str: Vec<String> = positions
                    .iter()
                    .map(|(l, r, c)| format!("L{}[{}][{}]", l, r, c))
                    .collect();
                println!("  「{}」: {}", c, pos_str.join(", "));
            }
        }

        // 不足は参考情報（119ポジションに135文字は入らないので）
        if !self.missing.is_empty() {
            println!("※ 未配置文字 ({} 件): 配置枠より文字数が多いため正常", self.missing.len());
        }

        if !self.extra.is_empty() {
            println!("※ 期待リスト外の文字 ({} 件):", self.extra.len());
            for c in &self.extra {
                print!("「{}」", c);
            }
            println!();
        }

        if self.is_valid() {
            println!("\n✓ 検証成功: 重複なし、配列は正常です");
        } else {
            println!("\n✗ 検証失敗: 重複があります");
        }
    }
}

// ============================================================================
// 月配列参照データ
// ============================================================================

/// 月配列の位置情報
pub struct TsukiLayout {
    pub char_positions: HashMap<char, KeyPos>,
}

impl TsukiLayout {
    /// 月配列2-263の配置を生成
    pub fn new() -> Self {
        let mut positions = HashMap::new();
        
        // Layer 0（表面）
        let layer0 = [
            ['そ', 'こ', 'し', 'て', 'ょ', 'つ', 'ん', 'い', 'の', 'り'],
            ['は', 'か', '☆', 'と', 'た', 'く', 'う', '★', '゛', 'き'],
            ['す', 'け', 'に', 'な', 'さ', 'っ', 'る', '、', '。', '゜'],
        ];
        
        // Layer 1（裏面）
        let layer1 = [
            ['ぁ', 'ひ', 'ほ', 'ふ', 'め', 'ぬ', 'え', 'み', 'や', 'ぇ'],
            ['ぃ', 'を', 'ら', 'あ', 'よ', 'ま', 'お', 'も', 'わ', 'ゆ'],
            ['ぅ', 'へ', 'せ', 'ゅ', 'ゃ', 'む', 'ろ', 'ね', 'ー', 'ぉ'],
        ];
        
        for (row, chars) in layer0.iter().enumerate() {
            for (col, &c) in chars.iter().enumerate() {
                if c != '☆' && c != '★' && c != '゛' && c != '゜' {
                    positions.insert(c, KeyPos::new(0, row, col));
                }
            }
        }
        
        for (row, chars) in layer1.iter().enumerate() {
            for (col, &c) in chars.iter().enumerate() {
                positions.insert(c, KeyPos::new(1, row, col));
            }
        }
        
        Self { char_positions: positions }
    }
}

impl Default for TsukiLayout {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ローマ字音素マッピング
// ============================================================================

/// かな文字からローマ字音素への分解
/// 戻り値: (子音, 母音)
pub fn romaji_phonemes(c: char) -> (Option<&'static str>, Option<&'static str>) {
    match c {
        'あ' => (None, Some("a")),
        'い' => (None, Some("i")),
        'う' => (None, Some("u")),
        'え' => (None, Some("e")),
        'お' => (None, Some("o")),
        'か' => (Some("k"), Some("a")),
        'き' => (Some("k"), Some("i")),
        'く' => (Some("k"), Some("u")),
        'け' => (Some("k"), Some("e")),
        'こ' => (Some("k"), Some("o")),
        'さ' => (Some("s"), Some("a")),
        'し' => (Some("s"), Some("i")),
        'す' => (Some("s"), Some("u")),
        'せ' => (Some("s"), Some("e")),
        'そ' => (Some("s"), Some("o")),
        'た' => (Some("t"), Some("a")),
        'ち' => (Some("t"), Some("i")),
        'つ' => (Some("t"), Some("u")),
        'て' => (Some("t"), Some("e")),
        'と' => (Some("t"), Some("o")),
        'な' => (Some("n"), Some("a")),
        'に' => (Some("n"), Some("i")),
        'ぬ' => (Some("n"), Some("u")),
        'ね' => (Some("n"), Some("e")),
        'の' => (Some("n"), Some("o")),
        'は' => (Some("h"), Some("a")),
        'ひ' => (Some("h"), Some("i")),
        'ふ' => (Some("h"), Some("u")),
        'へ' => (Some("h"), Some("e")),
        'ほ' => (Some("h"), Some("o")),
        'ま' => (Some("m"), Some("a")),
        'み' => (Some("m"), Some("i")),
        'む' => (Some("m"), Some("u")),
        'め' => (Some("m"), Some("e")),
        'も' => (Some("m"), Some("o")),
        'や' => (Some("y"), Some("a")),
        'ゆ' => (Some("y"), Some("u")),
        'よ' => (Some("y"), Some("o")),
        'ら' => (Some("r"), Some("a")),
        'り' => (Some("r"), Some("i")),
        'る' => (Some("r"), Some("u")),
        'れ' => (Some("r"), Some("e")),
        'ろ' => (Some("r"), Some("o")),
        'わ' => (Some("w"), Some("a")),
        'を' => (Some("w"), Some("o")),
        'ん' => (Some("n"), None),  // 「ん」は子音"n"のみ
        'が' => (Some("g"), Some("a")),
        'ぎ' => (Some("g"), Some("i")),
        'ぐ' => (Some("g"), Some("u")),
        'げ' => (Some("g"), Some("e")),
        'ご' => (Some("g"), Some("o")),
        'ざ' => (Some("z"), Some("a")),
        'じ' => (Some("z"), Some("i")),
        'ず' => (Some("z"), Some("u")),
        'ぜ' => (Some("z"), Some("e")),
        'ぞ' => (Some("z"), Some("o")),
        'だ' => (Some("d"), Some("a")),
        'ぢ' => (Some("d"), Some("i")),
        'づ' => (Some("d"), Some("u")),
        'で' => (Some("d"), Some("e")),
        'ど' => (Some("d"), Some("o")),
        'ば' => (Some("b"), Some("a")),
        'び' => (Some("b"), Some("i")),
        'ぶ' => (Some("b"), Some("u")),
        'べ' => (Some("b"), Some("e")),
        'ぼ' => (Some("b"), Some("o")),
        'ぱ' => (Some("p"), Some("a")),
        'ぴ' => (Some("p"), Some("i")),
        'ぷ' => (Some("p"), Some("u")),
        'ぺ' => (Some("p"), Some("e")),
        'ぽ' => (Some("p"), Some("o")),
        _ => (None, None),
    }
}

/// Colemakのキー位置マッピング
pub const COLEMAK_POSITIONS: &[(&str, usize, usize)] = &[
    // 母音位置
    ("a", 1, 0), ("e", 1, 7), ("i", 1, 8), ("o", 1, 9), ("u", 0, 7),
    // 子音位置
    ("k", 2, 6), ("s", 1, 2), ("t", 1, 3), ("n", 1, 6), ("h", 1, 5),
    ("m", 2, 7), ("y", 0, 8), ("r", 1, 1), ("w", 0, 1), ("g", 0, 4),
    ("z", 2, 0), ("d", 1, 4), ("b", 2, 4), ("p", 0, 3), ("f", 0, 2),
    ("j", 0, 5), ("l", 0, 6), ("v", 2, 3), ("q", 0, 0), ("x", 2, 1),
    ("c", 2, 2),
];
