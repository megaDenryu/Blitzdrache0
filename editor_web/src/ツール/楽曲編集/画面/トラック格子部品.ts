import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    type トラックの格子,
    type トラック定義,
    type 和音,
} from '../../../生成/編集資源契約.ts'
import { 小節あたりのステップ数 } from '../編集モデル/index.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import {
    格子の1行の打点を反映する,
    格子の1行を構築する,
    type 升目当たり配線,
} from './トラック格子の行.ts'
import type { 打ち込み升目部品 } from './打ち込み升目部品.ts'
import { 格子枠 } from './スタイル.css.ts'

// 1つのトラックの全行・全ステップ（開いているパターンの小節数から導くステップ数）の升目を並べる部品。
// パターンの小節数はパターンごとに違い、パターンの小節数を変える操作で変わることもあるため、
// 升目一覧はステップ数が変わるたびに組み直す。
export class トラック格子部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _トラック: トラック定義
    private readonly _トラックの位置: number
    private readonly _当たり配線: 升目当たり配線
    private _行升目一覧: (readonly 打ち込み升目部品[])[] = []
    private _ステップ数: number
    private _光っているステップ: number | null = null

    public constructor(
        トラック: トラック定義,
        トラックの位置: number,
        パターンの名乗り取得: () => string,
        on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void,
        on升目進入: (当たり: 升目の当たりの記録) => void,
        初期ステップ数: number,
    ) {
        super()
        this._componentRoot = div({ class: 格子枠 })
        this._トラック = トラック
        this._トラックの位置 = トラックの位置
        this._当たり配線 = { パターンの名乗り取得, on升目押下, on升目進入 }
        this._ステップ数 = 初期ステップ数
        this._格子を構築する()
    }

    // 再生位置の印を、前に光っていた列から新しい列へ移す。毎コマ全部の升目へ触らないために差分だけを触る。
    public 再生位置を示す(ステップ: number | null): void {
        if (this._光っているステップ === ステップ) return
        if (this._光っているステップ !== null) this._列の印を切り替える(this._光っているステップ, false)
        if (ステップ !== null) this._列の印を切り替える(ステップ, true)
        this._光っているステップ = ステップ
    }

    // 選択中のカードが指す小節の16列を強調する。選択が外れたら null で解除する。
    public 強調する小節を示す(小節の番号: number | null): void {
        const 始まり = 小節の番号 === null ? null : 小節の番号 * 小節あたりのステップ数
        for (const 升目一覧 of this._行升目一覧) {
            升目一覧.forEach((升目, ステップ) => {
                const 強調するか = 始まり !== null && ステップ >= 始まり && ステップ < 始まり + 小節あたりのステップ数
                升目.小節の強調を示す(強調するか)
            })
        }
    }

    private _列の印を切り替える(ステップ: number, 光らせるか: boolean): void {
        for (const 升目一覧 of this._行升目一覧) {
            升目一覧[ステップ]?.再生の印を示す(光らせるか)
        }
    }

    public 表示を更新する(
        トラック: トラック定義,
        トラック格子: トラックの格子,
        和音一覧: readonly 和音[] | null,
        パターンの名乗り: string,
        トラックの位置: number,
        ドラッグ見込み: 打ち込みドラッグ見込み | null,
        ステップ数: number,
    ): void {
        if (ステップ数 !== this._ステップ数) {
            this._ステップ数 = ステップ数
            this._格子を構築する()
        }

        const 当てどころ = { トラック, パターンの名乗り, トラックの位置, 和音一覧, ドラッグ見込み, ステップ数 }
        const 行数 = トラック.音の並び.値.length
        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const 行データ = トラック格子.行一覧[行位置]
            const 升目一覧 = this._行升目一覧[行位置]
            if (行データ === undefined || 升目一覧 === undefined) continue
            格子の1行の打点を反映する(行データ, 行位置, 升目一覧, 当てどころ)
        }
    }

    private _格子を構築する(): void {
        this._行升目一覧 = []
        this._componentRoot.clearChildren()
        this._光っているステップ = null
        const 行数 = this._トラック.音の並び.値.length

        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const { 升目一覧, 行要素 } = 格子の1行を構築する(
                this._トラック,
                this._トラックの位置,
                行位置,
                this._ステップ数,
                this._当たり配線,
            )
            this._行升目一覧.push(升目一覧)
            this._componentRoot.child(行要素)
        }
    }
}
