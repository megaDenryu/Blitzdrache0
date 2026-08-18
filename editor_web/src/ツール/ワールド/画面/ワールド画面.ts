import { div, DivC, LV2部品集約Base } from 'sengen-ui'
import type { ワールド編集状態 } from '../編集モデル/index.ts'
import { ワールド画面部品 } from './ワールド画面部品.ts'
import { 画面ルート } from './スタイル.css.ts'

// 三次元ビューとインスペクターパネルを画面上に統合配置するLV2部品集約Orchestrator。
export class ワールド画面 extends LV2部品集約Base<ワールド画面部品> {
    protected _componentRoot: DivC
    public readonly 部品: ワールド画面部品

    public constructor(編集状態: ワールド編集状態) {
        super()
        this.部品 = ワールド画面部品.作る(編集状態)
        this._componentRoot = this._ルートを構築する(this.部品)
    }

    public 寸法を合わせる(幅: number, 高さ: number, ピクセル比: number = 1): void {
        this.部品.三次元ビュー.寸法を合わせる(幅, 高さ, ピクセル比)
    }

    protected _ルートを構築する(部品: ワールド画面部品): DivC {
        return (
            div({ class: 画面ルート }).childs([
                部品.三次元ビュー,
                部品.インスペクター])
        )
    }

    public override delete(): void {
        this.部品.三次元ビュー.delete()
        this.部品.インスペクター.delete()
        super.delete()
    }
}
