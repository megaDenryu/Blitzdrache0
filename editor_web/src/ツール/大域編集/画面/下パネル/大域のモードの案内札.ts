import { DivC } from 'sengen-ui'
import { 棚の案内文 } from '../../../チャンク編集/画面/パネル/共通/棚.css.ts'
import { 大域モードヒント写像, type 大域編集モード } from '../パネル/モード切替/大域モード定義.ts'

// 視点の操作は全モードで同じであるため、モードごとの案内とは別に1行で常に添える(判断13)。
const 視点の案内 = '視点は右ドラッグで回し、中ドラッグで平行移動し、ホイールで寄り引きする。Altで次のモード、Shift+Altで前のモードへ移る。'

// いまのモードで左ボタンが何をするかを棚の一番上へ出す札(LV1拡張)。
// モードを変えるたびに文言が変わるため、書き換えをこのクラスへ閉じる。
export class 大域のモードの案内札 extends DivC {
    public constructor(初期モード: 大域編集モード) {
        super({ class: 棚の案内文 })
        this.モードを更新する(初期モード)
    }

    public モードを更新する(モード: 大域編集モード): this {
        const 文言 = `${大域モードヒント写像[モード]} ${視点の案内}`
        this.setTextContent(文言)
        this.setTooltip(文言)
        return this
    }
}
