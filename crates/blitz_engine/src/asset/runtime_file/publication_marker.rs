//! 実行時アセット一式の公開完了を示す生成台帳のファイル名を、書き手と監視側で共有する契約。

pub struct 実行時アセットの公開完了印;

impl 実行時アセットの公開完了印 {
    pub const fn ファイル名() -> &'static str {
        "generation_ledger.txt"
    }
}
