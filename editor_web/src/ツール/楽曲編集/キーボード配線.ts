import { 入力欄がフォーカス中か } from '../チャンク編集/画面/三次元/カメラ/キー入力ガード.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'

// Ctrl+Zによる取り消しおよびAltキーによる進行の外モード切り替えを配線する。
export function 楽曲編集キーボード入力を配線する(
    UI状態: 楽曲編集UI状態,
    操作: 楽曲履歴適用サービス,
    同期: () => void,
): () => void {
    const キー押下処理 = (e: KeyboardEvent): void => {
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'z') {
            if (入力欄がフォーカス中か()) return
            e.preventDefault()
            操作.直前の操作を取り消す()
            return
        }
        if (e.key === 'Alt' && !e.repeat) {
            if (入力欄がフォーカス中か()) return
            e.preventDefault()
            UI状態.進行の外モードか = !UI状態.進行の外モードか
            同期()
        }
    }

    window.addEventListener('keydown', キー押下処理)
    return () => {
        window.removeEventListener('keydown', キー押下処理)
    }
}
