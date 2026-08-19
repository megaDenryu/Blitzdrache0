import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { チャンク座標 } from '../../../生成/編集資源契約.ts'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { チャンク編集画面部品 } from './チャンク編集画面部品.ts'
import type { 三次元の配色 } from './三次元/三次元の配色.ts'
import { 画面ルート } from './スタイル.css.ts'

// 三次元ビューを画面いっぱいに配置し、インスペクター部品を保持するLV2部品集約Orchestrator。
export class チャンク編集画面 extends LV2部品集約Base<チャンク編集画面部品> {
    protected _componentRoot: DivC
    public readonly 部品: チャンク編集画面部品

    public constructor(編集状態: ワールド編集状態, 対象座標: チャンク座標) {
        super()
        this.部品 = チャンク編集画面部品.作る(編集状態, 対象座標)
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.部品.三次元ビュー.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    public 背景色を設定する(色: string | number): void {
        this.部品.三次元ビュー.背景色を設定する(色)
    }

    public 三次元の配色を設定する(配色: 三次元の配色): void {
        this.部品.三次元ビュー.三次元の配色を設定する(配色)
    }

    protected _ルートを構築する(部品: チャンク編集画面部品): DivC {
        return div({ class: 画面ルート }).childs([部品.三次元ビュー])
    }

    public override delete(): void {
        this.部品.三次元ビュー.delete()
        this.部品.インスペクター.delete()
        super.delete()
    }
}
