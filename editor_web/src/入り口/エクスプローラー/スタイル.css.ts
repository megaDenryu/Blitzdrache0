import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../テーマ/テーマ変数.ts'

// 左サイドバーのエクスプローラー木ビューのスタイル定義。
// テーマCSS変数により各配色トークンを参照し、WCAG AA(4.5:1以上)のコントラスト比を保証する。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '8px 0',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('サイドバー背景'),
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: エディターCSS変数('テキスト薄'),
    padding: '6px 12px 2px 12px',
    letterSpacing: '0.5px',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const 木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px',
    fontSize: '13px',
    color: エディターCSS変数('テキスト主'),
    cursor: 'pointer',
    borderRadius: '3px',
    margin: '1px 4px',
    transition: 'background-color 0.1s',
    selectors: {
        '&:hover': {
            backgroundColor: エディターCSS変数('項目ホバー背景'),
            color: エディターCSS変数('項目ホバー文字'),
        },
        '&[data-selected="true"]': {
            backgroundColor: エディターCSS変数('選択背景'),
            color: エディターCSS変数('選択文字'),
            fontWeight: 'bold',
        },
    },
})

export const 子木項目コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
})

export const 子木項目 = style({
    display: 'flex',
    alignItems: 'center',
    padding: '4px 12px 4px 28px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
    cursor: 'pointer',
    borderRadius: '3px',
    margin: '1px 4px',
    transition: 'background-color 0.1s',
    selectors: {
        '&:hover': {
            backgroundColor: エディターCSS変数('項目ホバー背景'),
            color: エディターCSS変数('項目ホバー文字'),
        },
        '&[data-selected="true"]': {
            backgroundColor: エディターCSS変数('選択背景'),
            color: エディターCSS変数('選択文字'),
            fontWeight: 'bold',
        },
    },
})

// 木の項目名(アイコンに続く2つ目の子)は1行物のため折返しを許さない。幅が足りない場合は
// 末尾を省略記号で収める(木項目・子木項目のどちらも構成はアイコン+ラベルの2要素で固定)。
globalStyle(`${木項目} > *:last-child`, {
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

globalStyle(`${子木項目} > *:last-child`, {
    minWidth: 0,
    flex: '1',
    whiteSpace: 'nowrap',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
})

export const アイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: エディターCSS変数('アイコン色'),
})

export const フォルダアイコン = style({
    marginRight: '6px',
    fontSize: '12px',
    width: '16px',
    textAlign: 'center',
    color: エディターCSS変数('フォルダアイコン色'),
})
