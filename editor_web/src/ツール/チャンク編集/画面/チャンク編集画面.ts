import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { チャンク座標 } from '../../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { チャンク編集画面部品 } from './チャンク編集画面部品.ts'
import type { 三次元の配色 } from './三次元/三次元の配色.ts'
import { 画面ルート, 固定の行, 三次元の枠 } from './スタイル.css.ts'

// チャンク1件を編集する文書タブのエディタ領域。上に固定の1行(チャンクの名前・大きさ・モードの切替・
// チャンクぜんたいに効く操作)を置き、その下の残りをすべて地形の三次元へ渡す。
// 設定と一覧は インスペクター(右サイドバー)へ、これから使う筆と置くものは 棚(下パネル)へ渡す。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断14」
export class チャンク編集画面 extends LV2部品集約Base<チャンク編集画面部品> {
    protected _componentRoot: DivC
    public readonly 部品: チャンク編集画面部品

    public constructor(編集状態: ワールド編集状態, 対象座標: チャンク座標) {
        super()
        this.部品 = チャンク編集画面部品.作る(編集状態, 対象座標)
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    // 三次元が占める高さは固定の行と下パネルの開閉で変わる。呼び出し側が寸法を持参すると
    // CSSの値と食い違った解像度で描いても誰も気づかないため、キャンバスの枠を自分で測る。
    public 寸法を合わせる(ピクセル比: number = 1): void {
        this.部品.三次元ビュー.いまの枠の大きさへ合わせる(ピクセル比)
    }

    public 背景色を設定する(色: string | number): void {
        this.部品.三次元ビュー.背景色を設定する(色)
    }

    public 三次元の配色を設定する(配色: 三次元の配色): void {
        this.部品.三次元ビュー.三次元の配色を設定する(配色)
    }

    protected _ルートを構築する(部品: チャンク編集画面部品): DivC {
        return (
            div({ class: 画面ルート }).childs([
                div({ class: 固定の行 }).child(部品.操作帯),
                div({ class: 三次元の枠 }).child(部品.三次元ビュー)])
        )
    }

    public override delete(): void {
        this.部品.delete()
        super.delete()
    }
}
