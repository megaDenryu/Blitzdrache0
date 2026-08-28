import { div, span, input, DivC, InputC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { テンポの下限, テンポの上限 } from '../../../../生成/編集資源契約.ts'
import { つまみの綴りを数値として読む } from '../パネル/共通/つまみの値.ts'
import { テンポのつまみ, テンポの欄 as テンポの欄のスタイル, 項目の名前 } from './スタイル.css.ts'

export interface Iテンポの欄配線 {
    readonly onテンポ変更: (新しいテンポ: number) => void
}

// つまみの隣に出す数の表示。つまみを動かした瞬間にここが変わるため、押しても変わらない操作にならない。
// 単位は隣の見出しとつまみの吹き出しが担うため、ここは数だけを出す。
class テンポの数の表示 extends SpanC {
    public constructor(初期値: number) {
        super({ class: 項目の名前, text: String(初期値) })
    }

    public 値を反映する(値: number): this {
        this.setTextContent(String(値))
        return this
    }
}

// テンポを動かす欄。曲の設定のパネルではなく操作帯に置き、同じ値を2箇所で変えられる形を作らない。
// つまみを動かしている間は数だけが動き、手を離したときに1つだけコマンドを積む(設計正本の判断13)。
export class テンポの欄 extends LV2HtmlComponentBase implements I配線可能<Iテンポの欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iテンポの欄配線> = new 配線ポート<Iテンポの欄配線>('テンポの欄')
    private readonly _数の表示: テンポの数の表示
    private readonly _つまみ: InputC

    public constructor(初期のテンポ: number) {
        super()
        this._数の表示 = new テンポの数の表示(初期のテンポ)
        this._つまみ = input({ class: テンポのつまみ, type: 'range', value: String(初期のテンポ) })
            .setRangeParam({ min: テンポの下限, max: テンポの上限, step: 1 })
            .setTooltip('テンポ(1分あたりの拍数)')
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: Iテンポの欄配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 値を設定する(値: number): void {
        this._つまみ.setValue(String(値))
        this._数の表示.値を反映する(値)
    }

    public override delete(): void {
        this._数の表示.delete()
        this._つまみ.delete()
        super.delete()
    }

    // 動かしている間は数の表示だけを追わせる。取り消しの履歴を1回のドラッグで埋めないためである。
    private _つまみが動いた(): void {
        this._数の表示.値を反映する(this._読み取ったテンポ())
    }

    private _つまみから手を離した(): void {
        const 数値 = this._読み取ったテンポ()
        this._数の表示.値を反映する(数値)
        if (this._配線.配線済みか) this._配線.先.onテンポ変更(数値)
    }

    private _読み取ったテンポ(): number {
        return Math.round(つまみの綴りを数値として読む(this._つまみ.getValue()))
    }

    private _ルートを構築する(): DivC {
        return div({ class: テンポの欄のスタイル }).childs([
            span({ class: 項目の名前, text: 'テンポ' }),
            this._つまみ
                .onInput(() => this._つまみが動いた())
                .onChange(() => this._つまみから手を離した()),
            this._数の表示,
        ])
    }
}
