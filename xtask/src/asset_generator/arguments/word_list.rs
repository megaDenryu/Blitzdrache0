//! 組み立て中の語の並び。所有するのは積み上げ途中の語の列であり、値として置く語が選択肢に見えないことの
//! 確認をここに閉じる。どの生成器がどの語を積むかは親が決め、ここは積み方だけを知る。
//!
//! パスと綴りで積む口を分けるのは、パスが`OsString`のまま渡せて綴りの検査を要さないためである。

use std::ffi::OsString;
use std::path::Path;

use super::super::error::生成器エラー;

pub(super) struct 語の並び(Vec<OsString>);

impl 語の並び {
    pub(super) fn 空から始める() -> Self {
        Self(Vec::new())
    }

    pub(super) fn 語を足す(&mut self, 語: &str) {
        self.0.push(OsString::from(語));
    }

    pub(super) fn パスを足す(&mut self, パス: &Path) {
        self.0.push(パス.as_os_str().to_os_string());
    }

    /// 選択肢を伴う値と、位置で渡す値の両方をここが積む。値が選択肢に見えると、生成器側の
    /// 「知らない引数である」という遠い失敗になるため、積む手前で落とす。
    pub(super) fn 値を足す(
        &mut self, 選択肢: Option<&'static str>, 役割: &'static str, 値: Option<String>
    ) -> Result<(), 生成器エラー> {
        let Some(語) = 値 else { return Ok(()) };
        if 語.starts_with('-') {
            return Err(生成器エラー::値が選択肢の綴りに見える { 役割, 綴り: 語 });
        }
        if let Some(綴り) = 選択肢 {
            self.語を足す(綴り);
        }
        self.語を足す(&語);
        Ok(())
    }

    /// 積み終えた語の列を取り出す。完成した生成引数へ移る境界である。
    pub(super) fn 取り出す(self) -> Vec<OsString> {
        self.0
    }
}
