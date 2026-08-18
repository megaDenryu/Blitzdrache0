import { div, LV2HtmlComponentBase, type DivC } from 'sengen-ui'
import type { ワールド編集状態 } from './編集モデル/index.ts'
import { 初期ワールド状態を生成する } from './初期ワールド生成.ts'
import { ワールド画面 } from './画面/index.ts'
import { ワールドエディター状態 } from './ワールドエディター状態.ts'
import { ワールドエディター同期サービス } from './ワールドエディター同期サービス.ts'
import { ワールドエディター操作サービス } from './ワールドエディター操作サービス.ts'
import { パネルイベントを配線する } from './ワールドエディター配線.ts'
import { ポインタとキー入力を配線する } from './ワールドエディターポインタ配線.ts'

// ワールドパイプラインエディターの編集モデル・画面・描画ループ・操作配線を統括するツールルート。
export class ワールドパイプラインエディター extends LV2HtmlComponentBase {
    protected _componentRoot: DivC
    public readonly 編集状態: ワールド編集状態
    public readonly 画面: ワールド画面
    public readonly UI状態: ワールドエディター状態
    public readonly 同期サービス: ワールドエディター同期サービス
    public readonly 操作サービス: ワールドエディター操作サービス
    private readonly _購読解除: () => void

    public constructor(初期状態?: ワールド編集状態) {
        super()
        this.編集状態 = 初期状態 ?? 初期ワールド状態を生成する()
        this.画面 = new ワールド画面(this.編集状態)
        this._componentRoot = div().setStyleCSS({ width: '100%', height: '100%', position: 'relative' }).child(this.画面)

        this.UI状態 = new ワールドエディター状態()
        this.同期サービス = new ワールドエディター同期サービス(this.編集状態, this.UI状態, this.画面.部品)
        this.操作サービス = new ワールドエディター操作サービス(this.編集状態, this.UI状態, this.同期サービス)

        パネルイベントを配線する(
            this.画面.部品,
            this.UI状態,
            this.操作サービス,
            this.同期サービス,
            this.編集状態,
        )
        this._購読解除 = ポインタとキー入力を配線する(
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
