import { div, LV2HtmlComponentBase, 配線ポート, type DivC } from 'sengen-ui'
import { 実サーバー接続, type 建物の格子接続 } from '../../境界/通信/index.ts'
import type { 建物定義ID } from '../../境界/建物定義ID.ts'
import { 建物インスペクターパネル, 建物編集画面, 部品の棚 } from './画面/index.ts'
import { 初期の建物の格子を作る } from './編集モデル/index.ts'
import { 建物編集キーボード入力を配線する } from './キーボード配線.ts'
import { 建物編集の状態 } from './建物編集の状態.ts'
import type { I建物の表示名の届け先 } from './建物編集の表示係.ts'
import { 建物編集同期サービス } from './建物編集同期サービス.ts'

// 建物1件の升目の複体を編集する文書タブのツールルート。
// エディタ領域には建物の形の三次元と平面図だけを置き、設定と階の一覧は インスペクター として
// 右サイドバーへ、これから配置する部品の棚は 下パネル として画面下へ渡す(判断14)。
// 三次元表示が実部品のモデルを読まないのは、部品のモデルを配る経路がまだブラウザへ無いためであり、
// 寸法の分かる簡易の形で代える(判断13の第2段)。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「判断13」「判断14」
export class 建物編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 建物編集画面
    public readonly 状態: 建物編集の状態
    public readonly 同期: 建物編集同期サービス
    public readonly 表示名の知らせの口: 配線ポート<I建物の表示名の届け先> = new 配線ポート<I建物の表示名の届け先>('建物編集ツール')
    private readonly _鍵盤の解除: () => void

    public constructor(建物定義ID: 建物定義ID, 表示名: string, 接続?: 建物の格子接続) {
        super()
        this.画面 = new 建物編集画面(建物定義ID)
        this.状態 = new 建物編集の状態(初期の建物の格子を作る(建物定義ID, 表示名))
        this.同期 = new 建物編集同期サービス(this.画面, this.状態, 接続 ?? new 実サーバー接続(), this.表示名の知らせの口)
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        this.画面の出来事を同期サービスへ結ぶ()
        this._鍵盤の解除 = 建物編集キーボード入力を配線する(this.同期)
        this.同期.表示を作り直す()
        void this.同期.永続化.読み直す(() => this.同期.表示を作り直す())
    }

    public get インスペクター(): 建物インスペクターパネル {
        return this.画面.部品.インスペクター
    }

    public get 下パネル(): 部品の棚 {
        return this.画面.部品.部品の棚
    }

    // 幅と高さを使わないのは、三次元表示がエディタ領域の中で高さの決まった箱を埋めるからである。
    // 箱の大きさはCSSが決めるため、パネルが自分のキャンバスを測る。ピクセル比だけを外殻から受け取る。
    public 寸法を合わせる(_幅: number, _高さ: number, ピクセル比: number = 1): void {
        this.画面.部品.三次元.いまの枠の大きさへ合わせる(ピクセル比)
    }

    public 前面になった(): void {
        this.画面.部品.三次元.描画を始める()
    }

    public 背面になった(): void {
        this.画面.部品.三次元.描画を止める()
    }

    public override delete(): void {
        this._鍵盤の解除()
        this.画面.delete()
        super.delete()
    }

    // 画面の部品の出来事を同期サービスのメソッドへ結ぶ。配線をこのツールが持つのは、ここがこの道具の
    // コンポジションルートであり、画面とサービスの両方を知ってよい唯一の場所だからである。
    private 画面の出来事を同期サービスへ結ぶ(): void {
        const 部品 = this.画面.部品
        部品.建物名.配線する({
            on表示名が入力された: (入力中の表示名) => this.同期.表示名が入力された(入力中の表示名),
            on表示名が決まった: (新しい表示名) => this.同期.表示名が決まった(新しい表示名),
        })
        部品.操作帯.配線する({
            on取り消す: () => this.同期.取り消す(),
            onやり直す: () => this.同期.やり直す(),
            on識別色の重ねを切り替える: () => this.同期.識別色の重ねを切り替える(),
            on建物ぜんたいを写す: () => this.同期.建物ぜんたいを写す(),
        })
        部品.インスペクター.永続化.on保存クリック(() => void this.同期.永続化.保存する())
        部品.インスペクター.永続化.on読込クリック(() => void this.同期.永続化.読み直す(() => this.同期.表示を作り直す()))
        部品.三次元.触りを結ぶ({ on升目を選ぶ: (座標) => this.同期.升目を選ぶ(座標) })
    }
}
