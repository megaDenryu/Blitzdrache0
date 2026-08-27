import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import {
    パターンのステップ数,
    type トラックの格子,
    type トラック定義,
    type 和音,
} from '../../../生成/編集資源契約.ts'
import {
    数値からセルへ変換する,
    トラックの行の音はステップで許されるか,
} from '../編集モデル/index.ts'
import { 見込みを反映したセルを計算する } from './見込みセル計算.ts'
import type { 打ち込みドラッグ見込み, 升目の当たりの記録 } from './打ち込み見込み.ts'
import { 格子枠, 行枠, 行見出し, 升目列 } from './スタイル.css.ts'
import { 打ち込み升目部品 } from './打ち込み升目部品.ts'
import { トラック行の表示名 } from './音名表示.ts'

// 1つのトラックの全行・全ステップ（パターンのステップ数）の升目を並べる部品。
export class トラック格子部品 extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    private readonly _行升目一覧: 打ち込み升目部品[][] = []
    private _光っているステップ: number | null = null

    public constructor(
        トラック: トラック定義,
        トラックの位置: number,
        パターンの名乗り取得: () => string,
        on升目押下: (当たり: 升目の当たりの記録, ボタン: number) => void,
        on升目進入: (当たり: 升目の当たりの記録) => void,
    ) {
        super()
        this._componentRoot = div({ class: 格子枠 })
        const 行数 = トラック.音の並び.値.length

        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const 表示名 = トラック行の表示名(トラック.音の並び, 行位置)
            const 行見出し要素 = div({ class: 行見出し, text: 表示名 }).setTooltip(表示名)
            const 升目一覧: 打ち込み升目部品[] = []

            for (let ステップ = 0; ステップ < パターンのステップ数; ステップ++) {
                const 升目 = new 打ち込み升目部品(ステップ)
                const 当たりを作る = (): 升目の当たりの記録 => ({
                    パターンの名乗り: パターンの名乗り取得(),
                    トラックの位置,
                    行の位置: 行位置,
                    ステップ,
                })
                升目.onポインタ押下((ボタン) => on升目押下(当たりを作る(), ボタン))
                升目.onポインタ進入(() => on升目進入(当たりを作る()))
                升目一覧.push(升目)
            }
            this._行升目一覧.push(升目一覧)

            const 升目列要素 = div({ class: 升目列 }).childs(升目一覧)
            const 行要素 = div({ class: 行枠 }).childs([行見出し要素, 升目列要素])
            this._componentRoot.child(行要素)
        }
    }

    // 再生位置の印を、前に光っていた列から新しい列へ移す。毎コマ全部の升目へ触らないために差分だけを触る。
    public 再生位置を示す(ステップ: number | null): void {
        if (this._光っているステップ === ステップ) return
        if (this._光っているステップ !== null) this._列の印を切り替える(this._光っているステップ, false)
        if (ステップ !== null) this._列の印を切り替える(ステップ, true)
        this._光っているステップ = ステップ
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
    ): void {
        const 行数 = トラック.音の並び.値.length
        for (let 行位置 = 0; 行位置 < 行数; 行位置++) {
            const 行データ = トラック格子.行一覧[行位置]
            const 升目一覧 = this._行升目一覧[行位置]
            if (行データ === undefined || 升目一覧 === undefined) continue

            for (let ステップ = 0; ステップ < パターンのステップ数; ステップ++) {
                const 生値 = 行データ[ステップ]
                const 升目部品 = 升目一覧[ステップ]
                if (生値 === undefined || 升目部品 === undefined) continue

                const 対象セル = 見込みを反映したセルを計算する(
                    数値からセルへ変換する(生値),
                    パターンの名乗り,
                    トラックの位置,
                    行位置,
                    ステップ,
                    ドラッグ見込み,
                )
                const 許されるか = トラックの行の音はステップで許されるか(
                    トラック,
                    行位置,
                    ステップ,
                    和音一覧,
                )
                升目部品.表示を更新する(対象セル, 許されるか)
            }
        }
    }
}


