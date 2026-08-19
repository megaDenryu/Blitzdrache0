import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../../境界/index.ts'

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

// 見出しラベル行(全幅)とバッジ行(左寄せ)を縦に積む。同じ行で幅を奪い合わせると
// 見出しが省略され情報が欠けるため、バッジは見出しの下の行へ分離する。
export const 見出し行 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

// 見出しラベル(1つ目の子)。nowrap+ellipsisは280px程度のサイドバー幅でも通常は
// 全文が収まるが、極端な狭幅時の保険として残す。
globalStyle(`${見出し行} > *:first-child`, {
    minWidth: 0,
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

// 情報表示のバッジであり、成功・活性の意味を持たないため中立トーンにする。
// alignSelf:flex-startでヘッダー(縦積み)内の既定引き伸ばしを打ち消し、内容幅で左寄せにする。
export const バッジ = style({
    alignSelf: 'flex-start',
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
