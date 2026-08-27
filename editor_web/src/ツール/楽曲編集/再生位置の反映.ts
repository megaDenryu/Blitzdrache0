import type { I再生位置の届け先, 再生位置, 演奏サービス } from './画面/index.ts'
import type { 楽曲編集画面 } from './画面/index.ts'
import type { 楽曲編集状態 } from './編集モデル/index.ts'
import type { 楽曲編集の表示の同期 } from './表示の同期.ts'

// 音声の時計から導いた再生位置を画面へ映す。
// 曲構成のとおりに鳴らしているとき、鳴っているパターンが変わったら開いているパターンもそちらへ移す。
export class 再生位置の反映 implements I再生位置の届け先 {
    public constructor(
        private readonly _画面: 楽曲編集画面,
        private readonly _状態: 楽曲編集状態,
        private readonly _演奏: 演奏サービス,
        private readonly _同期: 楽曲編集の表示の同期,
    ) {}

    public 再生位置が変わった(位置: 再生位置 | null): void {
        if (this._鳴っているパターンへ移すべきか(位置)) {
            this._状態.選択中パターンの名乗り = 位置 === null ? null : 位置.パターンの名乗り
            this._同期.再構築する()
        }
        this._画面.再生位置を示す(位置, this._演奏.再生中か, this._演奏.演奏の範囲)
    }

    private _鳴っているパターンへ移すべきか(位置: 再生位置 | null): boolean {
        if (位置 === null) return false
        if (位置.パターンの名乗り === this._状態.選択中パターンの名乗り) return false
        return this._状態.パターンが存在するか(位置.パターンの名乗り)
    }
}
