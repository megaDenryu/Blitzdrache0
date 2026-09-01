//! 天空配線の生成局面。呼ばれるのは起動時の1回だけであり、以降のフレームは値を読むだけである。
//! 担当するのは「世界の種別と起動指定から世界の空方針と間接照明方針を決め、時計と下ろし済みの大気を組み、
//! 方針から照明問い合わせ契約を立て、初回のライティングを導く」ことである。

use blitz_engine::sky::世界の空方針;
use blitz_render::{ライティング入力, レンダラーエラー};

use super::super::aerial_distance_record::空中遠近の最遠距離の記録;
use super::super::atmosphere_update::大気更新判定;
use super::super::clock::時間帯;
use super::super::distant_environment_update::遠方環境更新判定;
use super::super::exposure_elapsed::自動露出の経過秒源;
use super::super::scene_policy;
use super::super::sun_disk_override;
use super::super::thinning::遠方環境の間引き;
use super::天空配線;
use crate::atmosphere_medium::確定した大気散乱媒体へ写す;
use crate::cli::{世界の種別, 時間帯起動設定, 起動モード};

/// 天空配線を組むのに要る材料。呼び出し元が起動設定から取り出して渡す。
pub(in crate::app) struct 生成材料<'材料> {
    pub(in crate::app) 種別: 世界の種別,
    pub(in crate::app) 時間帯: &'材料 時間帯起動設定,
    pub(in crate::app) 起動モード: 起動モード, // 自動露出の経過秒を実時間で進めるか固定の刻みで進めるかを決める
    pub(in crate::app) 基準ライティング: ライティング入力,
    pub(in crate::app) 明示境界を使わない: bool, // --no-explicit-shadow-bandsで真になる。大規模世界でも距離区分を再配分せず実用分割のまま走る
}

pub(super) fn 生成する(材料: 生成材料<'_>) -> Result<天空配線, レンダラーエラー> {
    let (種別, 設定, 基準) = (材料.種別, 材料.時間帯, 材料.基準ライティング);
    let 明示境界を使わない = 材料.明示境界を使わない;
    let 方針 = scene_policy::世界の空方針を決める(種別, 設定.空);
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
        空中遠近の最遠距離の記録: 空中遠近の最遠距離の記録::新規(),
        照明問い合わせ契約: scene_policy::世界の間接照明方針を決める(種別).照明問い合わせ契約へ写す(大気.is_some())?,
        露出方式: scene_policy::世界の露出方式を決める(種別, 設定.自動露出),
        自動露出の経過秒源: 自動露出の経過秒源::起動モードから決める(材料.起動モード),
        遠方環境更新判定: 遠方環境更新判定::新規(),
        間引き: 遠方環境の間引き::新規(),
        段差走査: None,
        基準ライティング: 基準,
        明示境界を使わない,
        ライティング: 基準,
    };
    配線.ライティングを導き直す();
    Ok(配線)
}
