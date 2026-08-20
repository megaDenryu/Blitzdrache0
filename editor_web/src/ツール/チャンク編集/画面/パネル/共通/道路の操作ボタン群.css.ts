import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const ボタン群の枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
})

export const 行ボタン群 = style({
    display: 'flex',
    gap: '8px',
})

export const 対象の道ラベル = style({
    fontSize: '11px',
    fontFamily: 'monospace',
    color: エディターCSS変数('テキストコード'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

const 共通のボタン = {
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
} as const

export const 削除ボタン = style({
    ...共通のボタン,
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    color: エディターCSS変数('危険ボタン文字'),
    ':hover': {
        backgroundColor: エディターCSS変数('危険ボタンホバー'),
    },
    ':disabled': {
        opacity: 0.3,
        cursor: 'not-allowed',
    },
})

export const 副ボタン = style({
    ...共通のボタン,
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('ボタン文字'),
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
    ':disabled': {
        opacity: 0.3,
        cursor: 'not-allowed',
    },
})
