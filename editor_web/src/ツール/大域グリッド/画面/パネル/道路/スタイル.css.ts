import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
})

export const 見出し = style({
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const アクション区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: `1px solid ${エディターCSS変数('境界線')}`,
})

export const 行ボタン群 = style({
    display: 'flex',
    gap: '8px',
})

export const 削除ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('危険ボタン枠線')}`,
    backgroundColor: エディターCSS変数('危険ボタン背景'),
    color: エディターCSS変数('危険ボタン文字'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('危険ボタンホバー'),
    },
    ':disabled': {
        opacity: 0.3,
        cursor: 'not-allowed',
    },
})

export const 副ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})
