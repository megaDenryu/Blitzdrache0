import { 入力欄がフォーカス中か } from '../チャンク編集/画面/三次元/カメラ/キー入力ガード.ts'
import type { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'

// 鍵盤の空白から再生と停止を切り替えられる相手の規約。演奏サービスが実装する。
export interface I再生を切り替えられる側 {
    再生と停止を切り替える(): void
}

// 空白キーは押せるものの上で「押す」の意味を持つため、それらの上では再生の切り替えに使わない。
const 空白キーを渡さないタグ名一覧 = new Set<string>(['INPUT', 'TEXTAREA', 'SELECT', 'BUTTON', 'OPTION'])

// 事象の届いた先のタグ名を、外から来た値として型を絞りながら読む。
function 押した先のタグ名(事象: KeyboardEvent): string | null {
    const 届いた先: unknown = 事象.target
    if (届いた先 === null || typeof 届いた先 !== 'object') return null
    if (!('tagName' in 届いた先)) return null
    const タグ名: unknown = 届いた先.tagName
    return typeof タグ名 === 'string' ? タグ名 : null
}

function 空白キーで再生を切り替えてよいか(事象: KeyboardEvent): boolean {
    if (入力欄がフォーカス中か()) return false
    const タグ名 = 押した先のタグ名(事象)
    return タグ名 === null || !空白キーを渡さないタグ名一覧.has(タグ名)
}

// 鍵盤の空白による再生と停止、Ctrl+Zによる取り消し、Altによる進行の外モードの切り替えを配線する。
export function 楽曲編集キーボード入力を配線する(
    UI状態: 楽曲編集UI状態,
    操作: 楽曲履歴適用サービス,
    同期: () => void,
    演奏: I再生を切り替えられる側,
): () => void {
    const キー押下処理 = (e: KeyboardEvent): void => {
        if (e.code === 'Space') {
            if (!空白キーで再生を切り替えてよいか(e)) return
            e.preventDefault()
            演奏.再生と停止を切り替える()
            return
        }
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
