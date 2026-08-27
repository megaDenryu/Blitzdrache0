import type { 楽曲ID } from '../../境界/index.ts'
import type { 楽曲接続 } from '../../境界/通信/index.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import type { 楽曲編集画面 } from './画面/index.ts'

// ツール起動時にサーバーから楽曲データを非同期に読み込み、画面へ反映する。
export async function 起動時に楽曲を読み込む(
    画面: 楽曲編集画面,
    状態: 楽曲編集状態,
    UI状態: 楽曲編集UI状態,
    接続: 楽曲接続,
    楽曲ID: 楽曲ID,
): Promise<void> {
    try {
        const 結果 = await 接続.楽曲を読む(楽曲ID)
        if (結果.種別 === '成功') {
            状態.状態を上書きする(結果.値)
            画面.永続化.状態文言を更新する('読込完了(起動時)', false)
        } else if (結果.種別 === '無し') {
            画面.永続化.状態文言を更新する('未保存: 初期楽曲を表示中', false)
        } else {
            画面.永続化.状態文言を更新する(
                `起動時読込失敗: ${結果.エラー.種別} ${結果.エラー.説明}`,
                true,
            )
        }
    } catch (原因: unknown) {
        const メッセージ = 原因 instanceof Error ? 原因.message : String(原因)
        画面.永続化.状態文言を更新する(`起動時読込失敗: ${メッセージ}`, true)
    }
    画面.表示を更新する(
        状態.楽曲を取得する(),
        状態.選択中パターンの名乗り,
        UI状態.進行の外モードか,
        UI状態.ドラッグ見込み,
    )
}
