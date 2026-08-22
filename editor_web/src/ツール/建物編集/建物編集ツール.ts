import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import { 実サーバー接続, type 建物の格子接続 } from '../../境界/通信/index.ts'
import type { 建物定義ID } from '../../境界/建物定義ID.ts'
import { 建物編集画面 } from './画面/index.ts'
import { 初期の建物の格子を作る } from './編集モデル/index.ts'
import { 建物編集の状態 } from './建物編集の状態.ts'
import { 建物編集同期サービス } from './建物編集同期サービス.ts'

// 建物1件の升目の複体を編集する文書タブのツールルート。平面図と、役割ごとの識別色の三次元表示を並べる。
// 三次元表示が実部品のglbを読まないのは判断9のとおりであり、絵の真実はエンジンが担う。
export class 建物編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 建物編集画面
    public readonly 状態: 建物編集の状態
    public readonly 同期: 建物編集同期サービス

    public constructor(建物定義ID: 建物定義ID, 表示名: string, 接続?: 建物の格子接続) {
        super()
        this.画面 = new 建物編集画面(建物定義ID)
        this.状態 = new 建物編集の状態(初期の建物の格子を作る(建物定義ID, 表示名))
        this.同期 = new 建物編集同期サービス(this.画面, this.状態, 接続 ?? new 実サーバー接続())
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        this.画面.表示名入力.setValue(表示名)
        this.画面の出来事を同期サービスへ結ぶ()
        this.同期.表示を作り直す()
        void this.同期.永続化.読み直す(() => this.同期.表示を作り直す())
    }

    // 画面の押しボタンを同期サービスのメソッドへ結ぶ。配線をこのツールが持つのは、ここがこの道具の
    // コンポジションルートであり、画面とサービスの両方を知ってよい唯一の場所だからである。
    private 画面の出来事を同期サービスへ結ぶ(): void {
        this.画面.取り消しボタン.onClick(() => this.同期.取り消す())
        this.画面.やり直しボタン.onClick(() => this.同期.やり直す())
        this.画面.表示名入力.onChange(() => this.同期.表示名を変える())
        this.画面.永続化.on保存クリック(() => void this.同期.永続化.保存する())
        this.画面.永続化.on読込クリック(() => void this.同期.永続化.読み直す(() => this.同期.表示を作り直す()))
    }

    // 幅と高さを使わないのは、三次元表示が画面いっぱいでなく、巻き取る文書の中の高さの決まった箱だからである。
    // 箱の大きさはCSSが決めるため、パネルが自分のキャンバスを測る。ピクセル比だけを外殻から受け取る。
    public 寸法を合わせる(_幅: number, _高さ: number, ピクセル比: number = 1): void {
        this.画面.三次元.いまの枠の大きさへ合わせる(ピクセル比)
    }

    public 前面になった(): void {
        this.画面.三次元.描画を始める()
    }

    public 背面になった(): void {
        this.画面.三次元.描画を止める()
    }

    public override delete(): void {
        this.画面.delete()
        super.delete()
    }
}
