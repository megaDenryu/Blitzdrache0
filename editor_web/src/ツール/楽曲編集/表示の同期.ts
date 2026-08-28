import type { 配線ポート } from 'sengen-ui'
import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲編集画面 } from './画面/index.ts'

// 楽曲の表示名が変わったことを受け取る、ツールの外側の相手の規約。文書タブの見出しを追随させるために使う。
export interface I楽曲の表示名の届け先 {
    楽曲の表示名が変わった(新しい表示名: string): void
}

// 編集の状態と画面の表示を合わせる工程を1箇所へ集める。
// 操作コマンドの適用後・パターンの選び直し・再生でパターンが移ったときの、どこから来ても同じ1つの経路にする。
// 楽曲の表示名は中央の欄だけでなく文書タブの見出しにも出ているため、ここが変化を外側へも知らせる。
export class 楽曲編集の表示の同期 {
    private _直前に知らせた表示名: string | null = null

    public constructor(
        private readonly _画面: 楽曲編集画面,
        private readonly _状態: 楽曲編集状態,
        private readonly _UI状態: 楽曲編集UI状態,
        private readonly _表示名の届け先: 配線ポート<I楽曲の表示名の届け先>,
    ) {}

    public 再構築する(): void {
        const 楽曲 = this._状態.楽曲を取得する()
        this._画面.表示を更新する(
            楽曲,
            this._状態.選択中パターンの名乗り,
            this._UI状態.進行の外モードか,
            this._UI状態.ドラッグ見込み,
        )
        this._表示名の変化を知らせる(楽曲.表示名)
    }

    // 打点のたびに見出しを組み直させないため、前に知らせた名前と違うときだけ届ける。
    private _表示名の変化を知らせる(表示名: string): void {
        if (表示名 === this._直前に知らせた表示名) return
        this._直前に知らせた表示名 = 表示名
        if (this._表示名の届け先.配線済みか) this._表示名の届け先.先.楽曲の表示名が変わった(表示名)
    }
}
