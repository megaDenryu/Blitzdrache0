import { div, textInput, DivC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物定義ID } from '../../../境界/建物定義ID.ts'
import { 建物名の枠, 表示名の入力, 識別子の添え } from './スタイル.css.ts'

export interface I建物名の欄配線 {
    readonly on表示名変更: (新しい表示名: string) => void
}

// いま編集している建物の表示名を出し、その場で書き換える欄。
// 建物ぜんたいの操作帯と同じ行へ置いて名前のためだけの行を無くすため、エディタ領域の上部の
// 固定の行が持つ(設計正本の判断14)。同じ値を2箇所で変えられる形にしないため、
// 右サイドバーの設定側にはこの欄を置かない。
export class 建物名の欄 extends LV2HtmlComponentBase implements I配線可能<I建物名の欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物名の欄配線> = new 配線ポート<I建物名の欄配線>('建物名の欄')
    private readonly _表示名入力: TextInputC
    private readonly _識別子表示: DivC

    public constructor(建物定義ID: 建物定義ID) {
        super()
        this._表示名入力 = textInput({ class: 表示名の入力, value: '', placeholder: '建物の表示名' })
            .setTooltip('建物の表示名。ここで変えると文書タブの見出しも変わる')
        this._識別子表示 = div({ class: 識別子の添え, text: 建物定義ID })
            .setTooltip('保存先を決める識別子。変えられない')
        this._componentRoot = div({ class: 建物名の枠 }).childs([this._表示名入力, this._識別子表示])
    }

    public 配線する(配線: I建物名の欄配線): this {
        this._配線.配線する(配線)
        this._表示名入力.onInput(() => this._入力された())
        return this
    }

    public 表示を更新する(表示名: string): void {
        if (this._表示名入力.getValue() !== 表示名) this._表示名入力.setValue(表示名)
    }

    public override delete(): void {
        this._表示名入力.delete()
        this._識別子表示.delete()
        super.delete()
    }

    private _入力された(): void {
        if (this._配線.配線済みか) this._配線.先.on表示名変更(this._表示名入力.getValue())
    }
}
