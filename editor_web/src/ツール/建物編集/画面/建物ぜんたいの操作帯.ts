import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 横並び, 選択ボタン } from './スタイル.css.ts'

export interface I建物ぜんたいの操作帯配線 {
    readonly on取り消す: () => void
    readonly onやり直す: () => void
    readonly on識別色の重ねを切り替える: () => void
    readonly on建物ぜんたいを写す: () => void
}

// 建物ぜんたいに効く操作(取り消し・やり直し・表示の切替)を1行へ並べる帯。
// エディタ領域の上部へ固定して置くのは、これらが編集の対象と一緒にスクロールして消えると、
// いま何をしているかが分からなくなるためである(設計正本の判断14)。
// 数が増えない操作なので、縦の幅を取らずに建物名の欄と同じ1行へ収める。
export class 建物ぜんたいの操作帯 extends LV2HtmlComponentBase implements I配線可能<I建物ぜんたいの操作帯配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物ぜんたいの操作帯配線> = new 配線ポート<I建物ぜんたいの操作帯配線>('建物ぜんたいの操作帯')
    private readonly _取り消しボタン: DivC
    private readonly _やり直しボタン: DivC
    private readonly _識別色の重ねボタン: DivC

    public constructor() {
        super()
        this._取り消しボタン = div({ class: 選択ボタン, text: '取り消し' })
            .setTooltip('直前の操作を取り消す(Ctrl+Z)')
            .onClick(() => this._配線.先.on取り消す())
        this._やり直しボタン = div({ class: 選択ボタン, text: 'やり直し' })
            .setTooltip('取り消した操作をやり直す')
            .onClick(() => this._配線.先.onやり直す())
        this._識別色の重ねボタン = div({ class: 選択ボタン, text: '役割の識別色を重ねる' })
            .setTooltip('部品の役割ごとの色を、建物の形の上へ重ねて示す')
            .onClick(() => this._配線.先.on識別色の重ねを切り替える())
        this._componentRoot = div({ class: 横並び }).childs([
            this._取り消しボタン,
            this._やり直しボタン,
            this._識別色の重ねボタン,
            div({ class: 選択ボタン, text: '建物ぜんたいを写す' })
                .setTooltip('視点を建物ぜんたいが収まる位置へ戻す')
                .onClick(() => this._配線.先.on建物ぜんたいを写す()),
        ])
    }

    public 配線する(配線: I建物ぜんたいの操作帯配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 表示を更新する(取り消せるか: boolean, やり直せるか: boolean, 識別色を重ねるか: boolean): void {
        this._取り消しボタン.setStyleCSS({ opacity: 取り消せるか ? '1' : '0.4' })
        this._やり直しボタン.setStyleCSS({ opacity: やり直せるか ? '1' : '0.4' })
        this._識別色の重ねボタン.setAttribute('data-selected', String(識別色を重ねるか))
    }
}
