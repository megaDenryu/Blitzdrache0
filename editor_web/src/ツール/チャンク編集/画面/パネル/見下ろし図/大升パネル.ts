import { div, span, button, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 地表材質層 } from '../../../../../生成/編集資源契約.ts'
import type { 下書きと正本の揃い } from '../../../編集モデル/index.ts'
import { 数値入力項目 } from '../共通/数値入力項目.ts'
import { 大升の塗りの設定, type I大升の塗りの設定配線, type 大升の塗りの初期設定 } from './大升の塗りの設定.ts'
import { 選択中の大升欄, type 選択中の大升情報 } from './選択中の大升欄.ts'
import { 揃いの表示 } from './揃いの表示.ts'
import { パネル, 見出し, ボタン区画, アクションボタン, 注意文 } from './スタイル.css.ts'

export interface I大升パネル配線 extends I大升の塗りの設定配線 {
    readonly on一辺の升目数確定: (升目数: number) => void
    readonly on選択高さ確定: (高さメートル: number | null) => void
    readonly on選択層確定: (層: 地表材質層 | null) => void
    readonly on選択を消す: () => void
    readonly on地形を生成: () => void
    readonly on高さ場から導く: () => void
}

// 一辺を変えると塗りの意味が変わるため塗りは空になる(設計正本の判断4)。取り返しがつかないため欄のすぐ下に出す。
const 一辺の注意 = '一辺を変えると、いまの大升の塗りは空になる。値は格子の解像度の升目数を割り切る数だけ受け付ける(既定8)。'

// 右サイドバーの大升の設定。大升の一辺・置く高さと層・塗りを消す切替と、地形の生成と逆変換のボタン、
// 下書きと正本の揃いを収める(設計正本の判断7)。
export class 大升パネル extends LV2HtmlComponentBase implements I配線可能<I大升パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I大升パネル配線> = new 配線ポート<I大升パネル配線>('大升パネル')
    private readonly _選択中の欄: 選択中の大升欄 = new 選択中の大升欄()
    private readonly _一辺: 数値入力項目
    private readonly _塗りの設定: 大升の塗りの設定
    private readonly _揃い: 揃いの表示 = new 揃いの表示()

    public constructor(初期の一辺の升目数: number, 初期の塗り: 大升の塗りの初期設定) {
        super()
        this._一辺 = new 数値入力項目('大升の一辺の升目数', 初期の一辺の升目数, 1)
        this._塗りの設定 = new 大升の塗りの設定(初期の塗り)
        this._componentRoot = this._ルートを構築する()
    }

    public 配線する(配線: I大升パネル配線): this {
        this._配線.配線する(配線)
        this._選択中の欄.配線する({
            on高さ確定: (v) => this._配線.先.on選択高さ確定(v),
            on層確定: (層) => this._配線.先.on選択層確定(層),
            on消す: () => this._配線.先.on選択を消す(),
        })
        this._一辺.配線する({ on確定: (v) => this._配線.先.on一辺の升目数確定(v) })
        this._塗りの設定.配線する(配線)
        return this
    }

    // 一辺の欄は「次に塗るときの一辺」であり人が打った値を保つため、ここでは書き換えない。
    public 表示を更新する(選択中: 選択中の大升情報 | null, 揃い: 下書きと正本の揃い): void {
        this._選択中の欄.表示を更新する(選択中)
        this._揃い.揃いを更新する(揃い)
    }

    public override delete(): void {
        this._選択中の欄.delete()
        this._一辺.delete()
        this._塗りの設定.delete()
        super.delete()
    }

    private _ルートを構築する(): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '大升' }).setTooltip('大升'),
                this._選択中の欄,
                this._一辺,
                span({ class: 注意文, text: 一辺の注意 }),
                this._塗りの設定,
                div({ class: ボタン区画 }).childs([
                    button({ class: アクションボタン, text: '大升から地形を生成する' })
                        .setTooltip('塗った大升の高さと層から高さ格子と材質重みを作り直す。塗っていない大升は変わらない。')
                        .onClick(() => this._配線.先.on地形を生成()),
                    button({ class: アクションボタン, text: '高さ場から大升を導く' })
                        .setTooltip('いまの高さ場と材質重みから大升の塗りを導き、下書きの塗りを置き換える。')
                        .onClick(() => this._配線.先.on高さ場から導く()),
                    this._揃い])])
        )
    }
}
