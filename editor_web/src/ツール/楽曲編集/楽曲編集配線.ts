import type { 楽曲ID } from '../../境界/index.ts'
import type { 楽曲接続 } from '../../境界/通信/index.ts'
import { パネル操作を実行する, type パネル操作結果 } from '../チャンク編集/永続化パネル操作.ts'
import { 楽曲編集キーボード入力を配線する } from './キーボード配線.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { 楽曲編集ポインタ振り分け } from './楽曲編集ポインタ振り分け.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲編集画面 } from './画面/index.ts'

// 楽曲編集画面のイベント（ポインタ打ち込み・キーボード・永続化）を配線する。
export function 楽曲編集イベントを配線する(
    画面: 楽曲編集画面,
    状態: 楽曲編集状態,
    UI状態: 楽曲編集UI状態,
    操作: 楽曲履歴適用サービス,
    接続: 楽曲接続,
    楽曲ID: 楽曲ID,
): () => void {
    const 表示を再構築する = (): void => {
        画面.表示を更新する(
            状態.楽曲を取得する(),
            状態.選択中パターンの名乗り,
            UI状態.進行の外モードか,
            UI状態.ドラッグ見込み,
        )
    }

    const 発音配線 = 画面.発音配線.配線済みか ? 画面.発音配線.先 : undefined
    const 振り分け = new 楽曲編集ポインタ振り分け(
        状態,
        UI状態,
        操作,
        表示を再構築する,
        発音配線,
    )

    画面.升目操作を配線する(
        (当たり, ボタン) => { 振り分け.押された(ボタン, 当たり) },
        (当たり) => { 振り分け.動かされた(当たり) },
    )

    const ポインタ解放処理 = (e: PointerEvent): void => {
        振り分け.離された(e.button)
    }
    const ポインタキャンセル処理 = (): void => {
        振り分け.キャンセルされた()
    }
    window.addEventListener('pointerup', ポインタ解放処理)
    window.addEventListener('pointercancel', ポインタキャンセル処理)

    const キー解除 = 楽曲編集キーボード入力を配線する(UI状態, 操作, 表示を再構築する)

    const 保存する = async (): Promise<パネル操作結果> => {
        const 結果 = await 接続.楽曲を保存する(状態.楽曲を取得する())
        if (結果.種別 === '失敗') {
            return { 文言: `保存失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, エラーか: true }
        }
        return { 文言: '保存完了', エラーか: false }
    }

    const 読み込む = async (): Promise<パネル操作結果> => {
        const 結果 = await 接続.楽曲を読む(楽曲ID)
        if (結果.種別 === '失敗') {
            return { 文言: `読込失敗: ${結果.エラー.種別} ${結果.エラー.説明}`, エラーか: true }
        }
        if (結果.種別 === '成功') 状態.状態を上書きする(結果.値)
        表示を再構築する()
        return {
            文言: 結果.種別 === '成功' ? '読み込み完了' : '未保存: 初期楽曲を表示中',
            エラーか: false,
        }
    }

    画面.永続化.on保存クリック(() => {
        void パネル操作を実行する(画面.永続化, '保存中...', 保存する, '保存失敗')
    })
    画面.永続化.on読込クリック(() => {
        void パネル操作を実行する(画面.永続化, '読み込み中...', 読み込む, '読込失敗')
    })

    表示を再構築する()
    return () => {
        window.removeEventListener('pointerup', ポインタ解放処理)
        window.removeEventListener('pointercancel', ポインタキャンセル処理)
        キー解除()
    }
}
