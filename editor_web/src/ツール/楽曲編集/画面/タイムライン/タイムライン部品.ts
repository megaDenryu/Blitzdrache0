import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { I配線可能 } from 'sengen-ui'
import type { 楽曲 } from '../../../../生成/編集資源契約.ts'
import {
    カード位置は同じか,
    曲構成をカードの列へ展開する,
    type カード位置,
    type 曲構成のカード,
    type 演奏の範囲,
} from '../../編集モデル/index.ts'
import type { 再生位置 } from '../演奏/index.ts'
import { カード操作の押せるかを計算する } from './カード操作の押せるかを計算する.ts'
import { カード部品 } from './カード部品.ts'
import type { カード操作の種類 } from './カード操作の種類.ts'
import { 節の枠部品 } from './節の枠部品.ts'
import { 節の枠一覧を組み立てる, 節の枠一覧を配線する } from './節の枠一覧を組み立てる.ts'
import type { 節移動の種類 } from './節移動の種類.ts'
import { タイムラインの再生印 } from './タイムラインの再生印.ts'
import { 末尾へ追加ボタン } from './末尾へ追加ボタン.ts'
import { タイムライン枠, 案内文 } from './スタイル.css.ts'

export interface Iタイムライン配線 {
    readonly onカード選択: (位置: カード位置) => void
    readonly onカード操作: (位置: カード位置, 種類: カード操作の種類) => void
    readonly on節移動: (位置: カード位置, 種類: 節移動の種類) => void
    readonly on末尾へ追加: () => void
}

// 曲構成を横方向へ展開したカードの並びを見せる、エディタ領域の固定の行(設計正本の判断15)。
// 枠の中だけが横にスクロールし、縦には伸びない。
export class タイムライン部品 extends LV2HtmlComponentBase implements I配線可能<Iタイムライン配線> {
    protected _componentRoot: DivC
    private readonly _配線: 配線ポート<Iタイムライン配線> = new 配線ポート<Iタイムライン配線>('タイムライン部品')
    private readonly _追加ボタン: 末尾へ追加ボタン = new 末尾へ追加ボタン()
    private _カード部品一覧: カード部品[] = []
    private _節の枠一覧: 節の枠部品[] = []
    private _カード列: readonly 曲構成のカード[] = []
    private readonly _再生印: タイムラインの再生印 = new タイムラインの再生印()

    public constructor() {
        super()
        this._componentRoot = div({ class: タイムライン枠 })
        this._追加ボタン.onClick(() => { if (this._配線.配線済みか) this._配線.先.on末尾へ追加() })
    }

    public 配線する(配線: Iタイムライン配線): this {
        this._配線.配線する(配線)
        this._カード列を配線する()
        return this
    }

    public 表示を更新する(楽曲: 楽曲, 選択中のカード: カード位置 | null, 選択中パターンの名乗り: string | null): void {
        for (const カード of this._カード部品一覧) カード.delete()
        for (const 枠 of this._節の枠一覧) 枠.delete()
        this._componentRoot.clearChildren()
        this._カード部品一覧 = []
        this._節の枠一覧 = []

        let 選択中の添字: number | null = null
        this._カード列 = 曲構成をカードの列へ展開する(楽曲.曲構成)
        if (this._カード列.length === 0) {
            this._componentRoot.child(div({ class: 案内文, text: '曲構成が空。いまのパターンを繰り返して鳴らす' }))
        } else {
            this._カード部品一覧 = this._カード列.map((カード, 添字) => {
                const 選択中か = 選択中のカード !== null && カード位置は同じか(カード.位置, 選択中のカード)
                if (選択中か) 選択中の添字 = 添字
                const 押せるか = 選択中か ? カード操作の押せるかを計算する(楽曲.曲構成, カード.位置) : null
                return new カード部品(カード, 楽曲.パターン一覧, 選択中か, 押せるか)
            })
            this._節の枠一覧 = 節の枠一覧を組み立てる(楽曲, this._カード列, this._カード部品一覧)
            this._componentRoot.childs(this._節の枠一覧)
        }
        this._再生印.リセットする(選択中の添字)
        this._componentRoot.child(this._追加ボタン.選択中パターンを反映する(選択中パターンの名乗り))
        this._カード列を配線する()
        if (this._配線.配線済みか) {
            節の枠一覧を配線する(this._節の枠一覧, this._カード列, (位置, 種類) => this._配線.先.on節移動(位置, 種類))
        }
    }

    // 音声の時計から導いた再生位置が、いま鳴っているカードのどれに当たるかを求める(画面から呼ばれる)。
    public 再生中のカード位置を求める(位置: 再生位置 | null, 範囲: 演奏の範囲): カード位置 | null {
        return this._再生印.再生中のカード位置を求める(this._カード列, 位置, 範囲)
    }

    // 再生中は画面の1コマごとに呼ばれるため、変わったカードだけ触る(トラック格子部品と同じ規律)。
    public 再生位置を示す(再生中のカード: カード位置 | null, 繰り返し中か: boolean): void {
        this._再生印.示す(this._カード部品一覧, 再生中のカード, 繰り返し中か)
    }

    public override delete(): void {
        for (const カード of this._カード部品一覧) カード.delete()
        for (const 枠 of this._節の枠一覧) 枠.delete()
        this._追加ボタン.delete()
        super.delete()
    }

    private _カード列を配線する(): void {
        if (!this._配線.配線済みか) return
        for (const カード of this._カード部品一覧) {
            カード.配線する({
                onクリック: () => this._配線.先.onカード選択(カード.位置),
                on操作: (種類) => this._配線.先.onカード操作(カード.位置, 種類),
            })
        }
    }
}
