import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 記号ボタン } from '../パネル/共通/スタイル.css.ts'
import type { カード部品 } from './カード部品.ts'
import type { 節移動の押せるか } from './節移動の押せるかを計算する.ts'
import type { 節移動の種類 } from './節移動の種類.ts'
import { 節の枠, 節の枠見出し, 節の枠見出し文言, 節の枠見出し操作, 節の枠カード列 } from './スタイル.css.ts'

export interface I節の枠配線 {
    readonly on移動: (種類: 節移動の種類) => void
}

// 同じ節に属するカードを括る枠。見出しにパターンの表示名と繰り返し回数を出し、
// 節を単位にした前へ移動・後へ移動のボタンを持つ(カードの移動ボタンはここへ移した、issue #88)。
// 保持する情報は節の位置と見出しの文言だけに閉じ、中身のカードの列は外から受け取る
// (issue #87で育つパターンの枠を見据え、いま持つ情報を最小に保つ)。
export class 節の枠部品 extends LV2HtmlComponentBase implements I配線可能<I節の枠配線> {
    protected _componentRoot: DivC
    public readonly 節の位置: number
    private readonly _配線: 配線ポート<I節の枠配線> = new 配線ポート<I節の枠配線>('節の枠部品')
    private readonly _前へボタン: ButtonC
    private readonly _後へボタン: ButtonC

    public constructor(
        節の位置: number,
        見出しの文言: string,
        カード部品列: readonly カード部品[],
        押せるか: 節移動の押せるか,
    ) {
        super()
        this.節の位置 = 節の位置
        this._前へボタン = button({ class: 記号ボタン, text: '←' })
            .setTooltip('節を1つ前へ移動')
            .setDisabled(!押せるか.前へ移動)
        this._後へボタン = button({ class: 記号ボタン, text: '→' })
            .setTooltip('節を1つ後へ移動')
            .setDisabled(!押せるか.後へ移動)
        const 見出し = div({ class: 節の枠見出し }).childs([
            span({ class: 節の枠見出し文言, text: 見出しの文言 }),
            div({ class: 節の枠見出し操作 }).childs([this._前へボタン, this._後へボタン]),
        ])
        this._componentRoot = div({ class: 節の枠 }).childs([
            見出し,
            div({ class: 節の枠カード列 }).childs([...カード部品列]),
        ])
    }

    public 配線する(配線: I節の枠配線): this {
        this._配線.配線する(配線)
        this._前へボタン.onClick((event) => {
            event.stopPropagation()
            if (this._配線.配線済みか) this._配線.先.on移動('前へ移動')
        })
        this._後へボタン.onClick((event) => {
            event.stopPropagation()
            if (this._配線.配線済みか) this._配線.先.on移動('後へ移動')
        })
        return this
    }

    public override delete(): void {
        this._前へボタン.delete()
        this._後へボタン.delete()
        super.delete()
    }
}
