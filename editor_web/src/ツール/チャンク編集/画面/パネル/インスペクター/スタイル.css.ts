import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// VSL右サイドバースロットに収まるインスペクターパネルのスタイル定義。
export const インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    backgroundColor: エディターCSS変数('パネル背景'),
    padding: '16px',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    color: エディターCSS変数('テキスト主'),
})

// タイトル行(全幅)とバッジ行(左寄せ)を縦に積む。同じ行で幅を奪い合わせると
// タイトルが省略され情報が欠けるため、バッジは見出しの下の行へ分離する。
export const ヘッダー = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
    paddingBottom: '10px',
})

// タイトルの折返し防止用の包み。nowrap+ellipsisは280px程度の
// サイドバー幅でも通常は全文が収まるが、極端な狭幅時の保険として残す。
export const タイトル群 = style({
    minWidth: 0,
    overflow: 'hidden',
})

export const タイトル = style({
    fontSize: '13px',
    fontWeight: 700,
    color: エディターCSS変数('アクセント文字'),
    letterSpacing: '0.05em',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

// 情報表示のバッジであり、成功・活性の意味を持たないため中立トーンにする。
// alignSelf:flex-startでヘッダー(縦積み)内の既定引き伸ばしを打ち消し、内容幅で左寄せにする。
export const バッジ = style({
    alignSelf: 'flex-start',
    padding: '2px 8px',
    borderRadius: '4px',
    fontSize: '10px',
    fontFamily: 'monospace',
    backgroundColor: エディターCSS変数('中立バッジ背景'),
    color: エディターCSS変数('中立バッジ文字'),
    border: `1px solid ${エディターCSS変数('中立バッジ枠線')}`,
    whiteSpace: 'nowrap',
    flexShrink: 0,
})
