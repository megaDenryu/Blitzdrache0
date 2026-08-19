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

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト副'),
})

export const 件数ラベル = style({
    fontFamily: 'monospace',
    color: エディターCSS変数('テキストコード'),
})

export const 生成ボタングリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '4px',
})

export const 生成ボタン = style({
    padding: '6px 2px',
    fontSize: '10px',
    fontWeight: 500,
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
})

export const アクション区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: `1px solid ${エディターCSS変数('境界線')}`,
})

export const 平坦化ボタン = style({
    padding: '8px',
    fontSize: '11px',
    fontWeight: 600,
    borderRadius: '4px',
    border: 'none',
    backgroundColor: エディターCSS変数('アクセント背景'),
    color: エディターCSS変数('アクセント文字白'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': { backgroundColor: エディターCSS変数('アクセントホバー') },
})

export const 行ボタン群 = style({
    display: 'flex',
    gap: '8px',
})

export const 接地ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('ボタン枠線')}`,
    backgroundColor: エディターCSS変数('ボタン背景'),
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    ':hover': { backgroundColor: エディターCSS変数('ボタンホバー背景') },
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
    ':disabled': { opacity: 0.3, cursor: 'not-allowed' },
    ':hover': { backgroundColor: エディターCSS変数('危険ボタンホバー') },
})
