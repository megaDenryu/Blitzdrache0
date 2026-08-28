import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 選ぶボタンを見出しの下へ並べる小さな組。筆の棚・階の一覧・入口の向きが同じ姿で並ぶように、
// この3つのパネルが同じスタイルを参照する。
export const セクション = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
})

export const セクション見出し = style({
    fontSize: '12px',
    fontWeight: 700,
    color: エディターCSS変数('テキスト副'),
})

export const 説明文 = style({
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
})

export const 横並び = style({
    display: 'flex',
    alignItems: 'center',
    gap: '6px',
    flexWrap: 'wrap',
})

export const 選択ボタン = style({
    padding: '4px 10px',
    fontSize: '12px',
    borderRadius: '3px',
    cursor: 'pointer',
    whiteSpace: 'nowrap',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    selectors: {
        '&[data-selected="true"]': {
            backgroundColor: エディターCSS変数('選択背景'),
            borderColor: エディターCSS変数('選択枠線'),
            color: エディターCSS変数('選択文字'),
        },
    },
})
