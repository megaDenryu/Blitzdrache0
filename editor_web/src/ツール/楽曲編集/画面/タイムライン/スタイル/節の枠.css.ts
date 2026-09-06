import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 同じ節に属するカードを括る節の枠(判断15の是正、issue #88)。
export const 節の枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    flexShrink: 0,
    padding: '4px',
    borderRadius: '4px',
    border: `1px dashed ${エディターCSS変数('カード枠線')}`,
})

export const 節の枠見出し = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '4px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
})

export const 節の枠見出し文言 = style({
    whiteSpace: 'nowrap',
})

export const 節の枠見出し操作 = style({
    display: 'flex',
    gap: '2px',
})

export const 節の枠カード列 = style({
    display: 'flex',
    gap: '8px',
})
