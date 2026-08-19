import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
})

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    gap: '8px',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

// 見出しラベル(1つ目の子)は残り幅を占有して省略記号で収め、本数ラベル(2つ目の子)は
// 縮まず常に全文表示する。flex子は既定でminWidth:autoのため明示しないと、狭い枠内で
// 日本語文字が任意の位置で折り返される。
globalStyle(`${見出し行} > *:first-child`, {
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

globalStyle(`${見出し行} > *:last-child`, {
    flexShrink: 0,
    whiteSpace: 'nowrap',
})

export const 本数ラベル = style({
    fontFamily: 'monospace',
    color: エディターCSS変数('テキストコード'),
})

export const 再ベイクボタン = style({
    padding: '6px',
    fontSize: '11px',
    fontFamily: 'monospace',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト主'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    ':hover': {
        backgroundColor: エディターCSS変数('ボタンホバー背景'),
    },
})
