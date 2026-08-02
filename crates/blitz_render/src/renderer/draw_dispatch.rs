//! 取得済み画像への実際の描画呼び出し。通常描画/読み戻しの`描画方式`を決め、各サブシステムから集めた入力束とフレーム画像一式を`vulkan::frame::描画する`へ渡す。ビュー射影行列等はdraw_execute.rsが事前にUBOへ書き込み済み(判断24)のため、ここではフレーム添字に対応するディスクリプタセットを選ぶだけでよい。
//! 描画方式の判別は`draw_mode`、作業領域の充填と資源表世代の確定は`work_area_fill`にある。

mod draw_mode;
mod work_area_fill;

use blitz_math::大域ワールド位置;

use super::cpu_timing::CPU区間時計;
use super::frame_progress::フレームスロット資源;
use super::presentation::取得済み提示;
use super::レンダラー;
use crate::clear_color::クリアカラー;
use crate::error::レンダラーエラー;
use crate::frame_input::フレーム描画入力;
use crate::frame_input::プリミティブ発行受け皿;
use crate::terrain_detail::地形詳細段選択;
use crate::visible_instance_selection::可視個体選択一覧;
use crate::vulkan;
use crate::vulkan::frame::UI描画入力;
use crate::vulkan::relative_anchor::カメラ相対の基準原点;

impl レンダラー {
    /// 戻り値: 提示劣化の有無と、記録の実績(GPUタイムスタンプの「パス名→クエリ開始添字」対応(判断30。計測無効なら空配列)と、このフレームで積んだ大気のベイク済み画像生成パスの本数)。
    /// `cpu区間時計`は作業領域の更新とコマンド記録以降の境界を刻む。計測が無効なら`None`であり、時刻を1度も読まない。
    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    pub(super) fn 現在の画像で描画する(
        &mut self,
        取得済み: &取得済み提示,
        スロット資源: &フレームスロット資源,
        mut cpu区間時計: Option<&mut CPU区間時計>,
        入力: &フレーム描画入力<'_>,
        クリア色: クリアカラー,
        露出: f32,
        布介入件数: u32,
        読み戻し要求: bool,
        ui入力: Option<&UI描画入力>,
        カメラ大域原点: 大域ワールド位置,
        地形詳細段選択一覧: &[地形詳細段選択],
        可視個体選択一覧: 可視個体選択一覧<'_>,
        プリミティブ発行: &プリミティブ発行受け皿,
    ) -> Result<(bool, vulkan::frame::記録の実績), レンダラーエラー> {
        let フレーム添字 = スロット資源.スロット;
        // フェンス待機と退役の回収は`draw_execute/prepare.rs`で済んでいる。借りた世代の材質のセットをこのフレームの
        // 全シーン描画発行が読み、保持の記録は送信が成った後に行う。
        let 資源表世代の束縛 = self.資源表世代を最新にして束縛を借りる()?;
        self.セット別束縛計数.数え直す();

        // 前提: このスロットのフェンス待機は`draw_execute/prepare.rs`で済んでおり、合成のディスクリプタを読むGPU作業は完了している。
        self.描画段階資源
            .合成の深度を結び直す(self.環境.device(), フレーム添字, 取得済み.画像組().深度ビュー);
        let 描画方式 = self.描画方式を決める(読み戻し要求)?;
        let 原点由来の基準原点 = カメラ相対の基準原点::世界原点から生成する(カメラ大域原点)?;
        let 任意材料 = self.任意入力の材料を集める(フレーム添字, 入力, 露出, 布介入件数, 原点由来の基準原点)?;
        let クエリプール = self.gpu計測.as_ref().map(|計測| 計測.クエリプール(フレーム添字));
        let 画像一式 = self.フレーム画像一式を組み立てる(取得済み);
        let 提示id = self.実表示計測.提示idを発番する();
        work_area_fill::積む(
            self,
            work_area_fill::充填の材料 {
                フレーム添字,
                資源表世代の束縛,
                カメラ大域原点,
                地形詳細段選択一覧,
                可視個体選択一覧,
                プリミティブ発行,
            },
            cpu区間時計.as_deref_mut(),
        )?;

        let 結果 = vulkan::frame::描画する(
            self.環境.device(),
            self.環境.queue(),
            スロット資源.command_buffer,
            &self.フレーム構成,
            self.提示先を組み立てる(取得済み, 提示id),
            &画像一式,
            self.提示.寸法(),
            クリア色,
            self.描画段階資源.シーンpipeline(),
            self.シーン描画資源.描画対象入力を作る(self.共有ディスクリプタ.束縛を作る(
                フレーム添字,
                資源表世代の束縛.材質のセット,
                &self.セット別束縛計数,
            )),
            任意材料.借用する(ui入力),
            描画方式,
            クエリプール,
            self.同期入力を組み立てる(スロット資源, 取得済み),
        );
        if let Some(時計) = cpu区間時計 {
            時計.描画を終了する();
        }
        if 結果.is_ok() {
            // 送信が成ったフレームだけが世代を保持する。前で失敗したフレームは何も保持しないため、次に同じスロットを使うフレームがそのまま束縛できる。
            self.材質資源表.フレームの保持を記録する(フレーム添字);
        }
        結果
    }
}
