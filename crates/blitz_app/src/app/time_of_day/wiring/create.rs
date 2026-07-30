//! 天空配線の生成局面。呼ばれるのは起動時の1回だけであり、以降のフレームは値を読むだけである。
//! 担当するのは「シーン名と起動指定から世界の空方針を決め、時計と下ろし済みの大気を組み、初回のライティングを導く」ことである。

use blitz_engine::sky::世界の空方針;
use blitz_render::ライティング入力;

use super::super::atmosphere_update::大気更新判定;
use super::super::clock::時間帯;
use super::super::scene_policy;
use super::super::sun_disk_override;
use super::天空配線;
use crate::atmosphere_medium::確定した大気散乱媒体へ写す;
use crate::cli::時間帯起動設定;

pub(super) fn 生成する(シーン名: &str, 設定: &時間帯起動設定, 基準: ライティング入力) -> 天空配線 {
    let 方針 = scene_policy::世界の空方針を決める(シーン名, 設定.空);
    let 方針 = sun_disk_override::太陽円盤を反映する(方針, 設定.太陽円盤);
    let (時間帯, 大気) = match 方針 {
        世界の空方針::空なし => (None, None),
        世界の空方針::空あり { 空と太陽, 既定時刻 } => (
            Some(時間帯::生成する(空と太陽, 設定.初期時刻を決める(既定時刻), 設定.時間倍率)),
            Some((空と太陽.大気媒体(), 確定した大気散乱媒体へ写す(&空と太陽.大気媒体()))),
        ),
    };
    let mut 配線 = 天空配線 {
        時間帯,
        空を描く: scene_policy::空を描くか(方針, 設定.空),
        大気,
        大気更新判定: 大気更新判定::新規(),
        基準ライティング: 基準,
        ライティング: 基準,
    };
    配線.ライティングを導き直す();
    配線
}
