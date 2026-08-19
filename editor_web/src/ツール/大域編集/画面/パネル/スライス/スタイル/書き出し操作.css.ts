import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

export const 書き出し行 = style({
    display: 'flex',
    gap: '6px',
    marginTop: '4px',
})

export const 世界名入力 = style({
    flex: 1,
    minWidth: 0,
    padding: '5px 8px',
    fontSize: '11px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
})

export const 書き出しボタン = style({
    padding: '5px 10px',
    fontSize: '11px',
    fontWeight: 600,
    whiteSpace: 'nowrap',
    color: エディターCSS変数('プライマリボタン文字'),
    backgroundColor: エディターCSS変数('プライマリボタン背景'),
    border: `1px solid ${エディターCSS変数('プライマリボタン枠線')}`,
    borderRadius: '3px',
    cursor: 'pointer',
    ':hover': {
        backgroundColor: エディターCSS変数('プライマリボタンホバー'),
    },
    ':disabled': {
        backgroundColor: エディターCSS変数('非活性背景'),
        color: エディターCSS変数('非活性文字'),
        cursor: 'not-allowed',
    },
})

export const 書き出し状態メッセージ = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    minHeight: '16px',
})

export const 書き出しエラー状態メッセージ = style({
    fontSize: '11px',
    color: エディターCSS変数('危険ボタン文字'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    minHeight: '16px',
})
