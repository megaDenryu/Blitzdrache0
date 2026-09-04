//! `blitz_app`のアプリのモジュールの木を除いた未是正の自由関数の一覧。検収の報告と入力の状態を丸ごと受け取るものが並ぶ。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 18] = [
    未是正の自由関数::生成する("build_support/slangc_entry_compile.rs", "エントリを1つコンパイルする", "エントリ指定"),
    未是正の自由関数::生成する("src/cli/types/frame_dump_setting.rs", "走査の書き出し先を確かめる", "フレームダンプ指定"),
    未是正の自由関数::生成する("src/input/confirm.rs", "確定する", "入力状態"),
    未是正の自由関数::生成する("src/input/ingest.rs", "カーソル移動を反映する", "入力状態"),
    未是正の自由関数::生成する("src/input/ingest.rs", "キー入力を反映する", "入力状態"),
    未是正の自由関数::生成する("src/input/ingest.rs", "ホイールを反映する", "入力状態"),
    未是正の自由関数::生成する("src/input/ingest.rs", "取り込む", "入力状態"),
    未是正の自由関数::生成する("src/input/ingest.rs", "左ボタンを反映する", "入力状態"),
    未是正の自由関数::生成する("src/reports/cloth_xpbd_reference/lines.rs", "曲げの硬さを出す", "曲げの硬さの観測"),
    未是正の自由関数::生成する("src/reports/cloth_xpbd_reference/lines.rs", "硬さを出す", "硬さの観測"),
    未是正の自由関数::生成する("src/reports/indirect_probe.rs", "代表板を照合する", "照合の材料"),
    未是正の自由関数::生成する("src/reports/indirect_probe.rs", "板1枚を照合する", "照合の材料"),
    未是正の自由関数::生成する("src/reports/indirect_probe/expected_color.rs", "期待を求める", "期待の材料"),
    未是正の自由関数::生成する("src/reports/indirect_probe/specular_lookup.rs", "注入した鏡面の色", "鏡面の参照先"),
    未是正の自由関数::生成する("src/reports/sky_pixel.rs", "代表画素を照合する", "照合の材料"),
    未是正の自由関数::生成する("src/reports/sky_pixel.rs", "照合する", "照合の材料"),
    未是正の自由関数::生成する("src/reports/sky_pixel/expected_radiance.rs", "期待を求める", "期待の材料"),
    未是正の自由関数::生成する("src/reports/sky_pixel/view_ray.rs", "画素の視線を求める", "照合の材料"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates/blitz_app", &項目一覧)
}
