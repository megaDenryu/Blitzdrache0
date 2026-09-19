import { カード位置は同じか, type カード位置, type 曲構成のカード, type 演奏の範囲 } from '../../編集モデル/index.ts'
import type { 再生位置 } from '../演奏/index.ts'
import type { カード部品 } from './カード部品.ts'
import { 再生中のカードを求める } from './再生中のカードを求める.ts'

// タイムラインの再生中の印と繰り返し中の印を、触れる添字2つだけに閉じて管理する
// (再生中は画面の1コマごとに呼ばれるため、変わったカードだけ触る規律をここへ閉じる。タイムライン部品からの工程分離)。
export class タイムラインの再生印 {
    private _再生中の添字: number | null = null
    private _選択中の添字: number | null = null
    private _繰り返し中か: boolean = false

    // 表示の再構築のたびに呼び、選択中の添字だけを引き継いで印の状態を初期化する。
    public リセットする(選択中の添字: number | null): void {
        this._再生中の添字 = null
        this._繰り返し中か = false
        this._選択中の添字 = 選択中の添字
    }

    public 再生中のカード位置を求める(
        カード列: readonly 曲構成のカード[],
        位置: 再生位置 | null,
        範囲: 演奏の範囲,
    ): カード位置 | null {
        return 再生中のカードを求める(カード列, 位置, 範囲)
    }

    public 示す(カード部品一覧: readonly カード部品[], 再生中のカード: カード位置 | null, 繰り返し中か: boolean): void {
        const 見つかった添字 = 再生中のカード === null
            ? -1
            : カード部品一覧.findIndex((カード) => カード位置は同じか(カード.位置, 再生中のカード))
        const 新しい添字 = 見つかった添字 === -1 ? null : 見つかった添字
        if (新しい添字 !== this._再生中の添字) {
            if (this._再生中の添字 !== null) カード部品一覧[this._再生中の添字]?.再生中の印を示す(false)
            if (新しい添字 !== null) カード部品一覧[新しい添字]?.再生中の印を示す(true)
            this._再生中の添字 = 新しい添字
        }
        if (繰り返し中か !== this._繰り返し中か) {
            this._繰り返し中か = 繰り返し中か
            if (this._選択中の添字 !== null) カード部品一覧[this._選択中の添字]?.繰り返し中の印を示す(繰り返し中か)
        }
    }
}
