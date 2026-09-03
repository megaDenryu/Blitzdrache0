import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 下書きと正本の揃い } from '../../../編集モデル/index.ts'
import { 数値入力項目 } from '../共通/数値入力項目.ts'
import { 選択中の等高線欄, type 選択中の等高線情報 } from './選択中の等高線欄.ts'
import { 揃いの表示 } from './揃いの表示.ts'
import { パネル, 見出し, ボタン区画, アクションボタン, 注意文 } from './スタイル.css.ts'

export interface I等高線パネル配線 {
    readonly on選んだ線の高さ確定: (高さメートル: number) => void
    readonly on選んだ線を削除する: () => void
    readonly on新しい線の高さ確定: (高さメートル: number) => void
    readonly on導く間隔確定: (間隔メートル: number) => void
    readonly on高さ場を生成: () => void
    readonly on高さ場から導く: () => void
}

// 膜の解の性質は描いてみないと分からないため、案内として常に出す(設計正本の判断3)。
const 平坦になる案内 = '閉じた線の内側に他の線が無いと、内側はその線の高さで平らになる。丘の頂を作るには内側へ高い線か頂点1つの線を置く。'

// 右サイドバーの等高線の設定。選んだ線の高さ・新しく描く線の高さ・導く間隔と、高さ場の生成と逆変換のボタン、
// 下書きと正本の揃いを収める(設計正本の判断7)。数値は入力が決まったときだけコマンドになる。
export class 等高線パネル extends LV2HtmlComponentBase implements I配線可能<I等高線パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I等高線パネル配線> = new 配線ポート<I等高線パネル配線>('等高線パネル')
    private readonly _選択中の欄: 選択中の等高線欄 = new 選択中の等高線欄()
    private readonly _新しい線の高さ: 数値入力項目
    private readonly _導く間隔: 数値入力項目
    private readonly _揃い: 揃いの表示 = new 揃いの表示()

    public constructor(初期の新しい線の高さ: number, 初期の導く間隔: number) {
        super()
        this._新しい線の高さ = new 数値入力項目('新しく描く線の高さ', 初期の新しい線の高さ, 0.5, 'm')
        this._導く間隔 = new 数値入力項目('高さ場から導く間隔', 初期の導く間隔, 0.5, 'm')
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I等高線パネル配線): this {
        this._配線.配線する(配線)
        this._選択中の欄.配線する({
            on高さ確定: (v) => this._配線.先.on選んだ線の高さ確定(v),
            on削除: () => this._配線.先.on選んだ線を削除する(),
        })
        this._新しい線の高さ.配線する({ on確定: (v) => this._配線.先.on新しい線の高さ確定(v) })
        this._導く間隔.配線する({ on確定: (v) => this._配線.先.on導く間隔確定(v) })
        return this
    }

    public 表示を更新する(選択中: 選択中の等高線情報 | null, 揃い: 下書きと正本の揃い): void {
        this._選択中の欄.表示を更新する(選択中)
        this._揃い.揃いを更新する(揃い)
    }

    public override delete(): void {
        this._選択中の欄.delete()
        this._新しい線の高さ.delete()
        this._導く間隔.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '等高線' }).setTooltip('等高線'),
                this._選択中の欄,
                this._新しい線の高さ,
                span({ class: 注意文, text: 平坦になる案内 }),
                this._導く間隔,
                div({ class: ボタン区画 }).childs([
                    button({ class: アクションボタン, text: '等高線から高さ場を生成する' })
                        .setTooltip('等高線を拘束にして高さ格子を作り直す。外周の格子点は変わらない。')
                        .onClick(() => this._配線.先.on高さ場を生成()),
                    button({ class: アクションボタン, text: '高さ場から等高線を導く' })
                        .setTooltip('いまの高さ場から上の間隔で等高線を導き、下書きの等高線を置き換える。')
                        .onClick(() => this._配線.先.on高さ場から導く()),
                    this._揃い])])
        )
    }
}
