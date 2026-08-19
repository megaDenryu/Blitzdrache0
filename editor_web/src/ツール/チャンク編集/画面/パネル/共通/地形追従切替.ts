import { span, checkbox, DivC, CheckboxInputC } from 'sengen-ui'
import { チェック行, チェック入力 } from './スタイル.css.ts'

const 説明文 = 'オンのとき、WASDQE移動・回転・パンの後に注視点とカメラの高さを地形へ合わせます。E/Qは地形からの高さを上下する意味になります。'

// カメラを地形の起伏へ沿わせるかどうかを切り替えるチェックボックス(LV1拡張)。
// カメラ操作は全モード共通のため、両ツールのモード切替パネルの操作ヒント近くへ常設する。
export class 地形追従切替 extends DivC {
    private readonly _チェック: CheckboxInputC

    public constructor(初期有効: boolean = false) {
        super({ class: チェック行 })
        this._チェック = checkbox({ class: チェック入力, checked: 初期有効 })
        this.childs([
            this._チェック,
            span({ text: 'カメラを地形に沿わせる' }).setTooltip(説明文)])
    }

    public 切替時(コールバック: (有効: boolean) => void): this {
        this._チェック.onCheckChange(コールバック)
        return this
    }

    public 有効か(): boolean {
        return this._チェック.isChecked()
    }
}
