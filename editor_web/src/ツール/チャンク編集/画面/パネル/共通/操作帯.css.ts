import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// エディタ領域の上部の固定の行へ収める操作帯。編集対象の名前・大きさの札・モードの切替・
// 対象の全体に効く操作を1行へ並べ、名前を出すためだけの行を作らない(設計正本の判断14)。
export const 操作帯の枠 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '12px',
    flexWrap: 'wrap',
})

export const 対象の名前 = style({
    fontSize: '13px',
    fontWeight: 700,
    color: エディターCSS変数('アクセント文字'),
    letterSpacing: '0.05em',
    whiteSpace: 'nowrap',
})

// 大きさの案内であり、成功や活性の意味を持たないため中立トーンにする。
export const 対象の大きさの札 = style({
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    fontFamily: 'monospace',
    backgroundColor: エディターCSS変数('中立バッジ背景'),
    color: エディターCSS変数('中立バッジ文字'),
    border: `1px solid ${エディターCSS変数('中立バッジ枠線')}`,
    whiteSpace: 'nowrap',
})

export const 操作ボタン = style({
    padding: '4px 10px',
    fontSize: '11px',
    borderRadius: '4px',
    cursor: 'pointer',
    whiteSpace: 'nowrap',
    color: エディターCSS変数('ボタン文字'),
    backgroundColor: エディターCSS変数('ボタン背景'),
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
    ':disabled': {
        opacity: 0.3,
        cursor: 'not-allowed',
    },
})
