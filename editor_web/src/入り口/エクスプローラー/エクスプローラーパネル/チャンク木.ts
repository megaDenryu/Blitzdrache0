import { div, span, DivC, SpanC } from 'sengen-ui'
import type { チャンク座標 } from '../../../境界/通信/index.ts'
import { 木項目, 子木項目コンテナ, 子木項目, アイコン, フォルダアイコン } from '../スタイル.css.ts'

// エクスプローラーの「チャンク (n×n)」親ノードと、その配下に並ぶ全チャンクの子項目群。
// 展開/折りたたみと選択表示の状態はこの型が触れるフィールドに閉じる。
export class チャンク木 {
    private readonly _親ノード: DivC
    private readonly _展開アイコン: SpanC
    private readonly _子項目コンテナ: DivC
    private readonly _ノードマップ: Map<string, DivC> = new Map<string, DivC>()
    private _展開中: boolean = true

    public constructor(軸あたりチャンク数: number, onチャンクを開く: (座標: チャンク座標) => void) {
        const 子項目一覧: DivC[] = []
        for (let z = 0; z < 軸あたりチャンク数; z++) {
            for (let x = 0; x < 軸あたりチャンク数; x++) {
                const 座標: チャンク座標 = { x, z }
                const キー = `${x},${z}`
                const ラベル = `チャンク (${x}, ${z})`
                const 項目 = div({ class: 子木項目 }).childs([
                    span({ class: アイコン, text: 'C' }),
                    span({ text: ラベル }).setTooltip(ラベル),
                ]).onClick(() => onチャンクを開く(座標))
                this._ノードマップ.set(キー, 項目)
                子項目一覧.push(項目)
            }
        }

        this._展開アイコン = span({ class: フォルダアイコン, text: '▼' })
        this._子項目コンテナ = div({ class: 子木項目コンテナ }).childs(子項目一覧)

        const 親ラベル = `チャンク (${軸あたりチャンク数}×${軸あたりチャンク数})`
        this._親ノード = div({ class: 木項目 }).childs([
            this._展開アイコン,
            span({ text: 親ラベル }).setTooltip(親ラベル),
        ]).onClick(() => this._展開を切り替える())
    }

    public get ルート要素一覧(): DivC[] {
        return [this._親ノード, this._子項目コンテナ]
    }

    public 選択表示する(座標: チャンク座標): void {
        if (!this._展開中) {
            this._展開を切り替える()
        }
        const 対象キー = `${座標.x},${座標.z}`
        for (const [キー, ノード] of this._ノードマップ.entries()) {
            ノード.setAttribute('data-selected', キー === 対象キー ? 'true' : 'false')
        }
    }

    public 選択解除する(): void {
        for (const ノード of this._ノードマップ.values()) {
            ノード.setAttribute('data-selected', 'false')
        }
    }

    private _展開を切り替える(): void {
        this._展開中 = !this._展開中
        this._展開アイコン.setTextContent(this._展開中 ? '▼' : '▶')
        this._子項目コンテナ.setStyleCSS({ display: this._展開中 ? 'flex' : 'none' })
    }
}
