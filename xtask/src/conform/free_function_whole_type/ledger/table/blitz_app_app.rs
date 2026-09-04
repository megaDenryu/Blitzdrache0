//! `blitz_app`のアプリのモジュールの木の未是正の自由関数の一覧。`アプリ`を丸ごと受け取るものが大半であり、コンポジションルートの多段化で消える。
//!
//! 注意: この一覧への追加は禁止する。減らす方向にのみ動かす。削除できるのは、その工程が自分の触るものだけを
//! 名前の付いた引数で受け取る形へ直したときか、操作を親の型のメソッドへ移したときだけである。

use super::super::{区画の一覧, 未是正の自由関数};

const 項目一覧: [未是正の自由関数; 27] = [
    未是正の自由関数::生成する("draw_dispatch/rewrite_action.rs", "適用する", "アプリ"),
    未是正の自由関数::生成する(
        "draw_dispatch/temporal_reconstruction_check/readback.rs",
        "今のフレームの色を読み戻す",
        "アプリ",
    ),
    未是正の自由関数::生成する(
        "draw_dispatch/temporal_reconstruction_check/readback.rs",
        "圧縮前のhdrを読み戻す",
        "アプリ",
    ),
    未是正の自由関数::生成する(
        "draw_dispatch/temporal_reconstruction_injection/canon.rs",
        "差を取り込む",
        "突き合わせの要約",
    ),
    未是正の自由関数::生成する(
        "draw_dispatch/temporal_reconstruction_injection/synthesis.rs",
        "一画素を積む",
        "合成入力の材料",
    ),
    未是正の自由関数::生成する("frame/action.rs", "選ぶ", "アプリ"),
    未是正の自由関数::生成する("frame/borrowed_draw.rs", "受け皿を預けて描画する", "アプリ"),
    未是正の自由関数::生成する("frame/draw_input.rs", "組み立てる", "アプリ"),
    未是正の自由関数::生成する("frame/draw_input/frame_view.rs", "画素内ずらしを決める", "アプリ"),
    未是正の自由関数::生成する("frame/draw_input/frame_view.rs", "視点を求める", "アプリ"),
    未是正の自由関数::生成する("frame/finish.rs", "進めて必要なら終了する", "アプリ"),
    未是正の自由関数::生成する("frame_dump/cluster_assignment_check.rs", "報告する", "アプリ"),
    未是正の自由関数::生成する("frame_dump/depth_dump.rs", "読み戻して書き出す", "アプリ"),
    未是正の自由関数::生成する("frame_dump/dump_destination.rs", "決める", "アプリ"),
    未是正の自由関数::生成する("frame_dump/hdr_dump.rs", "読み戻して書き出す", "アプリ"),
    未是正の自由関数::生成する("frame_dump/indirect_probe_check.rs", "照合する", "アプリ"),
    未是正の自由関数::生成する("frame_dump/presentation_dump.rs", "読み戻して書き出す", "アプリ"),
    未是正の自由関数::生成する("frame_dump/sky_pixel_check.rs", "照合する", "アプリ"),
    未是正の自由関数::生成する("handler/resume.rs", "格納する", "アプリ"),
    未是正の自由関数::生成する("handler/resume.rs", "生成してアプリへ格納する", "アプリ"),
    未是正の自由関数::生成する("hot_reload_asset_apply/validation.rs", "反映前の検査を通す", "アプリ"),
    未是正の自由関数::生成する("measurement_setup.rs", "レンダラーの計測を有効にする", "アプリ"),
    未是正の自由関数::生成する("measurement_setup.rs", "実表示計測要求を決める", "アプリ"),
    未是正の自由関数::生成する("primitive_draw_item_registry/refill.rs", "詰め直す", "プリミティブ描画項目台帳"),
    未是正の自由関数::生成する("time_of_day/atmosphere_input.rs", "組む", "大気入力の材料"),
    未是正の自由関数::生成する("time_of_day/distant_environment_input.rs", "組む", "遠方環境入力の材料"),
    未是正の自由関数::生成する("visibility/frame_select.rs", "一フレーム分を選ぶ", "選択の条件"),
];

pub fn 一覧() -> 区画の一覧 {
    区画の一覧::生成する("crates/blitz_app/src/app", &項目一覧)
}
