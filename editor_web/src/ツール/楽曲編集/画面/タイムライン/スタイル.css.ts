import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// タイムライン部品の内側の行。人が編集中ずっと見る対象であるため縦には伸ばさない
// (設計正本の判断15)。見出しは固定し、カードの列だけを横にスクロールさせる
// (エディター制作スキル第7条「文脈は固定する」)。画面直下の「タイムラインの行」
// (画面/スタイル.css.ts)は、この部品を配置する外側の枠であり別の階層である。
export const タイムライン内側行 = style({
    display: 'flex',
    alignItems: 'stretch',
    gap: '8px',
    flexShrink: 0,
    padding: '8px 0',
})

export const タイムライン見出し = style({
    flexShrink: 0,
    alignSelf: 'center',
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
    whiteSpace: 'nowrap',
})

export const タイムラインのカード列域 = style({
    display: 'flex',
    alignItems: 'stretch',
    gap: '8px',
    overflowX: 'auto',
    overflowY: 'hidden',
    flex: 1,
    minWidth: 0,
})

export const 案内文 = style({
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
    display: 'flex',
    alignItems: 'center',
    padding: '4px 8px',
})

export const 末尾へ追加ボタンの配置 = style({
    flexShrink: 0,
    alignSelf: 'center',
})

export const カード枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    minWidth: '96px',
    padding: '6px 10px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    backgroundColor: エディターCSS変数('カード背景'),
    cursor: 'pointer',
    flexShrink: 0,
    selectors: {
        '&[data-節偶奇="1"]': { backgroundColor: エディターCSS変数('カード不透明背景') },
        '&[data-選択中="true"]': {
            borderColor: エディターCSS変数('選択枠線'),
            backgroundColor: エディターCSS変数('選択背景'),
        },
        '&[data-再生中="true"]': { outline: `2px solid ${エディターCSS変数('アクセント文字')}` },
    },
})

export const カードのパターン名 = style({
    fontSize: '12px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
    selectors: {
        [`${カード枠}[data-選択中="true"] &`]: { color: エディターCSS変数('選択文字') },
    },
})

export const カードの小節番号 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
    selectors: {
        [`${カード枠}[data-選択中="true"] &`]: { color: エディターCSS変数('選択文字') },
    },
})

export const 繰り返し中の印 = style({
    fontSize: '10px',
    color: エディターCSS変数('アクセント文字'),
    fontWeight: 700,
})

export const 操作ボタン行 = style({
    display: 'flex',
    flexWrap: 'wrap',
    gap: '2px',
    marginTop: '2px',
})

// 同じ節に属するカードを括る節の枠(判断15の是正、issue #88)。
export const 節の枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
    flexShrink: 0,
    padding: '4px',
    borderRadius: '4px',
    border: `1px dashed ${エディターCSS変数('カード枠線')}`,
})

export const 節の枠見出し = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
    gap: '4px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
})

export const 節の枠見出し文言 = style({
    whiteSpace: 'nowrap',
})

export const 節の枠見出し操作 = style({
    display: 'flex',
    gap: '2px',
})

export const 節の枠カード列 = style({
    display: 'flex',
    gap: '8px',
})
