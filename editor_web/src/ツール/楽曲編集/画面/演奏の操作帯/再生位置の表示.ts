import { SpanC } from 'sengen-ui'
import type { 再生位置 } from '../演奏/index.ts'
import { 位置の表示 } from './スタイル.css.ts'

// いま何ステップ目を鳴らしているかを数で見せる。値は音声の時計から導いた再生位置であり、画面の時計では進めない。
export class 再生位置の表示 extends SpanC {
    private _映している文言: string = '停止中'

    public constructor() {
        super({ class: 位置の表示, text: '停止中' })
    }

    // 再生中は画面の1コマごとに呼ばれるため、文言が変わるときだけ書き換える。
    public 再生位置を反映する(位置: 再生位置 | null, パターンの表示名: string | null): this {
        const 文言 = 再生位置の表示._文言を組み立てる(位置, パターンの表示名)
        if (this._映している文言 === 文言) return this
        this._映している文言 = 文言
        this.setTextContent(文言)
        return this
    }

    private static _文言を組み立てる(位置: 再生位置 | null, パターンの表示名: string | null): string {
        if (位置 === null) return '停止中'
        const 名前 = パターンの表示名 !== null ? パターンの表示名 : 位置.パターンの名乗り
        return `${名前} / ${位置.パターン内ステップ + 1} ステップ目`
    }
}
