import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標, 建物の格子の一覧項目, 建物定義ID, 楽曲ID } from '../../境界/通信/index.ts'
import { コンテナ, セクション見出し } from './スタイル.css.ts'
import { 単一項目ノードを作る } from './エクスプローラーパネル/単一項目ノード.ts'
import { チャンク木 } from './エクスプローラーパネル/チャンク木.ts'
import { 建物木 } from './エクスプローラーパネル/建物木.ts'
import { 楽曲木 } from './エクスプローラーパネル/楽曲木.ts'

export interface Iエクスプローラー配線 {
    readonly on大域世界を開く: () => void
    readonly onチャンクを開く: (座標: チャンク座標) => void
    readonly onマテリアルを開く: () => void
    readonly on建物を開く: (建物定義ID: 建物定義ID, 表示名: string) => void
    readonly on建物を作る: () => void
    readonly on楽曲を開く: (楽曲ID: 楽曲ID, 表示名: string) => void
    readonly on楽曲を作る: () => void
    readonly on使い方を開く: () => void
}


// 大域世界と全チャンク・建物・楽曲の木構造を左サイドバーに表示し開く操作を仲介するLV2素部品。
export class エクスプローラーパネル extends LV2HtmlComponentBase implements I配線可能<Iエクスプローラー配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iエクスプローラー配線> = new 配線ポート<Iエクスプローラー配線>('エクスプローラーパネル')
    private readonly _大域世界ノード: DivC
    private readonly _チャンク木: チャンク木
    private readonly _建物木: 建物木
    private readonly _楽曲木: 楽曲木
    private readonly _マテリアルノード: DivC
    private readonly _使い方ノード: DivC

    public constructor(軸あたりチャンク数: number = 4) {
        super()
        this._大域世界ノード = 単一項目ノードを作る('G', '大域世界', () => this._配線.先.on大域世界を開く())

        this._チャンク木 = new チャンク木(軸あたりチャンク数, (座標) => this._配線.先.onチャンクを開く(座標))
        this._建物木 = new 建物木(
            (建物定義ID, 表示名) => this._配線.先.on建物を開く(建物定義ID, 表示名),
            () => this._配線.先.on建物を作る(),
        )
        this._楽曲木 = new 楽曲木(
            (楽曲ID, 表示名) => this._配線.先.on楽曲を開く(楽曲ID, 表示名),
            () => this._配線.先.on楽曲を作る(),
        )

        this._マテリアルノード = 単一項目ノードを作る('M', 'マテリアル', () => this._配線.先.onマテリアルを開く())
        this._使い方ノード = 単一項目ノードを作る('?', '使い方', () => this._配線.先.on使い方を開く())

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: 'プロジェクト' }),
            this._大域世界ノード,
            ...this._チャンク木.ルート要素一覧,
            this._マテリアルノード,
            div({ class: セクション見出し, text: '建物' }),
            ...this._建物木.ルート要素一覧,
            div({ class: セクション見出し, text: '楽曲' }),
            ...this._楽曲木.ルート要素一覧,
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

    public 建物の一覧を作り直す(一覧: readonly 建物の格子の一覧項目[]): void {
        this._建物木.一覧を作り直す(一覧)
    }

    public 建物を選択表示する(建物定義ID: 建物定義ID): void {
        this._選択表示を解除する()
        this._建物木.選択表示する(建物定義ID)
    }

    public 楽曲の一覧を作り直す(一覧: readonly 楽曲ID[]): void {
        this._楽曲木.一覧を作り直す(一覧)
    }

    public 楽曲を選択表示する(楽曲ID: 楽曲ID): void {
        this._選択表示を解除する()
        this._楽曲木.選択表示する(楽曲ID)
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
        this._建物木.選択解除する()
        this._楽曲木.選択解除する()
    }
}
