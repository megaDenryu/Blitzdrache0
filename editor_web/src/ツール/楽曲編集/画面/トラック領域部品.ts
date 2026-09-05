import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 楽曲, パターン } from '../../../生成/編集資源契約.ts'
import { トラックに適用される和音一覧を解決する, パターンのステップ数を求める } from '../編集モデル/index.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import { トラックの並びの枠 } from './スタイル.css.ts'
import { トラックブロック部品 } from './トラックブロック部品.ts'

// 全トラックのブロック（見出し＋打ち込み格子）を縦に並べ、パターンの切り替えやトラック構成の変更に追従する部品。
// 中央で数が増えうるのはトラックだけであり、縦にスクロールするのはこの部品の中だけである(設計正本の判断14)。
export class トラック領域部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private _トラックブロック一覧: トラックブロック部品[] = []
    private _選択中パターン名乗り: string | null = null
    private _on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void = () => {}
    private _on升目進入: (当たり: 升目の当たりの記録) => void = () => {}

    public constructor() {
        super()
        this._componentRoot = div({ class: トラックの並びの枠 })
    }

    public 升目操作を配線する(
        on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void,
        on升目進入: (当たり: 升目の当たりの記録) => void,
    ): void {
        this._on升目押下 = on升目押下
        this._on升目進入 = on升目進入
    }

    public 表示を更新する(
        楽曲: 楽曲,
        パターン: パターン | undefined,
        ドラッグ見込み: 打ち込みドラッグ見込み | null,
        強調する小節: number | null,
    ): void {
        if (パターン === undefined) {
            this._選択中パターン名乗り = null
            this._componentRoot.clearChildren()
            return
        }

        const ステップ数 = パターンのステップ数を求める(パターン)
        this._選択中パターン名乗り = パターン.名乗り
        this.トラックブロック一覧を同期する(楽曲, ステップ数)

        for (let 位置 = 0; 位置 < 楽曲.トラック構成.length; 位置++) {
            const トラック = 楽曲.トラック構成[位置]
            const トラック格子 = パターン.格子[位置]
            const ブロック = this._トラックブロック一覧[位置]
            if (トラック === undefined || トラック格子 === undefined || ブロック === undefined) continue

            const トラック和音一覧 = トラックに適用される和音一覧を解決する(
                トラック,
                パターン.進行の参照,
                楽曲.独自進行一覧,
            )
            ブロック.表示を更新する(
                トラック,
                トラック格子,
                トラック和音一覧,
                パターン.名乗り,
                位置,
                ドラッグ見込み,
                ステップ数,
            )
            ブロック.強調する小節を示す(強調する小節)
        }
    }

    public 再生位置を示す(ステップ: number | null): void {
        for (const ブロック of this._トラックブロック一覧) ブロック.再生位置を示す(ステップ)
    }

    private トラックブロック一覧を同期する(楽曲: 楽曲, 初期ステップ数: number): void {
        if (this._トラックブロック一覧.length === 楽曲.トラック構成.length) return
        for (const ブロック of this._トラックブロック一覧) ブロック.delete()
        this._componentRoot.clearChildren()
        this._トラックブロック一覧 = 楽曲.トラック構成.map((トラック, 位置) => {
            const ブロック = new トラックブロック部品(
                トラック,
                位置,
                () => this._選択中パターン名乗り !== null ? this._選択中パターン名乗り : '',
                this._on升目押下,
                this._on升目進入,
                初期ステップ数,
            )
            this._componentRoot.child(ブロック)
            return ブロック
        })
    }

    public override delete(): void {
        for (const ブロック of this._トラックブロック一覧) ブロック.delete()
        super.delete()
    }
}
