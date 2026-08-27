import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { 楽曲ID } from '../../境界/index.ts'
import type { 楽曲接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import { 楽曲編集画面 } from './画面/index.ts'
import { 楽曲編集状態, 初期楽曲を生成する } from './編集モデル/index.ts'
import { 楽曲編集イベントを配線する, 起動時に楽曲を読み込む } from './楽曲編集配線.ts'

// 楽曲1件の打ち込み格子・進行の帯・永続化を編集する文書タブのツールルート。
// 三次元ビューを持たない。実行可能ツールの契約は空実装で満たす。
export class 楽曲編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 楽曲編集画面
    public readonly 状態: 楽曲編集状態
    public readonly 接続: 楽曲接続

    public constructor(楽曲ID: 楽曲ID, 表示名: string, 接続?: 楽曲接続) {
        super()
        this.画面 = new 楽曲編集画面(楽曲ID)
        this.状態 = new 楽曲編集状態(初期楽曲を生成する(楽曲ID, 表示名))
        this.接続 = 接続 === undefined ? new 実サーバー接続() : 接続
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        const 表示を再構築する = 楽曲編集イベントを配線する(this.画面, this.状態, this.接続, 楽曲ID)
        void 起動時に楽曲を読み込む(this.画面, this.状態, this.接続, 楽曲ID, 表示を再構築する)
    }

    public 寸法を合わせる(): void {}

    public 前面になった(): void {}

    public 背面になった(): void {}

    public override delete(): void {
        this.画面.delete()
        super.delete()
    }
}
