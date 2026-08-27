import { div, span, DivC } from 'sengen-ui'
import { 木項目, アイコン } from '../スタイル.css.ts'

// 子を持たないエクスプローラーの項目(大域世界・マテリアル・使い方)を1つ作る純粋関数
// (自由関数の許容2条件(a): 依存も副作用も持たない。返す部品の所有はパネルが持つ)。
export function 単一項目ノードを作る(頭文字: string, ラベル: string, 押されたら: () => void): DivC {
    return div({ class: 木項目 })
        .childs([span({ class: アイコン, text: 頭文字 }), span({ text: ラベル }).setTooltip(ラベル)])
        .onClick(押されたら)
}
