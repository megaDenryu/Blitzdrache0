import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import { コンテナ, セクション見出し, 木項目, アイコン } from './スタイル.css.ts'
import { チャンク木 } from './エクスプローラーパネル/チャンク木.ts'

export interface Iエクスプローラー配線 {
    readonly on大域世界を開く: () => void
    readonly onチャンクを開く: (座標: チャンク座標) => void
    readonly onマテリアルを開く: () => void
    readonly on使い方を開く: () => void
}

// 大域世界と全チャンクの木構造を左サイドバーに表示し開く操作を仲介するLV2素部品。
// チャンクの展開/選択表示はエクスプローラーパネル/チャンク木.ts が持つ。
export class エクスプローラーパネル extends LV2HtmlComponentBase implements I配線可能<Iエクスプローラー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iエクスプローラー配線> = new 配線ポート<Iエクスプローラー配線>('エクスプローラーパネル')
    private readonly _大域世界ノード: DivC
    private readonly _チャンク木: チャンク木
    private readonly _マテリアルノード: DivC
    private readonly _使い方ノード: DivC

    public constructor(軸あたりチャンク数: number = 4) {
        super()
        this._大域世界ノード = div({ class: 木項目 }).childs([
            span({ class: アイコン, text: 'G' }),
            span({ text: '大域世界' }).setTooltip('大域世界'),
        ]).onClick(() => {
            this._配線.先.on大域世界を開く()
        })

        this._チャンク木 = new チャンク木(軸あたりチャンク数, (座標) => this._配線.先.onチャンクを開く(座標))

        this._マテリアルノード = div({ class: 木項目 }).childs([
            span({ class: アイコン, text: 'M' }),
            span({ text: 'マテリアル' }).setTooltip('マテリアル'),
        ]).onClick(() => {
            this._配線.先.onマテリアルを開く()
        })

        this._使い方ノード = div({ class: 木項目 }).childs([
            span({ class: アイコン, text: '?' }),
            span({ text: '使い方' }).setTooltip('使い方'),
        ]).onClick(() => {
            this._配線.先.on使い方を開く()
        })

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: 'プロジェクト' }),
            this._大域世界ノード,
            ...this._チャンク木.ルート要素一覧,
            this._マテリアルノード,
            div({ class: セクション見出し, text: 'ヘルプ' }),
            this._使い方ノード,
        ])
    }

    public 配線する(配線: Iエクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 大域世界を選択表示する(): void {
        this._選択表示を解除する()
        this._大域世界ノード.setAttribute('data-selected', 'true')
    }

    public チャンクを選択表示する(座標: チャンク座標): void {
        this._選択表示を解除する()
        this._チャンク木.選択表示する(座標)
    }

    public マテリアルを選択表示する(): void {
        this._選択表示を解除する()
        this._マテリアルノード.setAttribute('data-selected', 'true')
    }

    public 使い方を選択表示する(): void {
        this._選択表示を解除する()
        this._使い方ノード.setAttribute('data-selected', 'true')
    }

    private _選択表示を解除する(): void {
        this._大域世界ノード.setAttribute('data-selected', 'false')
        this._マテリアルノード.setAttribute('data-selected', 'false')
        this._使い方ノード.setAttribute('data-selected', 'false')
        this._チャンク木.選択解除する()
    }
}
