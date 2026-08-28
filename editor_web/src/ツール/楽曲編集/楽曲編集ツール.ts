import { div, LV2HtmlComponentBase, 配線ポート, type DivC } from 'sengen-ui'
import type { 楽曲ID } from '../../境界/index.ts'
import type { 楽曲接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import { 表示名の編集をまとめる係 } from '../../文書の表示名の編集/index.ts'
import { 楽曲履歴適用サービス } from './操作コマンド/楽曲履歴適用サービス.ts'
import { 楽曲名の変更の反映 } from './楽曲名の変更の反映.ts'
import { 楽曲編集UI状態 } from './楽曲編集UI状態.ts'
import { 升目の発音, 演奏サービス, 楽曲インスペクターパネル, 楽曲編集画面 } from './画面/index.ts'
import { 楽曲編集状態, 初期楽曲を生成する } from './編集モデル/index.ts'
import { 楽曲編集イベントを配線する } from './楽曲編集配線.ts'
import { 楽曲編集の表示の同期, type I楽曲の表示名の届け先 } from './表示の同期.ts'
import { 起動時に楽曲を読み込む } from './楽曲起動時読込.ts'

// 楽曲1件の打ち込み格子・和音の帯・演奏・永続化を編集する文書タブのツールルート。
// 設定の一式は インスペクター として外殻の右サイドバーへ渡る(設計正本の判断14)。
// 三次元ビューを持たないため、寸法と前面背面の契約は空実装で満たす。
export class 楽曲編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 画面: 楽曲編集画面
    public readonly インスペクター: 楽曲インスペクターパネル
    public readonly 状態: 楽曲編集状態
    public readonly UI状態: 楽曲編集UI状態
    public readonly 操作: 楽曲履歴適用サービス
    public readonly 接続: 楽曲接続
    public readonly 演奏: 演奏サービス
    public readonly 表示名の知らせの口: 配線ポート<I楽曲の表示名の届け先> = new 配線ポート<I楽曲の表示名の届け先>('楽曲編集ツール')
    public readonly 表示名の編集: 表示名の編集をまとめる係
    private readonly 同期: 楽曲編集の表示の同期
    private readonly _購読解除: () => void

    public constructor(楽曲ID: 楽曲ID, 表示名: string, 接続?: 楽曲接続) {
        super()
        this.画面 = new 楽曲編集画面(楽曲ID)
        this.インスペクター = this.画面.インスペクター
        this.状態 = new 楽曲編集状態(初期楽曲を生成する(楽曲ID, 表示名))
        this.UI状態 = new 楽曲編集UI状態()
        // 操作を同期より先に作るのは、表示名の編集をまとめる係が操作を要り、同期がその係を要るためである。
        // 操作が受け取る作り直しの呼び出しは、構築が終わったあとにしか走らない。
        this.操作 = new 楽曲履歴適用サービス(this.状態, () => { this.同期.再構築する() })
        this.表示名の編集 = new 表示名の編集をまとめる係(
            表示名,
            new 楽曲名の変更の反映(this.表示名の知らせの口, this.操作),
        )
        this.同期 = new 楽曲編集の表示の同期(this.画面, this.状態, this.UI状態, this.表示名の編集)
        this.演奏 = new 演奏サービス(this.状態)
        this.接続 = 接続 === undefined ? new 実サーバー接続() : 接続
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%' }).child(this.画面)

        this.画面.発音配線.配線する(new 升目の発音(this.演奏))
        this._購読解除 = 楽曲編集イベントを配線する({
            画面: this.画面,
            状態: this.状態,
            UI状態: this.UI状態,
            操作: this.操作,
            接続: this.接続,
            演奏: this.演奏,
            同期: this.同期,
            表示名の編集: this.表示名の編集,
            楽曲ID,
        })
        void 起動時に楽曲を読み込む(this.画面, this.状態, this.接続, 楽曲ID, this.同期)
    }

    public 寸法を合わせる(): void {}

    public 前面になった(): void {}

    public 背面になった(): void {}

    public override delete(): void {
        this._購読解除()
        this.演奏.破棄する()
        this.画面.delete()
        super.delete()
    }
}
