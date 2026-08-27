import { div, span, DivC } from 'sengen-ui'
import { 全ての立体の役割, type 立体の役割 } from '../../編集モデル/index.ts'
import { 役割の識別色を引く } from './役割の識別色.ts'
import { 凡例の1件, 凡例の並び, 凡例の色見本 } from './三次元表示のスタイル.css.ts'

// どの色がどの役割かを並べる凡例。識別色の重ねを入れたときに、どの線の枠が何を表すかを読むためのものである。
export function 役割の凡例を作る(): DivC {
    return div({ class: 凡例の並び }).childs(全ての立体の役割.map(この役割の凡例を作る))
}

function この役割の凡例を作る(役割: 立体の役割): DivC {
    return div({ class: 凡例の1件 }).childs([
        div({ class: 凡例の色見本 }).setStyleCSS({ backgroundColor: 十六進の色綴りへ写す(役割の識別色を引く(役割)) }),
        span({ text: 役割 }),
    ])
}

// 三次元の材質が受け取る数の色を、CSSが読む綴りへ写す。色の値の正本は`役割の識別色`の1つである。
function 十六進の色綴りへ写す(色: number): string {
    return `#${色.toString(16).padStart(6, '0')}`
}
