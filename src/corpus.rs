//! コーパスモジュール
//! 
//! N-gramデータの読み込みと管理を行う。

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// コーパス統計データ
#[derive(Clone, Debug)]
pub struct CorpusStats {
    /// 1-gram（文字）頻度
    pub char_freq: HashMap<char, usize>,
    /// 2-gram（連続2文字）頻度
    pub bigram_freq: HashMap<(char, char), usize>,
    /// 3-gram（連続3文字）頻度
    pub trigram_freq: HashMap<(char, char, char), usize>,
    /// 4-gram（連続4文字）頻度
    pub fourgram_freq: HashMap<(char, char, char, char), usize>,
}

impl CorpusStats {
    /// 空のコーパス統計を作成
    pub fn new() -> Self {
        Self {
            char_freq: HashMap::new(),
            bigram_freq: HashMap::new(),
            trigram_freq: HashMap::new(),
            fourgram_freq: HashMap::new(),
        }
    }

    /// N-gramファイルからコーパス統計を読み込む
    /// 
    /// ファイル形式: `count\tcharacters\tn` (タブ区切り)
    /// - count: 出現回数
    /// - characters: 文字列（1-gram〜4-gram）
    /// - n: N-gramのN値
    /// 
    /// `〓` は改行を表すノイズとして除外される。
    pub fn from_ngram_files(
        gram1_path: Option<&Path>,
        gram2_path: Option<&Path>,
        gram3_path: Option<&Path>,
        gram4_path: Option<&Path>,
    ) -> Result<Self, std::io::Error> {
        let mut stats = Self::new();

        // 1-gram
        if let Some(path) = gram1_path {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((count, chars)) = Self::parse_ngram_line(&line) {
                    let c: Vec<char> = chars.chars().collect();
                    if c.len() == 1 && c[0] != '〓' {
                        stats.char_freq.insert(c[0], count);
                    }
                }
            }
        }

        // 2-gram
        if let Some(path) = gram2_path {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((count, chars)) = Self::parse_ngram_line(&line) {
                    let c: Vec<char> = chars.chars().collect();
                    if c.len() == 2 && !c.contains(&'〓') {
                        stats.bigram_freq.insert((c[0], c[1]), count);
                    }
                }
            }
        }

        // 3-gram
        if let Some(path) = gram3_path {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((count, chars)) = Self::parse_ngram_line(&line) {
                    let c: Vec<char> = chars.chars().collect();
                    if c.len() == 3 && !c.contains(&'〓') {
                        stats.trigram_freq.insert((c[0], c[1], c[2]), count);
                    }
                }
            }
        }

        // 4-gram
        if let Some(path) = gram4_path {
            let file = File::open(path)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                if let Some((count, chars)) = Self::parse_ngram_line(&line) {
                    let c: Vec<char> = chars.chars().collect();
                    if c.len() == 4 && !c.contains(&'〓') {
                        stats.fourgram_freq.insert((c[0], c[1], c[2], c[3]), count);
                    }
                }
            }
        }

        Ok(stats)
    }

    /// N-gram行をパース
    /// 形式: `count\tcharacters\tn`
    fn parse_ngram_line(line: &str) -> Option<(usize, String)> {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            if let Ok(count) = parts[0].parse::<usize>() {
                return Some((count, parts[1].to_string()));
            }
        }
        None
    }

    /// テキストファイルからコーパス統計を計算
    pub fn from_text(text: &str) -> Self {
        let mut stats = Self::new();
        let chars: Vec<char> = text.chars().collect();

        // 1-gram
        for &c in &chars {
            *stats.char_freq.entry(c).or_insert(0) += 1;
        }

        // 2-gram
        for window in chars.windows(2) {
            let key = (window[0], window[1]);
            *stats.bigram_freq.entry(key).or_insert(0) += 1;
        }

        // 3-gram
        for window in chars.windows(3) {
            let key = (window[0], window[1], window[2]);
            *stats.trigram_freq.entry(key).or_insert(0) += 1;
        }

        // 4-gram
        for window in chars.windows(4) {
            let key = (window[0], window[1], window[2], window[3]);
            *stats.fourgram_freq.entry(key).or_insert(0) += 1;
        }

        stats
    }

    /// テキストファイルを読み込んでコーパス統計を計算
    pub fn from_text_file(path: &Path) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        Ok(Self::from_text(&content))
    }

    /// コーパスの総文字数
    pub fn total_chars(&self) -> usize {
        self.char_freq.values().sum()
    }

    /// コーパスの総2-gram数
    pub fn total_bigrams(&self) -> usize {
        self.bigram_freq.values().sum()
    }

    /// コーパスの総3-gram数
    pub fn total_trigrams(&self) -> usize {
        self.trigram_freq.values().sum()
    }

    /// 統計情報のサマリーを表示
    pub fn summary(&self) -> String {
        format!(
            "Corpus Stats:\n  1-gram types: {}\n  2-gram types: {}\n  3-gram types: {}\n  4-gram types: {}\n  Total chars: {}",
            self.char_freq.len(),
            self.bigram_freq.len(),
            self.trigram_freq.len(),
            self.fourgram_freq.len(),
            self.total_chars()
        )
    }
}

impl Default for CorpusStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_text() {
        let text = "あいうえお";
        let stats = CorpusStats::from_text(text);
        
        assert_eq!(stats.char_freq.len(), 5);
        assert_eq!(stats.bigram_freq.len(), 4);
        assert_eq!(stats.trigram_freq.len(), 3);
    }

    #[test]
    fn test_parse_ngram_line() {
        let line = "1234\tあい\t2";
        let result = CorpusStats::parse_ngram_line(line);
        assert_eq!(result, Some((1234, "あい".to_string())));
    }
}
