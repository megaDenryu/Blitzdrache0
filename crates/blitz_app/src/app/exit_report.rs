//! 終了時にどの報告を出すかの取りまとめの局面。呼ばれるのはプロセスの終わりに1回だけであり、毎フレーム呼ばれる
//! 描画とは呼び出し頻度も触れるフィールドも違う。行の整形は`reports`配下の各報告が持ち、ここは順序だけを決める。
//!
//! 注意: レンダラー内部を読む報告(GPU時間・GPUメモリ・描画発行)は破棄より前に呼ぶ必要があるため、この局面は
//! `レンダラーを破棄する`より前に1回だけ呼ぶ。

use crate::reports::{display_timing, streaming_summary};

impl super::アプリ {
    pub(crate) fn 終了時報告を出す(&self) {
        if let Some(要約) = self.ゲームの進行の要約を作る() {
            crate::reports::game::ゲームの進行を表示する(&要約);
        }
        if self.gpu時間報告が必要か() {
            crate::reports::gpu_time_table::表示する(&self.パス別gpu時間を取得する());
        }
        if self.gpu時間のフレーム別生値報告が必要か() {
            crate::reports::gpu_frame_samples::表示する(self.パス別gpu時間のフレーム別標本を取得する());
        }
        if self.大気のベイク済み画像パス数報告が必要か() {
            match self.大気のベイク済み画像生成パス数の記録を取得する() {
                Some(記録) => crate::reports::atmosphere_passes::大気のベイク済み画像生成パス数を表示する(記録),
                None => println!("大気のベイク済み画像生成パス数: レンダラーが生成されなかったため数えていない"),
            }
            match self.間接照明生成パス数の記録を取得する() {
                Some(記録) => {
                    crate::reports::atmosphere_passes::間接照明生成パス数を表示する(記録);
                    crate::reports::atmosphere_passes::間接照明生成パス数の見込みを表示する(
                        self.間接照明の焼き上げ本数の見込みを取得する(),
                    );
                }
                None => println!("間接照明生成パス数: レンダラーが生成されなかったため数えていない"),
            }
            crate::reports::distant_environment_key::遠方環境の鍵を表示する(self.遠方環境の鍵の記録を取得する());
            crate::reports::distant_environment_key::太陽天頂区間を表示する(self.太陽天頂区間の記録を取得する());
        }
        if self.gpuメモリ報告が必要か() {
            match self.gpuメモリ統計を取得する() {
                Some(統計) => crate::reports::gpuメモリ統計を表示する(&統計),
                None => println!("Vulkanメモリ確保: レンダラーが生成されなかったため取得できない"),
            }
        }
        if self.描画発行報告が必要か() {
            match self.描画発行内訳を取得する() {
                Some(内訳) => crate::reports::draw_issue::描画発行内訳を表示する(&内訳),
                None => println!("描画発行内訳: レンダラーが生成されなかったため取得できない"),
            }
            match self.点光源の影の記録内訳を取得する() {
                Some(内訳) => crate::reports::point_light_shadow::点光源の影の記録内訳を表示する(&内訳),
                None => println!("点光源の影の記録内訳: レンダラーが生成されなかったため取得できない"),
            }
            let 計器 = self.可視個体の選別の計器を取得する();
            crate::reports::draw_issue::可視個体の選別の計数を表示する(計器.計数);
            crate::reports::draw_issue::距離区分の世界メートル毎テクセルを表示する(計器.解像度密度);
            match self.セット別ディスクリプタ束縛回数を取得する() {
                Some(回数) => crate::reports::draw_issue::セット別束縛回数を表示する(回数),
                None => println!("  セット別ディスクリプタ束縛回数: レンダラーが生成されなかったため取得できない"),
            }
            match self.材質資源表の要約を取得する() {
                Some(要約) => crate::reports::draw_issue::材質資源表の要約を表示する(要約),
                None => println!("  材質資源表: レンダラーが生成されなかったため取得できない"),
            }
            crate::reports::draw_issue::段切替回数を表示する(self.個体詳細段切替回数を取得する());
            crate::reports::draw_issue::可視材料登録数を表示する(self.可視材料登録数を取得する());
            let (読込総数, 起動後読込) = self.シーン読込回数を取得する();
            crate::reports::draw_issue::シーン読込回数を表示する(読込総数, 起動後読込);
        }
        if self.キャスター距離分布報告が必要か() {
            crate::reports::draw_issue::キャスター距離分布を表示する(&self.可視個体の選別の計器を取得する().距離分布);
        }
        if self.太陽角度報告が必要か() {
            crate::reports::sun_angle::太陽の角度を表示する(self.天空状態を取得する());
        }
        if self.フレーム時間報告が必要か() {
            match self.フレーム時間統計を取得する() {
                Some(統計) => crate::reports::フレーム時間統計を表示する(&統計),
                None => println!("CPU側フレーム間隔: 計測できなかった(ウォームアップ後のフレームがない)"),
            }
            crate::reports::レンダラーcpu区間を表示する(self.レンダラーcpu区間時間を取得する());
        }
        if self.インスタンス区間報告が必要か() {
            if !self.フレーム時間報告が必要か() {
                crate::reports::レンダラーcpu区間を表示する(self.レンダラーcpu区間時間を取得する());
            }
            crate::reports::可視個体の選別の区間を表示する(self.可視個体の選別の区間統計を取得する().as_ref());
        }
        if self.ストリーミング要約報告が必要か() {
            match self.ストリーミング要約を取得する() {
                Some(要約) => streaming_summary::表示する(&要約),
                None => println!("ストリーミング要約: --streamingが指定されていないため計測していない"),
            }
        }
        if self.実表示時間報告が必要か() {
            display_timing::実表示間隔を表示する(self.実表示計測状況を取得する(), self.実表示観測一覧を取得する());
        }
    }
}
