import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 右サイドバーへ収める建物の設定一式。エディタ領域とは別に、この枠の中だけが縦にスクロールする
// (設計正本の判断14)。
export const 建物インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    boxSizing: 'border-box',
    padding: '16px',
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
})

// 階は建物を積むほど増えるため、この一覧だけが自分の中で縦にスクロールする。
// 増えない設定と一緒にスクロールさせると、階を選ぶあいだに他の設定が画面から消える。
export const 階の一覧の巻き取り枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    maxHeight: '180px',
    overflowY: 'auto',
})

export const 階の1件 = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '8px',
    width: '100%',
    boxSizing: 'border-box',
})

// いま選んでいる升目の位置を綴りで出す札。選んでいないあいだも高さを保ち、
// 升目を選ぶたびに右サイドバーの並びが跳ねないようにする。
export const 選んでいる升目の札 = style({
    minHeight: '16px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
})
