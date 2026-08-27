import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

export const ラベル行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    gap: '8px',
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
})

// 項目名(1つ目の子)は残り幅を占有して省略記号で収め、値ラベル(2つ目の子)は縮まず常に全文表示する。
// flex子は既定でminWidth:autoのため、明示しないと狭い枠内で日本語文字が任意の位置で折り返される。
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

export const テキスト入力 = style({
    padding: '6px 10px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    boxSizing: 'border-box',
    width: '100%',
})

export const 選択セレクト = style({
    padding: '4px 8px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
    cursor: 'pointer',
    boxSizing: 'border-box',
    width: '100%',
})

// 横並びの行に置くセレクトへ重ねて、残り幅を占有させる修飾。
export const 幅を伸ばすセレクト = style({
    flex: '1',
    minWidth: '160px',
    width: 'auto',
})
