import { div, span, button, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 地表材質層 } from '../../../../../生成/編集資源契約.ts'
import { スライダー項目 } from '../共通/スライダー項目.ts'
import { パネル, 見出し, 材質グリッド, 材質ボタン, ベイク区画, アクションボタン } from './スタイル.css.ts'

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

class 材質選択ボタン extends ButtonC {
    public constructor(ラベル: string, 選択中: boolean) {
        super({ class: 材質ボタン, text: ラベル })
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        this.setTooltip(ラベル)
    }

    public 選択状態を設定する(選択中: boolean): this {
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        return this
    }
}

// 地表材質ペイントの材質選択・ブラシ半径・流量および自動ベイクを提供するLV2素部品。
export class 地表ペイントパネル extends LV2HtmlComponentBase implements I配線可能<I地表ペイントパネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I地表ペイントパネル配線> = new 配線ポート<I地表ペイントパネル配線>('地表ペイントパネル')
    private readonly _ボタンマップ: Map<地表材質層, 材質選択ボタン> = new Map<地表材質層, 材質選択ボタン>()
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
            ボタン.選択状態を設定する(層 === 選択層)
        }
    }

    public override delete(): void {
        this._半径スライダー.delete()
        this._流量スライダー.delete()
        super.delete()
    }

    private _ルートを構築する(初期層: 地表材質層): DivC {
        return (
            div({ class: パネル }).childs([
                span({ class: 見出し, text: '地表マテリアルペイント' }).setTooltip('地表マテリアルペイント'),
                div({ class: 材質グリッド }).childs(
                    材質層一覧.map(({ 層, ラベル }) => {
                        const btn = new 材質選択ボタン(ラベル, 層 === 初期層)
                        this._ボタンマップ.set(層, btn)
                        return btn.onClick(() => {
                            this.材質層を更新する(層)
                            this._配線.先.on材質層変更(層)
                        })
                    }),
                ),
                this._半径スライダー,
                this._流量スライダー,
                div({ class: ベイク区画 }).childs([
                    button({ class: アクションボタン, text: '急勾配(>30度)を自動で岩肌にベイク' })
                        .setTooltip('急勾配(>30度)を自動で岩肌にベイク')
                        .onClick(() => this._配線.先.on急勾配ベイク()),
                    button({ class: アクションボタン, text: '道路下を自動で泥にベイク' })
                        .setTooltip('道路下を自動で泥にベイク')
                        .onClick(() => this._配線.先.on道路下泥ベイク())])])
        )
    }
}
