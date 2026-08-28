import { 入力欄がフォーカス中か } from '../チャンク編集/画面/三次元/カメラ/キー入力ガード.ts'

// 鍵盤から取り消しを受ける相手の規約。建物編集同期サービスが実装する。
export interface I取り消せる側 {
    取り消す(): void
}

// Ctrl+Zによる取り消しを配線する。楽曲エディターと同じ割り当てにするのは、同じ種類の編集をする
// 道具どうしで操作が違うと、人が道具ごとに学び直すことになるためである。
// 参照: `~/.claude/skills/エディター制作` 第7条「操作の割り当ては、編集の種類ごとに1つに決める」
export function 建物編集キーボード入力を配線する(操作: I取り消せる側): () => void {
    const キー押下処理 = (事象: KeyboardEvent): void => {
        if (!(事象.ctrlKey || 事象.metaKey) || 事象.key.toLowerCase() !== 'z') return
        if (入力欄がフォーカス中か()) return
        事象.preventDefault()
        操作.取り消す()
    }

    window.addEventListener('keydown', キー押下処理)
    return () => {
        window.removeEventListener('keydown', キー押下処理)
    }
}
