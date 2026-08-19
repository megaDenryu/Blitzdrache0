import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { プロジェクト保管庫接続 } from '../../境界/通信/index.ts'
import { 実サーバー接続 } from '../../境界/通信/index.ts'
import type { チャンク座標 } from '../../生成/編集資源契約.ts'
import { ワールド編集状態, 初期ワールド状態を生成する } from './編集モデル/index.ts'
import { ワールド画面 } from './画面/index.ts'
import type { インスペクターパネル } from './画面/パネル/インスペクター/インスペクターパネル.ts'
import { ワールドエディター状態 } from './ワールドエディター状態.ts'
import { ワールドエディター同期サービス } from './ワールドエディター同期サービス.ts'
import { ワールドエディター操作サービス } from './ワールドエディター操作サービス.ts'
import { パネルイベントを配線する } from './ワールドエディター配線.ts'
import { 永続化イベントを配線する, 起動時にワールドを読み込む } from './ワールドエディター永続化配線.ts'
import { ポインタとキー入力を配線する } from './ワールドエディターポインタ配線.ts'

// 開いたチャンク座標を受け取り、編集モデル・画面・描画ループ・操作配線を統括するツールルート。
export class ワールドパイプラインエディター extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 対象座標: チャンク座標
    public readonly 編集状態: ワールド編集状態
    public readonly 画面: ワールド画面
    public readonly インスペクター: インスペクターパネル
    public readonly UI状態: ワールドエディター状態
    public readonly 同期サービス: ワールドエディター同期サービス
    public readonly 操作サービス: ワールドエディター操作サービス
    public readonly 保管庫: プロジェクト保管庫接続
    private readonly _購読解除: () => void

    public constructor(
        対象座標: チャンク座標 = { x: 0, z: 0 },
        初期状態?: ワールド編集状態,
        保管庫?: プロジェクト保管庫接続,
    ) {
        super()
        this.対象座標 = { ...対象座標 }
        this.保管庫 = 保管庫 ?? new 実サーバー接続()
        this.編集状態 = 初期状態 ?? 初期ワールド状態を生成する(this.対象座標)
        this.編集状態.選択中チャンク座標 = this.対象座標
        this.画面 = new ワールド画面(this.編集状態)
        this.インスペクター = this.画面.部品.インスペクター
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.画面)

        this.UI状態 = new ワールドエディター状態(this.対象座標)
        this.同期サービス = new ワールドエディター同期サービス(this.編集状態, this.UI状態, this.画面.部品)
        this.操作サービス = new ワールドエディター操作サービス(this.編集状態, this.UI状態, this.同期サービス)

        パネルイベントを配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        const 永続化解除 = 永続化イベントを配線する(this.インスペクター.部品.永続化, this.保管庫, this.編集状態, this.同期サービス, this.対象座標)
        const ポインタ解除 = ポインタとキー入力を配線する(this.画面.部品, this.UI状態, this.操作サービス, this.同期サービス, this.編集状態)
        this._購読解除 = (): void => {
            ポインタ解除()
            永続化解除()
        }

        this.同期サービス.全体を同期する()
        if (初期状態 === undefined) {
            void 起動時にワールドを読み込む(this.インスペクター.部品.永続化, this.保管庫, this.編集状態, this.同期サービス, this.対象座標)
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

    public override delete(): void {
        this._購読解除()
        this.画面.delete()
        super.delete()
    }
}
