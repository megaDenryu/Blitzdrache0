//! 布のXPBD参照比較の報告の行の見出し。見出しは行の種類を決める語であり、読む側は見出しで行を選んでから鍵の値を読む。
//! 片側だけ動くと、その種類の行が1本も読まれないまま「見出しの行が無い」という失敗になる。

use super::綴りの契約;

const 出す側: &str = "crates/blitz_app/src/reports/cloth_xpbd_reference/lines.rs";
const 読む側: &str = "xtask/src/cloth_xpbd_reference/parse.rs";
const 両側: &[&str] = &[出す側, 読む側];

pub(super) const 綴り一覧: [綴りの契約; 4] = [
    綴りの契約 {
        綴り: "布XPBD参照比較",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "布XPBD参照比較の差",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "布XPBD参照比較の拘束違反",
        現れるファイル一覧: 両側,
    },
    綴りの契約 {
        綴り: "布XPBD参照の硬さ",
        現れるファイル一覧: &[出す側],
    },
];
