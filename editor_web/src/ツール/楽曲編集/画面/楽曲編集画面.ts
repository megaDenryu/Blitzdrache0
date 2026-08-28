import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { 楽曲 } from '../../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../../../境界/index.ts'
import {
    コード進行参照から和音一覧を解決する,
    初期楽曲を生成する,
    演奏の範囲の既定,
    type 演奏の範囲,
} from '../編集モデル/index.ts'
import type { I楽曲発音配線 } from './発音配線.ts'
import type { 再生位置 } from './演奏/index.ts'
import { 演奏の操作帯 } from './演奏の操作帯/index.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import { コンテナ, 固定の行, 進行の行 } from './スタイル.css.ts'
import { 楽曲名の欄 } from './楽曲名の欄.ts'
import { 進行制約の表示 } from './進行制約の表示.ts'
import { 進行の帯部品 } from './進行の帯部品.ts'
import { トラック領域部品 } from './トラック領域部品.ts'
import { 楽曲インスペクターパネル } from './パネル/index.ts'

// 楽曲エディター文書タブの画面。中央には楽曲名と演奏の操作の行・和音の帯・トラックの並びだけを置き、
// 設定の一式はインスペクターへ渡して右サイドバーへ出す(設計正本の判断14)。
// 中央のうち縦にスクロールするのはトラック領域だけで、上の2行は常に見えている。
export class 楽曲編集画面 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 楽曲名: 楽曲名の欄
    public readonly 操作帯: 演奏の操作帯
    public readonly 進行制約: 進行制約の表示 = new 進行制約の表示()
    public readonly 進行の帯: 進行の帯部品 = new 進行の帯部品()
    public readonly トラック領域: トラック領域部品 = new トラック領域部品()
    public readonly インスペクター: 楽曲インスペクターパネル
    public readonly 発音配線: 配線ポート<I楽曲発音配線> = new 配線ポート<I楽曲発音配線>('楽曲編集画面')
    private _表示中のパターンの名乗り: string | null = null
    private _表示中のパターンの表示名: string | null = null

    public constructor(楽曲ID: 楽曲ID, 初期楽曲?: 楽曲) {
        super()
        const 楽曲データ = 初期楽曲 !== undefined ? 初期楽曲 : 初期楽曲を生成する(楽曲ID, 楽曲ID)
        const 先頭のパターン = 楽曲データ.パターン一覧[0]
        const 初期選択の名乗り = 先頭のパターン === undefined ? null : 先頭のパターン.名乗り

        this.楽曲名 = new 楽曲名の欄(楽曲データ)
        this.操作帯 = new 演奏の操作帯(楽曲データ, 演奏の範囲の既定)
        this.インスペクター = new 楽曲インスペクターパネル(楽曲データ, 初期選択の名乗り)

        this._componentRoot = div({ class: コンテナ }).childs([
            div({ class: 固定の行 }).childs([this.楽曲名, this.操作帯]),
            div({ class: 進行の行 }).childs([this.進行制約, this.進行の帯]),
            this.トラック領域,
        ])
    }

    public 升目操作を配線する(
        on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void,
        on升目進入: (当たり: 升目の当たりの記録) => void,
    ): void {
        this.トラック領域.升目操作を配線する(on升目押下, on升目進入)
    }

    public 表示を更新する(
        楽曲: 楽曲,
        選択中パターン名乗り: string | null,
        進行の外モードか: boolean = false,
        ドラッグ見込み: 打ち込みドラッグ見込み | null = null,
    ): void {
        const パターン = 選択中パターン名乗り === null
            ? 楽曲.パターン一覧[0]
            : 楽曲.パターン一覧.find((p) => p.名乗り === 選択中パターン名乗り)

        this._表示中のパターンの名乗り = パターン !== undefined ? パターン.名乗り : null
        this._表示中のパターンの表示名 = パターン !== undefined ? パターン.表示名 : null
        this.楽曲名.表示を更新する(楽曲)
        this.進行制約.表示を更新する(進行の外モードか)
        this.操作帯.楽曲を反映する(楽曲)

        if (パターン === undefined) {
            this.進行の帯.表示を更新する([])
            this.トラック領域.表示を更新する(楽曲, undefined, ドラッグ見込み)
        } else {
            const パターン和音一覧 = コード進行参照から和音一覧を解決する(パターン.進行の参照, 楽曲.独自進行一覧)
            this.進行の帯.表示を更新する(パターン和音一覧)
            this.トラック領域.表示を更新する(楽曲, パターン, ドラッグ見込み)
        }

        this.インスペクター.表示を更新する(楽曲, 選択中パターン名乗り)
    }

    // 再生位置の印を格子と操作帯へ映す。開いているパターンと違うパターンが鳴っているときは格子を光らせない。
    public 再生位置を示す(位置: 再生位置 | null, 再生中か: boolean, 範囲: 演奏の範囲): void {
        const 同じパターンか = 位置 !== null && 位置.パターンの名乗り === this._表示中のパターンの名乗り
        this.トラック領域.再生位置を示す(同じパターンか && 位置 !== null ? 位置.パターン内ステップ : null)
        this.操作帯.演奏の様子を反映する(
            再生中か,
            範囲,
            位置,
            同じパターンか ? this._表示中のパターンの表示名 : null,
        )
    }

    public override delete(): void {
        this.楽曲名.delete()
        this.操作帯.delete()
        this.進行制約.delete()
        this.進行の帯.delete()
        this.トラック領域.delete()
        this.インスペクター.delete()
        super.delete()
    }
}
