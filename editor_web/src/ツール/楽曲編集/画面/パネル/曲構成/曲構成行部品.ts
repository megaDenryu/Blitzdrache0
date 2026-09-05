import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 曲の節, パターン } from '../../../../../生成/編集資源契約.ts'
import { 節のパターン選択欄 } from './節のパターン選択欄.ts'
import { 節の繰り返し回数選択欄 } from './節の繰り返し回数選択欄.ts'
import { 節行枠, 節番号 } from './スタイル.css.ts'

export interface I曲構成行配線 {
    readonly on節変更: (新しいパターンの名乗り: string, 新しい繰り返し回数: number) => void
}

// 曲構成の1つの節(対象パターン・繰り返し回数)の編集行。
// 追加・削除・並べ替えはタイムラインが受け持つため、この行は節の詳細だけを持つ(設計正本の判断15)。
export class 曲構成行部品 extends LV2HtmlComponentBase implements I配線可能<I曲構成行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲構成行配線> = new 配線ポート<I曲構成行配線>('曲構成行部品')
    private readonly _パターン選択: 節のパターン選択欄
    private readonly _回数選択: 節の繰り返し回数選択欄

    public constructor(節: 曲の節, 位置: number, パターン一覧: readonly パターン[]) {
        super()
        this._パターン選択 = new 節のパターン選択欄(節, パターン一覧)
        this._回数選択 = new 節の繰り返し回数選択欄(節)
        this._componentRoot = div({ class: 節行枠 }).childs([
            span({ class: 節番号, text: `#${位置 + 1}` }),
            this._パターン選択,
            this._回数選択,
        ])
    }

    public 配線する(配線: I曲構成行配線): this {
        this._配線.配線する(配線)
        this._パターン選択.onSelectChange(() => this._選び直された節を伝える())
        this._回数選択.onSelectChange(() => this._選び直された節を伝える())
        return this
    }

    public override delete(): void {
        this._パターン選択.delete()
        this._回数選択.delete()
        super.delete()
    }

    private _選び直された節を伝える(): void {
        if (this._配線.配線済みか) {
            this._配線.先.on節変更(
                this._パターン選択.選ばれたパターンの名乗り(),
                this._回数選択.選ばれた繰り返し回数(),
            )
        }
    }
}
