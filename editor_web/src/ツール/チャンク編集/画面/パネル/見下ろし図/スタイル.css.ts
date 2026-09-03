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
    boxShadow: エディターCSS変数('カード影'),
})

export const 見出し = style({
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const ボタン区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: `1px solid ${エディターCSS変数('境界線')}`,
})

export const アクションボタン = style({
    padding: '6px 8px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト主'),
    cursor: 'pointer',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})

// 一辺を変えると塗りが空になる、という取り返しの案内。読み飛ばされないよう本文より少し強い色にする。
export const 注意文 = style({
    fontSize: '10px',
    lineHeight: '1.4',
    color: エディターCSS変数('テキスト副'),
    textWrap: 'pretty',
})

// 下書きと正本の揃いの札。大きさの札と同じ中立トーンにし、文言だけで4つの状態を伝える。
export const 揃いの札 = style({
    alignSelf: 'flex-start',
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    backgroundColor: エディターCSS変数('中立バッジ背景'),
    color: エディターCSS変数('中立バッジ文字'),
    border: `1px solid ${エディターCSS変数('中立バッジ枠線')}`,
    whiteSpace: 'nowrap',
})

export const 選択欄 = style({
    width: '100%',
    padding: '3px 6px',
    fontSize: '11px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト主'),
    ':disabled': {
        opacity: 0.4,
        cursor: 'not-allowed',
    },
})
