import { div, span, input, DivC, InputC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { つまみの綴りを数値として読む } from './つまみの値.ts'
import { 行コンテナ, ラベル行, 値ラベル, スライダー入力 } from './スタイル.css.ts'

export interface Iスライダー配線 {
    // つまみを動かしている最中に、値が変わるたびに呼ばれる。
    // 取り消しの履歴へ積む操作をここへ繋いではならない。1回のドラッグが数十件の履歴になるためである。
    readonly on値変更: (新値: number) => void
    // つまみから手を離して値が決まったときに1回だけ呼ばれる。
    // 取り消しの単位を人の操作へ揃えるため、コマンドを積む操作はこちらへ繋ぐ(設計正本の判断13)。
    readonly on値が決まった?: (決まった値: number) => void
}

// つまみを動かしている最中には何も伝えない、という意思をそのまま表す配線。
// 空の関数をそれぞれの呼び出し側へ書くと「書き忘れ」と見分けが付かないため、名前を与えて1箇所に置く。
export const 動かしている間は何も伝えない = (): void => {}

class スライダー値ラベル extends SpanC {
    private readonly _接尾辞: string

    public constructor(初期値: number, 接尾辞: string) {
        super({ class: 値ラベル, text: `${初期値}${接尾辞}` })
        this._接尾辞 = 接尾辞
    }

    public 値を更新する(新値: number): this {
        this.setTextContent(`${新値}${this._接尾辞}`)
        return this
    }
}

// 項目名・現在値ラベル・レンジ入力を一体化したLV2素部品。
export class スライダー項目 extends LV2HtmlComponentBase implements I配線可能<Iスライダー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iスライダー配線> = new 配線ポート<Iスライダー配線>('スライダー項目')
    private readonly _値表示: スライダー値ラベル
    private readonly _入力欄: InputC

    public constructor(
        ラベル名: string,
        最小値: number,
        最大値: number,
        刻み幅: number,
        初期値: number,
        接尾辞: string = '',
    ) {
        super()
        this._値表示 = new スライダー値ラベル(初期値, 接尾辞)
        this._入力欄 = input({ class: スライダー入力, type: 'range', value: 初期値.toString() })
            .setRangeParam({ min: 最小値, max: 最大値, step: 刻み幅 })
        this._componentRoot = this._ルートを構築する(ラベル名)
    }

    public 配線する(配線: Iスライダー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 値を更新する(新値: number): void {
        this._値表示.値を更新する(新値)
    }

    // 編集対象が切り替わったときに、つまみの位置と数値ラベルの両方をモデルの値へ合わせる。
    public 値を設定する(新値: number): void {
        this._入力欄.setValue(新値.toString())
        this._値表示.値を更新する(新値)
    }

    public 操作できるか設定する(操作できるか: boolean): void {
        this._入力欄.setDisabled(!操作できるか)
    }

    private _つまみが動いたときに値を伝える(): void {
        const 数値 = つまみの綴りを数値として読む(this._入力欄.getValue())
        this.値を更新する(数値)
        if (this._配線.配線済みか) {
            this._配線.先.on値変更(数値)
        }
    }

    private _つまみから手を離したときに値を伝える(): void {
        if (!this._配線.配線済みか) return
        const 決まった値を受ける側 = this._配線.先.on値が決まった
        if (決まった値を受ける側 === undefined) return
        決まった値を受ける側(つまみの綴りを数値として読む(this._入力欄.getValue()))
    }

    private _ルートを構築する(ラベル名: string): DivC {
        return div({ class: 行コンテナ }).childs([
            div({ class: ラベル行 }).childs([
                span({ text: ラベル名 }).setTooltip(ラベル名),
                this._値表示,
            ]),
            this._入力欄
                .onInput(() => this._つまみが動いたときに値を伝える())
                .onChange(() => this._つまみから手を離したときに値を伝える()),
        ])
    }
}
