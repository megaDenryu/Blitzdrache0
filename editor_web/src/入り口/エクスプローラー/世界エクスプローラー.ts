import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { チャンク座標 } from '../../境界/通信/index.ts'
import { タブ識別子 } from '../タブ識別子.ts'
import { コンテナ, セクション見出し } from './スタイル.css.ts'
import type { 領域エクスプローラー } from './領域エクスプローラー.ts'
import { 単一項目ノードを作る } from './木/単一項目ノード.ts'
import { チャンク木 } from './木/チャンク木.ts'

export interface I世界エクスプローラー配線 {
    readonly on大域世界を開く: () => void
    readonly onチャンクを開く: (座標: チャンク座標) => void
    readonly onマテリアルを開く: () => void
}

// 編集領域「世界」のエクスプローラー。大域世界・全チャンクの木・マテリアル台帳を並べる。
// マテリアル台帳をここへ置くのは、それが地表材質のための台帳だからである(判断9)。
export class 世界エクスプローラー extends LV2HtmlComponentBase implements I配線可能<I世界エクスプローラー配線>, 領域エクスプローラー {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<I世界エクスプローラー配線> = new 配線ポート<I世界エクスプローラー配線>('世界エクスプローラー')
    private readonly _大域世界ノード: DivC
    private readonly _チャンク木: チャンク木
    private readonly _マテリアルノード: DivC

    public constructor(軸あたりチャンク数: number = 4) {
        super()
        this._大域世界ノード = 単一項目ノードを作る('G', '大域世界', () => this._配線.先.on大域世界を開く())
        this._チャンク木 = new チャンク木(軸あたりチャンク数, (座標) => this._配線.先.onチャンクを開く(座標))
        this._マテリアルノード = 単一項目ノードを作る('M', 'マテリアル', () => this._配線.先.onマテリアルを開く())

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: セクション見出し, text: '世界' }),
            this._大域世界ノード,
            ...this._チャンク木.ルート要素一覧,
            this._マテリアルノード,
        ])
    }

    public 配線する(配線: I世界エクスプローラー配線): this {
        this._配線.配線する(配線)
        return this
    }

    public 前面のタブに合わせて選択表示する(タブ: タブ識別子): void {
        const 綴り = タブ.綴り()
        if (綴り === タブ識別子.大域世界().綴り()) {
            this._大域世界ノード.setAttribute('data-selected', 'true')
            return
        }
        if (綴り === タブ識別子.マテリアル().綴り()) {
            this._マテリアルノード.setAttribute('data-selected', 'true')
            return
        }
        const 座標 = タブ.チャンク座標を復元する()
        if (座標 !== null) this._チャンク木.選択表示する(座標)
    }

    public 選択表示を解除する(): void {
        this._大域世界ノード.setAttribute('data-selected', 'false')
        this._マテリアルノード.setAttribute('data-selected', 'false')
        this._チャンク木.選択解除する()
    }
}
