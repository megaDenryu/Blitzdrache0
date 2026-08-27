import type { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲編集画面 } from './画面/index.ts'

// 編集の状態と画面の表示を合わせる工程を1箇所へ集める。
// 操作コマンドの適用後・パターンの選び直し・再生でパターンが移ったときの、どこから来ても同じ1つの経路にする。
export class 楽曲編集の表示の同期 {
    public constructor(
        private readonly _画面: 楽曲編集画面,
        private readonly _状態: 楽曲編集状態,
        private readonly _UI状態: 楽曲編集UI状態,
    ) {}

    public 再構築する(): void {
        this._画面.表示を更新する(
            this._状態.楽曲を取得する(),
            this._状態.選択中パターンの名乗り,
            this._UI状態.進行の外モードか,
            this._UI状態.ドラッグ見込み,
        )
    }
}
