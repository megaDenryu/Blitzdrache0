import { div, span, input, DivC, InputC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { 拍毎分の下限, 拍毎分の上限 } from '../../../../生成/編集資源契約.ts'
import { 拍毎分のつまみ, 拍毎分の欄 as 拍毎分の欄のスタイル, 項目の名前 } from './スタイル.css.ts'

export interface I拍毎分の欄配線 {
    readonly on拍毎分変更: (新しい拍毎分: number) => void
}

// つまみの隣に出す数の表示。つまみを動かした瞬間にここが変わるため、押しても変わらない操作にならない。
class 拍毎分の数の表示 extends SpanC {
    public constructor(初期値: number) {
        super({ class: 項目の名前, text: `${初期値} 拍毎分` })
    }

    public 値を反映する(値: number): this {
        this.setTextContent(`${値} 拍毎分`)
        return this
    }
}

// 拍毎分を動かす欄。曲の設定のパネルではなく操作帯に置き、同じ値を2箇所で変えられる形を作らない。
export class 拍毎分の欄 extends LV2HtmlComponentBase implements I配線可能<I拍毎分の欄配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I拍毎分の欄配線> = new 配線ポート<I拍毎分の欄配線>('拍毎分の欄')
    private readonly _数の表示: 拍毎分の数の表示
    private readonly _つまみ: InputC

    public constructor(初期の拍毎分: number) {
        super()
        this._数の表示 = new 拍毎分の数の表示(初期の拍毎分)
        this._つまみ = input({ class: 拍毎分のつまみ, type: 'range', value: String(初期の拍毎分) })
            .setRangeParam({ min: 拍毎分の下限, max: 拍毎分の上限, step: 1 })
            .setTooltip('1分あたりの拍の数')
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I拍毎分の欄配線): this {
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

    private _つまみが動いた(): void {
        const 数値 = Number.parseInt(this._つまみ.getValue(), 10)
        if (!Number.isFinite(数値)) {
            throw new Error(`拍毎分のつまみの値を数値として読めません: ${this._つまみ.getValue()}`)
        }
        this._数の表示.値を反映する(数値)
        if (this._配線.配線済みか) this._配線.先.on拍毎分変更(数値)
    }

    private _ルートを構築する(): DivC {
        return div({ class: 拍毎分の欄のスタイル }).childs([
            span({ class: 項目の名前, text: '拍毎分' }),
            this._つまみ.onInput(() => this._つまみが動いた()),
            this._数の表示,
        ])
    }
}
