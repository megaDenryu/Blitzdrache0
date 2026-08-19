import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const 行コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
})

export const ラベル行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    gap: '8px',
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
})

// 項目名(1つ目の子)は残り幅を占有して省略記号で収め、値ラベル(2つ目の子)は縮まず
// 常に全文表示する。flex子は既定でminWidth:autoのため明示しないと、狭い枠内で
// 日本語文字が任意の位置で折り返される。
globalStyle(`${ラベル行} > *:first-child`, {
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const 値ラベル = style({
    fontFamily: 'monospace',
    color: エディターCSS変数('テキストコード'),
    whiteSpace: 'nowrap',
    flexShrink: 0,
})

export const スライダー入力 = style({
    width: '100%',
    accentColor: エディターCSS変数('アクセントホバー'),
    cursor: 'pointer',
})
