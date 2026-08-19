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
    boxShadow: エディターCSS変数('カード影'),
})

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    gap: '8px',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

// 見出しラベル(1つ目の子)は残り幅を占有して省略記号で収め、バッジ(2つ目の子)は
// 縮まず常に全文表示する。flex子は既定でminWidth:autoのため明示しないと、狭い枠内で
// 日本語文字が任意の位置で折り返される(「129×129 (1画素重複)」の分断の原因)。
globalStyle(`${見出し行} > *:first-child`, {
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

// 情報表示のバッジであり、成功・活性の意味を持たないため中立トーンにする。
export const バッジ = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: エディターCSS変数('中立バッジ文字'),
    backgroundColor: エディターCSS変数('中立バッジ背景'),
    border: `1px solid ${エディターCSS変数('中立バッジ枠線')}`,
    borderRadius: '4px',
    padding: '1px 6px',
    whiteSpace: 'nowrap',
    flexShrink: 0,
})

export const 説明リスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    fontSize: '11px',
    lineHeight: '1.4',
    color: エディターCSS変数('テキスト薄'),
    textWrap: 'pretty',
})
