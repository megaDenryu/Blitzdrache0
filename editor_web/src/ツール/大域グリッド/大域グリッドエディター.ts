import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { ワールド編集状態 } from '../ワールド/編集モデル/index.ts'
import { 初期大域ワールド状態を生成する } from './初期大域ワールド生成.ts'
import { 大域グリッド画面 } from './画面/index.ts'
import { 大域グリッド状態 } from './大域グリッド状態.ts'
import { 大域グリッド同期サービス } from './大域グリッド同期サービス.ts'
import { 大域グリッド操作サービス } from './大域グリッド操作サービス.ts'
import { 大域パネルイベントを配線する } from './大域グリッド配線.ts'
import { 大域ポインタとキー入力を配線する } from './大域グリッドポインタ配線.ts'

// 1024m四方の大域造成と広域道路を統括する大域グリッドマネージャーのツールルート。
export class 大域グリッドエディター extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 編集状態: ワールド編集状態
    public readonly 画面: 大域グリッド画面
    public readonly UI状態: 大域グリッド状態
    public readonly 同期サービス: 大域グリッド同期サービス
    public readonly 操作サービス: 大域グリッド操作サービス
    private readonly _購読解除: () => void

    public constructor(初期状態?: ワールド編集状態) {
        super()
        this.編集状態 = 初期状態 ?? 初期大域ワールド状態を生成する()
        this.画面 = new 大域グリッド画面(this.編集状態)
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.画面)

        this.UI状態 = new 大域グリッド状態()
        this.同期サービス = new 大域グリッド同期サービス(this.編集状態, this.UI状態, this.画面.部品)
        this.操作サービス = new 大域グリッド操作サービス(this.編集状態, this.UI状態, this.同期サービス)

        大域パネルイベントを配線する(
            this.画面.部品,
            this.UI状態,
            this.操作サービス,
            this.同期サービス,
            this.編集状態,
        )
        this._購読解除 = 大域ポインタとキー入力を配線する(
            this.画面.部品,
            this.UI状態,
            this.操作サービス,
            this.同期サービス,
            this.編集状態,
        )

        this.同期サービス.全体を同期する()
        this.画面.部品.三次元ビュー.描画ループ.開始する()
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.画面.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    public override delete(): void {
        this._購読解除()
        this.画面.部品.三次元ビュー.描画ループ.停止する()
        this.画面.delete()
        super.delete()
    }
}
