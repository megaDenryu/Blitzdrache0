import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { 楽曲ID } from '../../境界/index.ts'
import type { 楽曲接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { 楽曲編集画面 } from './画面/index.ts'
import { 楽曲編集状態, 初期楽曲を生成する } from './編集モデル/index.ts'
import { 楽曲編集イベントを配線する } from './楽曲編集配線.ts'
import { 起動時に楽曲を読み込む } from './楽曲起動時読込.ts'

// 楽曲1件の打ち込み格子・進行の帯・永続化を編集する文書タブのツールルート。
// 三次元ビューを持たない。実行可能ツールの契約は空実装で満たす。
export class 楽曲編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 楽曲編集画面
    public readonly 状態: 楽曲編集状態
    public readonly UI状態: 楽曲編集UI状態
    public readonly 操作: 楽曲履歴適用サービス
    public readonly 接続: 楽曲接続
    private readonly _購読解除: () => void

    public constructor(楽曲ID: 楽曲ID, 表示名: string, 接続?: 楽曲接続) {
        super()
        this.画面 = new 楽曲編集画面(楽曲ID)
        this.状態 = new 楽曲編集状態(初期楽曲を生成する(楽曲ID, 表示名))
        this.UI状態 = new 楽曲編集UI状態()
        this.操作 = new 楽曲履歴適用サービス(this.状態, () => {
            this.画面.表示を更新する(
                this.状態.楽曲を取得する(),
                this.状態.選択中パターンの名乗り,
                this.UI状態.進行の外モードか,
                this.UI状態.ドラッグ見込み,
            )
        })
        this.接続 = 接続 === undefined ? new 実サーバー接続() : 接続
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        this._購読解除 = 楽曲編集イベントを配線する(
            this.画面,
            this.状態,
            this.UI状態,
            this.操作,
            this.接続,
            楽曲ID,
        )
        void 起動時に楽曲を読み込む(this.画面, this.状態, this.UI状態, this.接続, 楽曲ID)
    }

    public 寸法を合わせる(): void {}

    public 前面になった(): void {}

    public 背面になった(): void {}

    public override delete(): void {
        this._購読解除()
        this.画面.delete()
        super.delete()
    }
}

