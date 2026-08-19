import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { プロジェクト保管庫接続, ソースアセット書き出し接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import type { ワールド編集状態 } from '../チャンク編集/編集モデル/index.ts'
import { 初期大域ワールド状態を生成する } from './初期大域ワールド生成.ts'
import { 大域編集画面 } from './画面/index.ts'
import type { 大域インスペクターパネル } from './画面/パネル/インスペクター/index.ts'
import { 大域編集状態 } from './大域編集状態.ts'
import { 大域編集同期サービス } from './大域編集同期サービス.ts'
import { 大域編集操作サービス } from './大域編集操作サービス.ts'
import { 大域パネルイベントを配線する } from './大域編集配線.ts'
import { 大域書き出しイベントを配線する } from './大域編集書き出し配線.ts'
import { 大域永続化イベントを配線する, 起動時に大域ワールドを読み込む } from './大域編集永続化配線.ts'
import { 大域ポインタとキー入力を配線する } from './大域編集ポインタ配線.ts'

// 1024m四方の大域造成と広域道路を統括する大域編集ツールのツールルート。
export class 大域編集ツール extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 編集状態: ワールド編集状態
    public readonly 画面: 大域編集画面
    public readonly インスペクター: 大域インスペクターパネル
    public readonly UI状態: 大域編集状態
    public readonly 同期サービス: 大域編集同期サービス
    public readonly 操作サービス: 大域編集操作サービス
    public readonly 保管庫: プロジェクト保管庫接続
    public readonly 書き出し接続: ソースアセット書き出し接続
    private readonly _購読解除: () => void

    public constructor(
        初期状態?: ワールド編集状態,
        保管庫?: プロジェクト保管庫接続,
        書き出し接続?: ソースアセット書き出し接続,
    ) {
        super()
        this.保管庫 = 保管庫 ?? new 実サーバー接続()
        this.書き出し接続 = 書き出し接続 ?? new 実サーバー接続()
        this.編集状態 = 初期状態 ?? 初期大域ワールド状態を生成する()
        this.画面 = new 大域編集画面(this.編集状態)
        this.インスペクター = this.画面.部品.インスペクター
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.画面)

        this.UI状態 = new 大域編集状態()
        this.同期サービス = new 大域編集同期サービス(this.編集状態, this.UI状態, this.画面.部品)
        this.操作サービス = new 大域編集操作サービス(this.編集状態, this.UI状態, this.同期サービス)

        大域パネルイベントを配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        大域書き出しイベントを配線する(this.インスペクター.部品.スライス, this.書き出し接続)
        const 永続化解除 = 大域永続化イベントを配線する(this.インスペクター.部品.永続化, this.保管庫, this.編集状態, this.同期サービス)
        const ポインタ解除 = 大域ポインタとキー入力を配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        this._購読解除 = (): void => {
            ポインタ解除()
            永続化解除()
        }

        this.同期サービス.全体を同期する()
        if (初期状態 === undefined) {
            void 起動時に大域ワールドを読み込む(this.インスペクター.部品.永続化, this.保管庫, this.編集状態, this.同期サービス)
        }
    }

    public 前面になった(): void {
        this.画面.部品.三次元ビュー.描画ループ.開始する()
    }

    public 背面になった(): void {
        this.画面.部品.三次元ビュー.描画ループ.停止する()
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.画面.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    public テーマを適用する(テーマ: {
        readonly 大域三次元背景色: number
        readonly 地形基本色: number
        readonly 道路色: number
    }): void {
        this.画面.背景色を設定する(テーマ.大域三次元背景色)
        this.画面.地形色を設定する(テーマ.地形基本色)
        this.画面.道路色を設定する(テーマ.道路色)
    }

    public override delete(): void {
        this._購読解除()
        this.画面.delete()
        super.delete()
    }
}
