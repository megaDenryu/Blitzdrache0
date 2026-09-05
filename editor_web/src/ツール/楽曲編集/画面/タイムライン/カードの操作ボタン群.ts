import { div, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 記号ボタン, 危険な記号ボタン } from '../パネル/共通/スタイル.css.ts'
import { 操作ボタン行 } from './スタイル.css.ts'
import type { カード操作の種類 } from './カード操作の種類.ts'

export interface Iカードの操作ボタン群配線 {
    readonly on操作: (種類: カード操作の種類) => void
}

// 押せるかどうかを操作の種類ごとに問える形。呼び出し側(タイムライン部品)が
// `カードの操作をコマンドへ写す`の結果から組み立てる。
export type カード操作の押せるか = { readonly [K in カード操作の種類]: boolean }

const ボタンの並び: ReadonlyArray<{ readonly 種類: カード操作の種類; readonly 記号: string; readonly 説明: string }> = [
    { 種類: '前へ移動', 記号: '←', 説明: '1つ前へ移動' },
    { 種類: '前へ挿入', 記号: '⊢', 説明: '同じパターンを前へ挿入' },
    { 種類: '複製', 記号: '⧉', 説明: '複製(繰り返し回数を1増やす)' },
    { 種類: '削除', 記号: '×', 説明: '削除' },
    { 種類: '後へ挿入', 記号: '⊣', 説明: '同じパターンを後へ挿入' },
    { 種類: '後へ移動', 記号: '→', 説明: '1つ後へ移動' },
]

// 選択中のカードにだけ出す、6つの操作の小さなボタンの列。
export class カードの操作ボタン群 extends LV2HtmlComponentBase implements I配線可能<Iカードの操作ボタン群配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iカードの操作ボタン群配線> =
        new 配線ポート<Iカードの操作ボタン群配線>('カードの操作ボタン群')
    private readonly _ボタン一覧: ReadonlyMap<カード操作の種類, ButtonC>

    public constructor(押せるか: カード操作の押せるか) {
        super()
        const ボタン一覧 = new Map<カード操作の種類, ButtonC>()
        for (const { 種類, 記号, 説明 } of ボタンの並び) {
            const クラス = 種類 === '削除' ? 危険な記号ボタン : 記号ボタン
            const ボタン = button({ class: クラス, text: 記号 })
                .setTooltip(説明)
                .setDisabled(!押せるか[種類])
            ボタン一覧.set(種類, ボタン)
        }
        this._ボタン一覧 = ボタン一覧
        this._componentRoot = div({ class: 操作ボタン行 }).childs([...this._ボタン一覧.values()])
    }

    public 配線する(配線: Iカードの操作ボタン群配線): this {
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
