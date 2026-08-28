import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { プロジェクト保管庫接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import type { チャンク座標 } from '../../生成/編集資源契約.ts'
import { ワールド編集状態, 初期ワールド状態を生成する } from './編集モデル/index.ts'
import { チャンク編集画面, type インスペクターパネル, type 筆と置くものの棚 } from './画面/index.ts'
import { 色トークンから三次元の配色を作る, type 三次元の色トークン } from './画面/三次元/三次元の配色.ts'
import { チャンク編集状態 } from './チャンク編集状態.ts'
import { チャンク編集同期サービス } from './チャンク編集同期サービス.ts'
import { チャンク編集操作サービス } from './チャンク編集操作サービス.ts'
import { 建物外形カタログを注入する } from './チャンク編集建物カタログ配線.ts'
import { 地表材質色を注入する } from './チャンク編集地表材質色配線.ts'
import { パネルイベントを配線する } from './チャンク編集配線.ts'
import { 永続化イベントを配線する, 起動時にワールドを読み込む } from './チャンク編集永続化配線.ts'
import { ポインタとキー入力を配線する } from './チャンク編集ポインタ配線.ts'
import { ワールド永続化サービス } from './操作コマンド/index.ts'

// 開いたチャンク座標を受け取り、編集モデル・画面・描画ループ・操作配線を統括するツールルート。
export class チャンク編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 対象座標: チャンク座標
    public readonly 編集状態: ワールド編集状態
    public readonly 画面: チャンク編集画面
    public readonly インスペクター: インスペクターパネル
    public readonly UI状態: チャンク編集状態
    public readonly 同期サービス: チャンク編集同期サービス
    public readonly 操作サービス: チャンク編集操作サービス
    public readonly 保管庫: プロジェクト保管庫接続
    public readonly 永続化: ワールド永続化サービス
    private readonly _購読解除: () => void

    public constructor(
        対象座標: チャンク座標 = { x: 0, z: 0 },
        初期状態?: ワールド編集状態,
        保管庫?: プロジェクト保管庫接続,
    ) {
        super()
        this.対象座標 = { ...対象座標 }
        this.保管庫 = 保管庫 ?? new 実サーバー接続()
        this.永続化 = new ワールド永続化サービス(this.保管庫)
        this.編集状態 = 初期状態 ?? 初期ワールド状態を生成する(this.対象座標)
        this.編集状態.選択中チャンク座標 = this.対象座標
        this.画面 = new チャンク編集画面(this.編集状態, this.対象座標)
        this.インスペクター = this.画面.部品.インスペクター
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.画面)

        this.UI状態 = new チャンク編集状態(this.対象座標)
        this.同期サービス = new チャンク編集同期サービス(this.編集状態, this.UI状態, this.画面.部品)
        this.操作サービス = new チャンク編集操作サービス(this.編集状態, this.UI状態, this.同期サービス)

        パネルイベントを配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        const 永続化解除 = 永続化イベントを配線する(this.インスペクター.部品.永続化, this.永続化, this.保管庫, this.編集状態, this.同期サービス, this.対象座標)
        const ポインタ解除 = ポインタとキー入力を配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        this._購読解除 = (): void => {
            ポインタ解除()
            永続化解除()
        }

        this.同期サービス.全体を同期する()
        if (初期状態 === undefined) {
            void 起動時にワールドを読み込む(this.インスペクター.部品.永続化, this.永続化, this.編集状態, this.同期サービス, this.対象座標)
        }
        void 地表材質色を注入する(this.画面, this.保管庫)
        void 建物外形カタログを注入する(this)
    }

    // これから使う筆と置くものの棚を下パネルへ渡す。外殻はこの口が在るツールでだけ下パネルを開く。
    public get 下パネル(): 筆と置くものの棚 {
        return this.画面.部品.棚
    }

    public 前面になった(): void {
        this.画面.部品.三次元ビュー.描画ループ.開始する()
    }

    public 背面になった(): void {
        this.画面.部品.三次元ビュー.描画ループ.停止する()
    }

    // 幅と高さを使わないのは、三次元がエディタ領域の中で高さの決まった箱を埋めるからである。
    // 箱の大きさはCSSが決めるため、画面が自分のキャンバスを測る。ピクセル比だけを外殻から受け取る。
    public 寸法を合わせる(_幅: number, _高さ: number, ピクセル比: number = 1): void {
        this.画面.寸法を合わせる(ピクセル比)
    }

    public テーマを適用する(テーマ: 三次元の色トークン & { readonly 三次元背景色: number }): void {
        this.画面.背景色を設定する(テーマ.三次元背景色)
        this.画面.三次元の配色を設定する(色トークンから三次元の配色を作る(テーマ))
    }

    public override delete(): void {
        this._購読解除()
        this.画面.delete()
        super.delete()
    }
}
