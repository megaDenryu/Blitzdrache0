import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { カード部品 } from './カード部品.ts'
import { 節の操作ボタン群 } from './節の操作ボタン群.ts'
import type { 節の操作の押せるか } from './節の操作の押せるかを計算する.ts'
import type { 節の操作の種類 } from './節の操作の種類.ts'
import { 節の枠, 節の枠見出し, 節の枠見出し文言, 節の枠見出し操作, 節の枠カード列 } from './スタイル.css.ts'

export interface I節の枠配線 {
    readonly on操作: (種類: 節の操作の種類) => void
}

// 同じ節に属するカードを括る枠。見出しにパターンの表示名と繰り返し回数を出し、
// 削除・複製・前後挿入・前後移動の6つの操作をすべてここへ集める(カードは選択だけを受け持つ、issue #87)。
// 保持する情報は節の位置と見出しの文言だけに閉じ、中身のカードの列は外から受け取る
// (issue #87で育つパターンの枠を見据え、いま持つ情報を最小に保つ)。
export class 節の枠部品 extends LV2HtmlComponentBase implements I配線可能<I節の枠配線> {
    protected _componentRoot: DivC
    public readonly 節の位置: number
    private readonly _配線: 配線ポート<I節の枠配線> = new 配線ポート<I節の枠配線>('節の枠部品')
    private readonly _操作ボタン群: 節の操作ボタン群

    public constructor(
        節の位置: number,
        見出しの文言: string,
        カード部品列: readonly カード部品[],
        押せるか: 節の操作の押せるか,
    ) {
        super()
        this.節の位置 = 節の位置
        this._操作ボタン群 = new 節の操作ボタン群(押せるか)
        const 見出し = div({ class: 節の枠見出し }).childs([
            span({ class: 節の枠見出し文言, text: 見出しの文言 }),
            div({ class: 節の枠見出し操作 }).childs([this._操作ボタン群]),
        ])
        this._componentRoot = div({ class: 節の枠 }).childs([
            見出し,
            div({ class: 節の枠カード列 }).childs([...カード部品列]),
        ])
    }

    public 配線する(配線: I節の枠配線): this {
        this._配線.配線する(配線)
        this._操作ボタン群.配線する({
            on操作: (種類) => {
                if (this._配線.配線済みか) this._配線.先.on操作(種類)
            },
        })
        return this
    }

    public override delete(): void {
        this._操作ボタン群.delete()
        super.delete()
    }
}
