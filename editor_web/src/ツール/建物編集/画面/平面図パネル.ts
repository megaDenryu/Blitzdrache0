import { div, DivC, LV2HtmlComponentBase } from 'sengen-ui'
import type { 升目の座標 } from '../../../生成/編集資源契約.ts'
import { 升目の座標が同じか, 隣の座標, type 建物の格子の編集モデル } from '../編集モデル/index.ts'
import { 平面図の升目部品, type I升目の配線 } from './平面図の升目部品.ts'
import { 平面図 } from './スタイル.css.ts'

// 空の升目を何列ぶん周りへ足して描くか。置ける場所が画面から読めるようにするため、
// 既にある升目の外側へ1列ぶんの余白を出す。
const 余白の列数 = 1

// 名指した階の升目を平面図として描くパネル。触った結果をどう変えるかは筆が決め、この部品は
// 触られた座標と面を配線先へ伝えるだけである。
export class 平面図パネル extends LV2HtmlComponentBase {
    protected _componentRoot: DivC = div({ class: 平面図 })

    public 再構築する(モデル: 建物の格子の編集モデル, 階: number, 配線: I升目の配線): void {
        this._componentRoot.clearChildren()
        const 範囲 = 描く範囲を求める(モデル, 階)
        this._componentRoot.setStyleCSS({
            gridTemplateColumns: `repeat(${範囲.横の最大 - 範囲.横の最小 + 1}, auto)`,
        })
        for (let 奥 = 範囲.奥の最小; 奥 <= 範囲.奥の最大; 奥 += 1) {
            for (let 横 = 範囲.横の最小; 横 <= 範囲.横の最大; 横 += 1) {
                this._componentRoot.child(this.升目を1つ作る(モデル, { 横, 奥, 階 }, 配線))
            }
        }
    }

    private 升目を1つ作る(モデル: 建物の格子の編集モデル, 座標: 升目の座標, 配線: I升目の配線): 平面図の升目部品 {
        return new 平面図の升目部品(
            {
                座標,
                宣言: モデル.宣言を引く(座標),
                根か: 升目の座標が同じか(モデル.根の升目, 座標),
                升目を置けない理由: モデル.升目を置けない理由を調べる(座標),
                隣に升目があるか: (側面) => モデル.升目があるか(隣の座標(座標, 側面)),
            },
            配線,
        )
    }
}

interface 描く範囲 {
    横の最小: number
    横の最大: number
    奥の最小: number
    奥の最大: number
}

// その階にある升目を全部含み、周りへ余白の列を足した範囲。升目が1つも無い階は原点の周りだけを描く。
function 描く範囲を求める(モデル: 建物の格子の編集モデル, 階: number): 描く範囲 {
    const その階の升目 = モデル.升目を昇順に並べる().filter((宣言) => 宣言.座標.階 === 階)
    if (その階の升目.length === 0) {
        return { 横の最小: -余白の列数, 横の最大: 余白の列数, 奥の最小: -余白の列数, 奥の最大: 余白の列数 }
    }
    const 横一覧 = その階の升目.map((宣言) => 宣言.座標.横)
    const 奥一覧 = その階の升目.map((宣言) => 宣言.座標.奥)
    return {
        横の最小: Math.min(...横一覧) - 余白の列数,
        横の最大: Math.max(...横一覧) + 余白の列数,
        奥の最小: Math.min(...奥一覧) - 余白の列数,
        奥の最大: Math.max(...奥一覧) + 余白の列数,
    }
}
