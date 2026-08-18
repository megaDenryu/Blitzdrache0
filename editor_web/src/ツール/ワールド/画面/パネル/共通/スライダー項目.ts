import { div, span, input, DivC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import * as styles from './スタイル.css.ts'

export interface Iスライダー配線 {
    readonly on値変更: (新値: number) => void
}

// 項目名・現在値ラベル・レンジ入力を一体化したLV2素部品。
export class スライダー項目 extends LV2HtmlComponentBase implements I配線可能<Iスライダー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iスライダー配線> = new 配線ポート<Iスライダー配線>('スライダー項目')
    private readonly _値表示: SpanC
    private readonly _接尾辞: string

    public constructor(
        ラベル名: string,
        最小値: number,
        最大値: number,
        刻み幅: number,
        初期値: number,
        接尾辞: string = '',
    ) {
        super()
        this._接尾辞 = 接尾辞
        this._値表示 = span({ class: styles.値ラベル, text: `${初期値}${接尾辞}` })

        this._componentRoot = this._ルートを構築する(ラベル名, 最小値, 最大値, 刻み幅, 初期値)
    }

    public 配線する(配線: Iスライダー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 値を更新する(新値: number): void {
        this._値表示.setTextContent(`${新値}${this._接尾辞}`)
    }

    private _ルートを構築する(
        ラベル名: string,
        最小値: number,
        最大値: number,
        刻み幅: number,
        初期値: number,
    ): DivC {
        return (
            div({ class: styles.行コンテナ }).childs([
                div({ class: styles.ラベル行 }).childs([
                    span({ text: ラベル名 }),
                    this._値表示]),
                input({
                    class: styles.スライダー入力,
                    type: 'range',
                    value: 初期値.toString(),
                })
                    .setRangeParam({ min: 最小値, max: 最大値, step: 刻み幅 })
                    .onInput((e: Event) => {
                        const 要素 = e.target
                        if (要素 instanceof HTMLInputElement) {
                            const 数値 = parseFloat(要素.value)
                            if (Number.isFinite(数値)) {
                                this.値を更新する(数値)
                                this._配線.先.on値変更(数値)
                            }
                        }
                    })])
        )
    }
}
