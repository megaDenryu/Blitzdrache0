import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 下パネルへ収める、これから配置する部品の棚。横長の区画であるため、筆の並びを横へ広げて
// 一度に見渡せるようにする(設計正本の判断14)。
export const 棚の枠 = style({
    width: '100%',
    height: '100%',
    boxSizing: 'border-box',
    padding: '10px 14px',
    overflowY: 'auto',
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
})

// 筆の並びは種類が増えるほど横へ伸びるため、この列だけが横にスクロールする。
export const 棚の列 = style({
    display: 'flex',
    alignItems: 'flex-start',
    gap: '20px',
    flexWrap: 'wrap',
})

export const 棚の1組 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    minWidth: 0,
})
