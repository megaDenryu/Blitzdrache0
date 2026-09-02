//! 布のXPBDの参照比較の報告(Issue #36「検証」)。終了時にGPUから読み戻した布の粒子とラグランジュ乗数を、同じ初期条件・同じ刻み数・
//! 同じ反復回数で回したCPUの参照計算(正典式)と突き合わせ、機械可読な行として標準出力へ出す。
//! 判定は`cargo xtask cloth-xpbd-reference`がこの出力を読んで行い、ここは事実の行だけを出す。
//! 突き合わせは`comparison`、反復回数と刻み幅を変えたときの硬さは`stiffness`、行の綴りは`lines`が持つ。

mod comparison;
mod lines;
mod stiffness;

use blitz_render::cloth_material::布の読み戻し;
use blitz_render::レンダラーエラー;

use crate::app::cloth_reference::布の参照比較;

pub(crate) fn 参照比較を表示する(
    比較: &布の参照比較, 読み戻し: Option<Result<Option<布の読み戻し>, レンダラーエラー>>
) {
    lines::題材を出す(比較);
    let 読み戻し = match 読み戻し {
        Some(Ok(Some(読み戻し))) => 読み戻し,
        Some(Ok(None)) => return lines::中止を出す("レンダラーが布を持っていない"),
        Some(Err(誤り)) => return lines::中止を出す(&format!("読み戻しに失敗した: {誤り}")),
        None => return lines::中止を出す("レンダラーが生成されなかった"),
    };
    let 参照 = match 比較.参照計算を回す() {
        Ok(参照) => 参照,
        Err(誤り) => return lines::中止を出す(&format!("CPUの参照計算を作れない: {誤り}")),
    };
    let 突き合わせ = comparison::突き合わせる(&読み戻し, &参照);
    lines::差を出す(&突き合わせ);
    lines::拘束違反を出す(&突き合わせ, &参照);
    stiffness::硬さを出す(比較);
}
