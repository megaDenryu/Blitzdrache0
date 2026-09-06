import { div, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 副ボタン, 危険ボタン } from '../パネル/共通/スタイル.css.ts'
import { 操作ボタン行 } from './スタイル.css.ts'
import type { 節の操作の押せるか } from './節の操作の押せるかを計算する.ts'
import type { 節の操作の種類 } from './節の操作の種類.ts'

export interface I節の操作ボタン群配線 {
    readonly on操作: (種類: 節の操作の種類) => void
}

const ボタンの並び: ReadonlyArray<{ readonly 種類: 節の操作の種類; readonly 文字: string; readonly 説明: string }> = [
    { 種類: '前へ移動', 文字: '前へ', 説明: '節を1つ前へ移動' },
    { 種類: '前へ挿入', 文字: '前に挿入', 説明: '同じパターンを前へ挿入' },
    { 種類: '複製', 文字: '複製', 説明: '複製(繰り返し回数を1増やす)' },
    { 種類: '削除', 文字: '削除', 説明: '削除' },
    { 種類: '後へ挿入', 文字: '後に挿入', 説明: '同じパターンを後へ挿入' },
    { 種類: '後へ移動', 文字: '後へ', 説明: '節を1つ後へ移動' },
]

// 節の枠に出す6つの操作のボタンの列。削除・複製・前後挿入・前後移動をすべてここへ集め、
// カード自身はボタンを持たない(判断16の是正、issue #87)。記号だけでは人が操作を
// 見つけられなかったため、ボタンは文字で表す(issue #92)。
export class 節の操作ボタン群 extends LV2HtmlComponentBase implements I配線可能<I節の操作ボタン群配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I節の操作ボタン群配線> = new 配線ポート<I節の操作ボタン群配線>('節の操作ボタン群')
    private readonly _ボタン一覧: ReadonlyMap<節の操作の種類, ButtonC>

    public constructor(押せるか: 節の操作の押せるか) {
        super()
        const ボタン一覧 = new Map<節の操作の種類, ButtonC>()
        for (const { 種類, 文字, 説明 } of ボタンの並び) {
            const クラス = 種類 === '削除' ? 危険ボタン : 副ボタン
            const ボタン = button({ class: クラス, text: 文字 })
                .setTooltip(説明)
                .setDisabled(!押せるか[種類])
            ボタン一覧.set(種類, ボタン)
        }
        this._ボタン一覧 = ボタン一覧
        this._componentRoot = div({ class: 操作ボタン行 }).childs([...this._ボタン一覧.values()])
    }

    public 配線する(配線: I節の操作ボタン群配線): this {
        this._配線.配線する(配線)
        for (const [種類, ボタン] of this._ボタン一覧) {
            ボタン.onClick((event) => {
                event.stopPropagation()
                if (this._配線.配線済みか) this._配線.先.on操作(種類)
            })
        }
        return this
    }

    public override delete(): void {
        for (const ボタン of this._ボタン一覧.values()) ボタン.delete()
        super.delete()
    }
}
