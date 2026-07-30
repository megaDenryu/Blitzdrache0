//! 終了時にどの報告を出すかの問い合わせ。読むのは起動設定から写した真偽値だけであり、レンダラーにもフレームの状態にも触れない。
//! 値の取り出し(`queries`本体)と分けているのは、こちらが「求められたか」を、あちらが「いくつだったか」を答える別の問いだからである。

use crate::app::アプリ;

impl アプリ {
    /// `--report-gpu-times`が指定されたか。
    pub(crate) fn gpu時間報告が必要か(&self) -> bool {
        self.報告要求.gpu時間
    }

    /// `--report-atmosphere-passes`が指定されたか。
    pub(crate) fn 大気のベイク済み画像パス数報告が必要か(&self) -> bool {
        self.報告要求.大気のベイク済み画像生成パス数
    }

    pub(crate) fn フレーム時間報告が必要か(&self) -> bool {
        self.フレーム間隔計測.is_some()
    }

    pub(crate) fn gpuメモリ報告が必要か(&self) -> bool {
        self.報告要求.gpuメモリ
    }

    /// `--report-draw-issue`が指定されたか。
    pub(crate) fn 描画発行報告が必要か(&self) -> bool {
        self.報告要求.描画発行
    }

    /// `--report-instance-sections`が指定されたか。
    pub(crate) fn インスタンス区間報告が必要か(&self) -> bool {
        self.可視個体の選別の計測.is_some()
    }

    /// `--report-display-timing`が指定されたか。
    pub(crate) fn 実表示時間報告が必要か(&self) -> bool {
        self.報告要求.実表示時間
    }

    /// `--report-streaming-summary`が指定されたか。
    pub(crate) fn ストリーミング要約報告が必要か(&self) -> bool {
        self.報告要求.ストリーミング要約
    }
}
