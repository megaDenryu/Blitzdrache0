import { div, span, textInput, DivC, TextInputC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲 } from '../../../../../生成/編集資源契約.ts'
import { 行コンテナ, 項目ラベル, テキスト入力 } from '../共通/スタイル.css.ts'

export interface I曲基本設定配線 {
    readonly on表示名変更: (新しい表示名: string) => void
}

// 楽曲の表示名を入力する部品。
// 拍毎分は演奏の操作帯が持つ。同じ値を2箇所で変えられる形にしないため、この部品からは外してある。
export class 曲基本設定部品 extends LV2HtmlComponentBase implements I配線可能<I曲基本設定配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I曲基本設定配線> = new 配線ポート<I曲基本設定配線>('曲基本設定部品')
    private readonly _表示名入力: TextInputC

    public constructor(初期楽曲: 楽曲) {
        super()
        this._表示名入力 = textInput({ class: テキスト入力, value: 初期楽曲.表示名, placeholder: '楽曲の表示名' })
            .setTooltip('楽曲の表示名')
        this._componentRoot = div().child(
            div({ class: 行コンテナ }).childs([
                span({ class: 項目ラベル, text: '楽曲の表示名' }),
                this._表示名入力,
            ]),
        )
    }

    public 配線する(配線: I曲基本設定配線): this {
        this._配線.配線する(配線)
        this._表示名入力.onInput(() => {
            if (this._配線.配線済みか) {
                this._配線.先.on表示名変更(this._表示名入力.getValue())
            }
        })
        return this
    }

    public 表示を更新する(楽曲: 楽曲): void {
        if (this._表示名入力.getValue() !== 楽曲.表示名) this._表示名入力.setValue(楽曲.表示名)
    }

    public override delete(): void {
        this._表示名入力.delete()
        super.delete()
    }
}
