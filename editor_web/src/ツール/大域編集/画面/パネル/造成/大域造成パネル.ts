import { div, span, DivC, ButtonC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 造成筆致種別 } from '../../../../../生成/編集資源契約.ts'
import { スライダー項目 } from '../../../../チャンク編集/画面/パネル/共通/スライダー項目.ts'
import { パネル, 見出し行, 補助テキスト, ボタングループ, ブラシ種別ボタン } from './スタイル.css.ts'

export interface I大域造成パネル配線 {
    readonly on筆致種別変更: (種別: 造成筆致種別) => void
    readonly on半径変更: (半径: number) => void
    readonly on強さ変更: (強さ: number) => void
}

const 筆致種別一覧: readonly { readonly 種別: 造成筆致種別; readonly ラベル: string }[] = [
    { 種別: '盛る', ラベル: '山脈造成' },
    { 種別: '平す', ラベル: '平滑化' },
    { 種別: '水平にする', ラベル: '水平化' },
]

class 筆致選択ボタン extends ButtonC {
    public constructor(ラベル: string, 選択中: boolean) {
        super({ class: ブラシ種別ボタン, text: ラベル })
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        this.setTooltip(ラベル)
    }

    public 選択状態を設定する(選択中: boolean): this {
        this.setAttribute('data-selected', 選択中 ? 'true' : 'false')
        return this
    }
}

// 1024m大域造成用ブラシの種類・半径・強度を設定するLV2素部品。
export class 大域造成パネル extends LV2HtmlComponentBase implements I配線可能<I大域造成パネル配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I大域造成パネル配線> = new 配線ポート<I大域造成パネル配線>('大域造成パネル')
    private readonly _ボタンマップ: Map<造成筆致種別, 筆致選択ボタン> = new Map<造成筆致種別, 筆致選択ボタン>()
    private readonly _半径スライダー: スライダー項目
    private readonly _強さスライダー: スライダー項目

    public constructor(初期種別: 造成筆致種別, 初期半径: number, 初期強さ: number) {
        super()
        this._半径スライダー = new スライダー項目('大域ブラシ半径', 20, 250, 5, 初期半径, 'm')
        this._強さスライダー = new スライダー項目('造成強度', 0.1, 5.0, 0.1, 初期強さ)
        this._componentRoot = this._ルートを構築する(初期種別)
    }

    public 配線する(配線: I大域造成パネル配線): this {
        this._配線.配線する(配線)
        this._半径スライダー.配線する({ on値変更: (v: number) => this._配線.先.on半径変更(v) })
        this._強さスライダー.配線する({ on値変更: (v: number) => this._配線.先.on強さ変更(v) })
        return this
    }

    public 筆致種別を更新する(選択種別: 造成筆致種別): void {
        for (const [種別, ボタン] of this._ボタンマップ.entries()) {
            ボタン.選択状態を設定する(種別 === 選択種別)
        }
    }

    public override delete(): void {
        this._半径スライダー.delete()
        this._強さスライダー.delete()
        super.delete()
    }

    private _ルートを構築する(初期種別: 造成筆致種別): DivC {
        return (
            div({ class: パネル }).childs([
                div({ class: 見出し行 }).childs([
                    span({ text: 'マクロ山脈・盆地造成ブラシ' }).setTooltip('マクロ山脈・盆地造成ブラシ'),
                    span({ class: 補助テキスト, text: '[シフト: 削り / 平滑化]' }).setTooltip('[シフト: 削り / 平滑化]'),
                ]),
                div({ class: ボタングループ }).childs(
                    筆致種別一覧.map(({ 種別, ラベル }) => {
                        const btn = new 筆致選択ボタン(ラベル, 種別 === 初期種別)
                        this._ボタンマップ.set(種別, btn)
                        return btn.onClick(() => {
                            this.筆致種別を更新する(種別)
                            this._配線.先.on筆致種別変更(種別)
                        })
                    }),
                ),
                this._半径スライダー,
                this._強さスライダー,
            ])
        )
    }
}
