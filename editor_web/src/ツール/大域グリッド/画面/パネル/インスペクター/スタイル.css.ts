import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// VSL右サイドバースロットに収まる大域インスペクターパネルのスタイル定義。
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

export const ヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '8px',
    borderBottom: `1px solid ${エディターCSS変数('境界線')}`,
    paddingBottom: '10px',
})

// タイトル・サブタイトルの折返し防止分だけヘッダーの残り幅を占有し、バッジ側を圧迫しない
// ようにするための包み。flexコンテナの子は既定でminWidth:autoのため、これが無いと
// 日本語文字が任意の位置で折り返されて読めなくなる(ellipsisも効かない)。
export const タイトル群 = style({
    minWidth: 0,
    flex: '1',
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

export const サブタイトル = style({
    fontSize: '10px',
    fontFamily: 'monospace',
    color: エディターCSS変数('テキスト薄'),
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

// 情報表示のバッジであり、成功・活性の意味を持たないため中立トーンにする。
export const バッジ = style({
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
