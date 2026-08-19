import { div, span, DivC, SpanC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import { コンテナ, セクション見出し, 木項目, 子木項目コンテナ, 子木項目, アイコン, フォルダアイコン } from './スタイル.css.ts'

export interface Iエクスプローラー配線 {
    readonly on大域世界を開く: () => void
    readonly onチャンクを開く: (座標: チャンク座標) => void
    readonly on使い方を開く: () => void
}

// 大域世界と全チャンクの木構造を左サイドバーに表示し開く操作を仲介するLV2素部品。
export class エクスプローラーパネル extends LV2HtmlComponentBase implements I配線可能<Iエクスプローラー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iエクスプローラー配線> = new 配線ポート<Iエクスプローラー配線>('エクスプローラーパネル')
    private readonly _大域世界ノード: DivC
    private readonly _チャンク親ノード: DivC
    private readonly _チャンク展開アイコン: SpanC
    private readonly _チャンク子項目コンテナ: DivC
    private readonly _チャンクノードマップ: Map<string, DivC> = new Map<string, DivC>()
    private readonly _使い方ノード: DivC
    private _チャンク展開中: boolean = true

    public constructor(軸あたりチャンク数: number = 4) {
        super()
        this._大域世界ノード = div({ class: 木項目 }).childs([
            span({ class: アイコン, text: 'G' }),
            span({ text: '大域世界' }).setTooltip('大域世界'),
        ]).onClick(() => {
            this._配線.先.on大域世界を開く()
        })

        const チャンク子項目一覧: DivC[] = []
        for (let z = 0; z < 軸あたりチャンク数; z++) {
            for (let x = 0; x < 軸あたりチャンク数; x++) {
                const 座標: チャンク座標 = { x, z }
                const キー = `${x},${z}`
                const チャンクラベル = `チャンク (${x}, ${z})`
                const 項目 = div({ class: 子木項目 }).childs([
                    span({ class: アイコン, text: 'C' }),
                    span({ text: チャンクラベル }).setTooltip(チャンクラベル),
                ]).onClick(() => {
                    this._配線.先.onチャンクを開く(座標)
                })
                this._チャンクノードマップ.set(キー, 項目)
                チャンク子項目一覧.push(項目)
            }
        }

        this._チャンク展開アイコン = span({ class: フォルダアイコン, text: '▼' })
        this._チャンク子項目コンテナ = div({ class: 子木項目コンテナ }).childs(チャンク子項目一覧)

        const チャンク親ラベル = `チャンク (${軸あたりチャンク数}×${軸あたりチャンク数})`
        this._チャンク親ノード = div({ class: 木項目 }).childs([
            this._チャンク展開アイコン,
            span({ text: チャンク親ラベル }).setTooltip(チャンク親ラベル),
        ]).onClick(() => {
            this._チャンク展開中 = !this._チャンク展開中
            this._チャンク展開アイコン.setTextContent(this._チャンク展開中 ? '▼' : '▶')
            this._チャンク子項目コンテナ.setStyleCSS({ display: this._チャンク展開中 ? 'flex' : 'none' })
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
            this._チャンク親ノード,
            this._チャンク子項目コンテナ,
            div({ class: セクション見出し, text: 'ヘルプ' }),
            this._使い方ノード,
        ])
    }

    public 配線する(配線: Iエクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 大域世界を選択表示する(): void {
        this._大域世界ノード.setAttribute('data-selected', 'true')
        this._使い方ノード.setAttribute('data-selected', 'false')
        for (const ノード of this._チャンクノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }

    public チャンクを選択表示する(座標: チャンク座標): void {
        this._大域世界ノード.setAttribute('data-selected', 'false')
        this._使い方ノード.setAttribute('data-selected', 'false')
        if (!this._チャンク展開中) {
            this._チャンク展開中 = true
            this._チャンク展開アイコン.setTextContent('▼')
            this._チャンク子項目コンテナ.setStyleCSS({ display: 'flex' })
        }
        const 対象キー = `${座標.x},${座標.z}`
        for (const [キー, ノード] of this._チャンクノードマップ.entries()) {
            ノード.setAttribute('data-selected', キー === 対象キー ? 'true' : 'false')
        }
    }

    public 使い方を選択表示する(): void {
        this._大域世界ノード.setAttribute('data-selected', 'false')
        this._使い方ノード.setAttribute('data-selected', 'true')
        for (const ノード of this._チャンクノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }
}
