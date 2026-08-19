import { div, span, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import { コンテナ, セクション見出し, 木項目, 子木項目, アイコン, フォルダアイコン } from './スタイル.css.ts'

export interface Iエクスプローラー配線 {
    readonly on大域世界を開く: () => void
    readonly onチャンクを開く: (座標: チャンク座標) => void
}

// 大域世界と全チャンクの木構造を左サイドバーに表示し開く操作を仲介するLV2素部品。
export class エクスプローラーパネル extends LV2HtmlComponentBase implements I配線可能<Iエクスプローラー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iエクスプローラー配線> = new 配線ポート<Iエクスプローラー配線>('エクスプローラーパネル')
    private readonly _大域世界ノード: DivC
    private readonly _チャンクノードマップ: Map<string, DivC> = new Map<string, DivC>()

    public constructor(軸あたりチャンク数: number = 4) {
        super()
        this._大域世界ノード = div({ class: 木項目 }).childs([
            span({ class: アイコン, text: 'G' }),
            span({ text: '大域世界' }),
        ]).onClick(() => {
            this._配線.先.on大域世界を開く()
        })

        const チャンク子項目一覧: DivC[] = []
        for (let z = 0; z < 軸あたりチャンク数; z++) {
            for (let x = 0; x < 軸あたりチャンク数; x++) {
                const 座標: チャンク座標 = { x, z }
                const キー = `${x},${z}`
                const 項目 = div({ class: 子木項目 }).childs([
                    span({ class: アイコン, text: 'C' }),
                    span({ text: `チャンク (${x}, ${z})` }),
                ]).onClick(() => {
                    this._配線.先.onチャンクを開く(座標)
                })
                this._チャンクノードマップ.set(キー, 項目)
                チャンク子項目一覧.push(項目)
            }
        }

        const チャンク親ノード = div({ class: 木項目 }).childs([
            span({ class: フォルダアイコン, text: '▼' }),
            span({ text: 'チャンク' }),
        ])

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: 'プロジェクト' }),
            this._大域世界ノード,
            div({ class: セクション見出し, text: '区画' }),
            チャンク親ノード,
            ...チャンク子項目一覧,
        ])
    }

    public 配線する(配線: Iエクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 大域世界を選択表示する(): void {
        this._大域世界ノード.setAttribute('data-selected', 'true')
        for (const ノード of this._チャンクノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }

    public チャンクを選択表示する(座標: チャンク座標): void {
        this._大域世界ノード.setAttribute('data-selected', 'false')
        const 対象キー = `${座標.x},${座標.z}`
        for (const [キー, ノード] of this._チャンクノードマップ.entries()) {
            ノード.setAttribute('data-selected', キー === 対象キー ? 'true' : 'false')
        }
    }
}
