//! 試験が差し込む替え玉。壊れる端末・中身を覚えるログ・必ず失敗するログ・読まれた本数を数える読み元を持つ。
//!
//! 本番の型と替え玉を同じファイルへ置かないのは、替え玉が「試験のために本物の代わりに置くもの」という
//! 1つの責務を持ち、どの試験からも同じ形で使われるためである。

use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use super::super::terminal_copy::{端末の書き込み先, 端末の流れの種別};

/// 決めた回数だけ書き込みに成功し、その後は管が壊れたことにする端末。
pub struct 途中から壊れる端末 {
    残りの成功回数: AtomicUsize,
}

impl 途中から壊れる端末 {
    pub fn 残りの成功回数を決めて作る(残りの成功回数: usize) -> Self {
        Self {
            残りの成功回数: AtomicUsize::new(残りの成功回数),
        }
    }
}

impl 端末の書き込み先 for 途中から壊れる端末 {
    fn 掃き出しながら書く(&self, _種別: &端末の流れの種別, _塊: &[u8]) -> std::io::Result<()> {
        let 残り = self.残りの成功回数.load(Ordering::Relaxed);
        if 残り == 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "端末の管が壊れた"));
        }
        self.残りの成功回数.store(残り - 1, Ordering::Relaxed);
        Ok(())
    }
}

/// 書かれた中身を覚えておくログ。試験は覚えた中身を後から読んで、欠けが無いことを見る。
pub struct 覚えているログ {
    中身: Arc<Mutex<Vec<u8>>>,
}

impl 覚えているログ {
    pub fn 同じ中身を指して作る(中身: &Arc<Mutex<Vec<u8>>>) -> Self {
        Self { 中身: Arc::clone(中身) }
    }
}

impl Write for 覚えているログ {
    fn write(&mut self, 塊: &[u8]) -> std::io::Result<usize> {
        let mut 中身 = self.中身.lock().unwrap_or_else(|毒| 毒.into_inner());
        中身.extend_from_slice(塊);
        Ok(塊.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// 一次の記録そのものが壊れた場合を作る、必ず失敗するログ。
pub struct 必ず失敗するログ;

impl Write for 必ず失敗するログ {
    fn write(&mut self, _塊: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("ログの置き場が壊れた"))
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// 決めた本数だけ塊を返してから終わりを告げる読み元。何本読まれたかを数え札へ記録する。
pub struct 数える読み元 {
    残りの本数: usize,
    読まれた本数: Arc<AtomicUsize>,
}

impl 数える読み元 {
    pub fn 本数と数え札を決めて作る(本数: usize, 読まれた本数: &Arc<AtomicUsize>) -> Self {
        Self {
            残りの本数: 本数,
            読まれた本数: Arc::clone(読まれた本数),
        }
    }
}

impl std::io::Read for 数える読み元 {
    fn read(&mut self, 緩衝: &mut [u8]) -> std::io::Result<usize> {
        if self.残りの本数 == 0 {
            return Ok(0);
        }
        self.残りの本数 -= 1;
        let 番号 = self.読まれた本数.fetch_add(1, Ordering::Relaxed);
        let 中身 = format!("塊{番号}\n");
        緩衝[..中身.len()].copy_from_slice(中身.as_bytes());
        Ok(中身.len())
    }
}
