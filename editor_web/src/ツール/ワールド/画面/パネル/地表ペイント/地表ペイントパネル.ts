import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 地表材質層 } from '../../../../../生成/編集資源契約.ts'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import * as styles from './スタイル.css.ts'

export interface I地表ペイントパネル配線 {
    readonly on材質層変更: (層: 地表材質層) => void
    readonly on半径変更: (半径: number) => void
    readonly on流量変更: (流量: number) => void
    readonly on急勾配ベイク: () => void
    readonly on道路下泥ベイク: () => void
}

const 材質層一覧: readonly { readonly 層: 地表材質層; readonly ラベル: string }[] = [
    { 層: '草', ラベル: '草地 (R)' },
    { 層: '泥', ラベル: '泥/土 (G)' },
    { 層: '岩', ラベル: '岩肌 (B)' },
    { 層: '砂', ラベル: '砂 (A)' },
]

// 地表材質ペイントの材質選択・ブラシ半径・流量および自動ベイクを提供するLV2素部品。
export class 地表ペイントパネル extends LV2HtmlComponentBase implements I配線可能<I地表ペイントパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I地表ペイントパネル配線> = new 配線ポート<I地表ペイントパネル配線>('地表ペイントパネル')
    private readonly _ボタンマップ: Map<地表材質層, ButtonC> = new Map<地表材質層, ButtonC>()
    private readonly _半径スライダー: スライダー項目
    private readonly _流量スライダー: スライダー項目

    public constructor(初期層: 地表材質層, 初期半径: number, 初期流量: number) {
        super()
        this._半径スライダー = new スライダー項目('ペイント半径', 3, 50, 1, 初期半径, 'm')
        this._流量スライダー = new スライダー項目('流量 (不透明度)', 0.05, 1.0, 0.05, 初期流量)
        this._componentRoot = this._ルートを構築する(初期層)
    }

    public 配線する(配線: I地表ペイントパネル配線): this {
        this._配線.配線する(配線)
        this._半径スライダー.配線する({ on値変更: (v) => this._配線.先.on半径変更(v) })
        this._流量スライダー.配線する({ on値変更: (v) => this._配線.先.on流量変更(v) })
        return this
    }

    public 材質層を更新する(選択層: 地表材質層): void {
        for (const [層, ボタン] of this._ボタンマップ.entries()) {
            ボタン.setAttribute('data-selected', 層 === 選択層 ? 'true' : 'false')
        }
    }

    private _ルートを構築する(初期層: 地表材質層): DivC {
        return (
            div({ class: styles.パネル }).childs([
                span({ class: styles.見出し, text: '地表マテリアルペイント' }),
                div({ class: styles.材質グリッド }).childs(
                    材質層一覧.map(({ 層, ラベル }) =>
                        button({
                            class: styles.材質ボタン,
                            text: ラベル,
                        })
                            .setAttribute('data-selected', 層 === 初期層 ? 'true' : 'false')
                            .tap((btn: ButtonC) => {
                                this._ボタンマップ.set(層, btn)
                            })
                            .onClick(() => {
                                this.材質層を更新する(層)
                                this._配線.先.on材質層変更(層)
                            }),
                    ),
                ),
                this._半径スライダー,
                this._流量スライダー,
                div({ class: styles.ベイク区画 }).childs([
                    button({ class: styles.アクションボタン, text: '急勾配(>30度)を自動で岩肌にベイク' })
                        .onClick(() => this._配線.先.on急勾配ベイク()),
                    button({ class: styles.アクションボタン, text: '道路下を自動で泥にベイク' })
                        .onClick(() => this._配線.先.on道路下泥ベイク())])])
        )
    }
}
