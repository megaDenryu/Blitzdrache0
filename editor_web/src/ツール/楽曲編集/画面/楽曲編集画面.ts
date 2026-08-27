import { div, DivC, LV2HtmlComponentBase, 配線ポート } from 'sengen-ui'
import type { 楽曲 } from '../../../生成/編集資源契約.ts'
import type { 楽曲ID } from '../../../境界/index.ts'
import { コード進行参照から和音一覧を解決する, 初期楽曲を生成する } from '../編集モデル/index.ts'
import { 永続化パネル } from '../../チャンク編集/画面/パネル/永続化/index.ts'
import type { I楽曲発音配線 } from './発音配線.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import { コンテナ, 本文幅 } from './スタイル.css.ts'
import { 楽曲ヘッダー部品 } from './楽曲ヘッダー部品.ts'
import { 進行の帯部品 } from './進行の帯部品.ts'
import { トラック領域部品 } from './トラック領域部品.ts'
import {
    曲設定パネル,
    パターンパネル,
    曲構成パネル,
    トラック設定パネル,
    コード進行パネル,
} from './パネル/index.ts'

// 楽曲エディター文書タブの画面全体。ヘッダー・進行の帯・トラック領域・各種編集パネル・永続化パネルを束ねる。
export class 楽曲編集画面 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly ヘッダー: 楽曲ヘッダー部品
    public readonly 進行の帯: 進行の帯部品 = new 進行の帯部品()
    public readonly トラック領域: トラック領域部品 = new トラック領域部品()
    public readonly パターン: パターンパネル
    public readonly 曲構成: 曲構成パネル
    public readonly 曲設定: 曲設定パネル
    public readonly トラック設定: トラック設定パネル
    public readonly コード進行: コード進行パネル
    public readonly 永続化: 永続化パネル = new 永続化パネル()
    public readonly 発音配線: 配線ポート<I楽曲発音配線> = new 配線ポート<I楽曲発音配線>('楽曲編集画面')

    public constructor(楽曲ID: 楽曲ID, 初期楽曲?: 楽曲) {
        super()
        const 楽曲データ = 初期楽曲 !== undefined ? 初期楽曲 : 初期楽曲を生成する(楽曲ID, 楽曲ID)
        const 初期選択名乗り = 楽曲データ.パターン一覧.length > 0 && 楽曲データ.パターン一覧[0] !== undefined
            ? 楽曲データ.パターン一覧[0].名乗り
            : null

        this.ヘッダー = new 楽曲ヘッダー部品(楽曲ID)
        this.パターン = new パターンパネル(楽曲データ, 初期選択名乗り)
        this.曲構成 = new 曲構成パネル(楽曲データ, 初期選択名乗り)
        this.曲設定 = new 曲設定パネル(楽曲データ)
        this.トラック設定 = new トラック設定パネル(楽曲データ)
        this.コード進行 = new コード進行パネル(楽曲データ)

        this._componentRoot = div({ class: コンテナ }).child(
            div({ class: 本文幅 }).childs([
                this.ヘッダー,
                this.進行の帯,
                this.トラック領域,
                this.パターン,
                this.曲構成,
                this.曲設定,
                this.トラック設定,
                this.コード進行,
                this.永続化,
            ]),
        )
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

        const パターン表示名 = パターン !== undefined ? パターン.表示名 : null
        this.ヘッダー.表示を更新する(楽曲, パターン表示名, 進行の外モードか)

        if (パターン === undefined) {
            this.進行の帯.表示を更新する([])
            this.トラック領域.表示を更新する(楽曲, undefined, ドラッグ見込み)
        } else {
            const パターン和音一覧 = コード進行参照から和音一覧を解決する(パターン.進行の参照, 楽曲.独自進行一覧)
            this.進行の帯.表示を更新する(パターン和音一覧)
            this.トラック領域.表示を更新する(楽曲, パターン, ドラッグ見込み)
        }

        this.パターン.表示を更新する(楽曲, 選択中パターン名乗り)
        this.曲構成.表示を更新する(楽曲, 選択中パターン名乗り)
        this.曲設定.表示を更新する(楽曲)
        this.トラック設定.表示を更新する(楽曲)
        this.コード進行.表示を更新する(楽曲)
    }

    public override delete(): void {
        this.ヘッダー.delete()
        this.進行の帯.delete()
        this.トラック領域.delete()
        this.パターン.delete()
        this.曲構成.delete()
        this.曲設定.delete()
        this.トラック設定.delete()
        this.コード進行.delete()
        this.永続化.delete()
        super.delete()
    }
}
