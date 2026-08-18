import { div, span, button, DivC, SpanC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 建物種別 } from '../../../../../生成/編集資源契約.ts'
import * as styles from './スタイル.css.ts'

export interface I建物パネル配線 {
    readonly on建物生成: (種別: 建物種別) => void
    readonly on基礎平坦化: () => void
    readonly on地面接地: () => void
    readonly on建物削除: () => void
}

// 建物の新規配置・基礎平坦化・接地・削除操作を提供するLV2素部品。
export class 建物パネル extends LV2HtmlComponentBase implements I配線可能<I建物パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I建物パネル配線> = new 配線ポート<I建物パネル配線>('建物パネル')
    private readonly _件数表示: SpanC
    private readonly _削除ボタン: ButtonC

    public constructor() {
        super()
        this._件数表示 = span({ class: styles.件数ラベル, text: '0 件' })
        this._削除ボタン = button({ class: styles.削除ボタン, text: '削除', disabled: true })

        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I建物パネル配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 件数を更新する(件数: number): void {
        this._件数表示.setTextContent(`${件数} 件`)
    }

    public 選択建物有効状態を設定する(選択あり: boolean): void {
        this._削除ボタン.setDisabled(!選択あり)
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: styles.パネル }).childs([
                div({ class: styles.見出し行 }).childs([
                    span({ text: '建物 / POI 配置' }),
                    this._件数表示]),
                div({ class: styles.生成ボタングリッド }).childs([
                    button({ class: styles.生成ボタン, text: '+ 家屋 (12m)' })
                        .onClick(() => this._配線.先.on建物生成('家屋')),
                    button({ class: styles.生成ボタン, text: '+ 塔 (8m)' })
                        .onClick(() => this._配線.先.on建物生成('塔')),
                    button({ class: styles.生成ボタン, text: '+ 宝箱 (2m)' })
                        .onClick(() => this._配線.先.on建物生成('宝箱'))]),
                div({ class: styles.アクション区画 }).childs([
                    button({ class: styles.平坦化ボタン, text: '選択建物の基礎に合わせて地形造成' })
                        .onClick(() => this._配線.先.on基礎平坦化()),
                    div({ class: styles.行ボタン群 }).childs([
                        button({ class: styles.接地ボタン, text: '地面に接地' })
                            .onClick(() => this._配線.先.on地面接地()),
                        this._削除ボタン
                            .onClick(() => this._配線.先.on建物削除())])])])
        )
    }
}
