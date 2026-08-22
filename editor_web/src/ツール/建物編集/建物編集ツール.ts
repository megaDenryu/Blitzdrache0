import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import { 実サーバー接続, type 建物の格子接続 } from '../../境界/通信/index.ts'
import { 建物編集画面 } from './画面/index.ts'
import { 初期の建物の格子を作る } from './編集モデル/index.ts'
import { 建物編集の状態 } from './建物編集の状態.ts'
import { 建物編集同期サービス } from './建物編集同期サービス.ts'

// 建物1件の升目の複体を編集する文書タブのツールルート。三次元ビューを持たない(判断9:
// 絵の真実はエンジンが担い、このエディターは正本の値を定める)。実行可能ツールの契約は
// 空実装で満たす(描画ループが無く、寸法合わせも要らない)。
export class 建物編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 建物編集画面
    public readonly 状態: 建物編集の状態
    public readonly 同期: 建物編集同期サービス

    public constructor(建物定義ID: string, 表示名: string, 接続?: 建物の格子接続) {
        super()
        this.画面 = new 建物編集画面(建物定義ID)
        this.状態 = new 建物編集の状態(初期の建物の格子を作る(建物定義ID, 表示名))
        this.同期 = new 建物編集同期サービス(this.画面, this.状態, 接続 ?? new 実サーバー接続())
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        this.画面.表示名入力.setValue(表示名)
        this.同期.画面の出来事を配線する()
        this.同期.表示を作り直す()
        void this.同期.永続化.読み直す(() => this.同期.表示を作り直す())
    }

    public 寸法を合わせる(): void {}

    public 前面になった(): void {}

    public 背面になった(): void {}

    public override delete(): void {
        this.画面.delete()
        super.delete()
    }
}
