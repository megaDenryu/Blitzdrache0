import { div, span, button, DivC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import * as styles from './スタイル.css.ts'

export interface I散布パネル配線 {
    readonly on最小間隔変更: (最小間隔: number) => void
    readonly on散布再ベイク: () => void
}

// 植生等の散布最小間隔設定および再ベイク操作を提供するLV2素部品。
export class 散布パネル extends LV2HtmlComponentBase implements I配線可能<I散布パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I散布パネル配線> = new 配線ポート<I散布パネル配線>('散布パネル')
    private readonly _本数表示: SpanC
    private readonly _間隔スライダー: スライダー項目

    public constructor(初期間隔: number) {
        super()
        this._本数表示 = span({ class: styles.本数ラベル, text: '0 本' })
        this._間隔スライダー = new スライダー項目('最小間隔', 2.0, 15.0, 0.5, 初期間隔, 'm')

        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I散布パネル配線): this {
        this._配線.配線する(配線)
        this._間隔スライダー.配線する({ on値変更: (v) => this._配線.先.on最小間隔変更(v) })
        return this
    }

    public 本数を更新する(本数: number): void {
        this._本数表示.setTextContent(`${本数} 本`)
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: styles.パネル }).childs([
                div({ class: styles.見出し行 }).childs([
                    span({ text: '植生散布 (ポアソンディスク)' }),
                    this._本数表示]),
                this._間隔スライダー,
                button({ class: styles.再ベイクボタン, text: '散布再ベイク' })
                    .onClick(() => this._配線.先.on散布再ベイク())])
        )
    }
}
