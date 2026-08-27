import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 曲の節, パターン } from '../../../../../生成/編集資源契約.ts'
import { 記号ボタン, 危険な記号ボタン } from '../共通/スタイル.css.ts'
import { 曲の節を上へ移動できるか, 曲の節を下へ移動できるか } from './曲構成表示計算.ts'
import { 節のパターン選択欄 } from './節のパターン選択欄.ts'
import { 節の繰り返し回数選択欄 } from './節の繰り返し回数選択欄.ts'
import { 節行枠, 節番号, 操作ボタン群 } from './スタイル.css.ts'

export interface I曲構成行配線 {
    readonly on節変更: (新しいパターンの名乗り: string, 新しい繰り返し回数: number) => void
    readonly on上へ移動: () => void
    readonly on下へ移動: () => void
    readonly on削除: () => void
}

// 曲構成の1つの節（対象パターン・繰り返し回数・並べ替え・削除）の編集行。
export class 曲構成行部品 extends LV2HtmlComponentBase implements I配線可能<I曲構成行配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲構成行配線> = new 配線ポート<I曲構成行配線>('曲構成行部品')
    private readonly _パターン選択: 節のパターン選択欄
    private readonly _回数選択: 節の繰り返し回数選択欄
    private readonly _上へボタン: ButtonC
    private readonly _下へボタン: ButtonC
    private readonly _削除ボタン: ButtonC

    public constructor(節: 曲の節, 位置: number, 全節数: number, パターン一覧: readonly パターン[]) {
        super()
        this._パターン選択 = new 節のパターン選択欄(節, パターン一覧)
        this._回数選択 = new 節の繰り返し回数選択欄(節)
        this._上へボタン = button({ class: 記号ボタン, text: '↑' })
            .setTooltip('上へ移動')
            .setDisabled(!曲の節を上へ移動できるか(位置))
        this._下へボタン = button({ class: 記号ボタン, text: '↓' })
            .setTooltip('下へ移動')
            .setDisabled(!曲の節を下へ移動できるか(位置, 全節数))
        this._削除ボタン = button({ class: 危険な記号ボタン, text: '×' }).setTooltip('この節を削除')
        this._componentRoot = this._ルートを構築する(位置)
    }

    public 配線する(配線: I曲構成行配線): this {
        this._配線.配線する(配線)
        this._パターン選択.onSelectChange(() => this._選び直された節を伝える())
        this._回数選択.onSelectChange(() => this._選び直された節を伝える())
        this._上へボタン.onClick(() => {
            if (this._配線.配線済みか) this._配線.先.on上へ移動()
        })
        this._下へボタン.onClick(() => {
            if (this._配線.配線済みか) this._配線.先.on下へ移動()
        })
        this._削除ボタン.onClick(() => {
            if (this._配線.配線済みか) this._配線.先.on削除()
        })
        return this
    }

    public override delete(): void {
        this._パターン選択.delete()
        this._回数選択.delete()
        this._上へボタン.delete()
        this._下へボタン.delete()
        this._削除ボタン.delete()
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

    private _ルートを構築する(位置: number): DivC {
        return div({ class: 節行枠 }).childs([
            span({ class: 節番号, text: `#${位置 + 1}` }),
            this._パターン選択,
            this._回数選択,
            div({ class: 操作ボタン群 }).childs([this._上へボタン, this._下へボタン, this._削除ボタン]),
        ])
    }
}
