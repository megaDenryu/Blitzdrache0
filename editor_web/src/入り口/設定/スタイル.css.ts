import { style, globalStyle } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../テーマ/テーマ変数.ts'

// 設定・テーマ選択パネルのスタイル定義。
// 全てエディターテーマCSS変数を参照し、直書き色を持たない。
export const コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    width: '100%',
    height: '100%',
    padding: '12px',
    overflowY: 'auto',
    userSelect: 'none',
    boxSizing: 'border-box',
    backgroundColor: エディターCSS変数('サイドバー背景'),
    color: エディターCSS変数('テキスト主'),
    gap: '16px',
})

export const セクション見出し = style({
    fontSize: '11px',
    fontWeight: 'bold',
    textTransform: 'uppercase',
    color: エディターCSS変数('テキスト薄'),
    letterSpacing: '0.5px',
})

export const テーマリスト = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
})

export const テーマカード = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    borderRadius: '6px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('カード背景'),
    backdropFilter: エディターCSS変数('ガラス背景ぼかし'),
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        borderColor: エディターCSS変数('アクセントホバー'),
        backgroundColor: エディターCSS変数('項目ホバー背景'),
    },
})

globalStyle(`${テーマカード}[data-selected="true"]`, {
    borderColor: エディターCSS変数('選択枠線'),
    backgroundColor: エディターCSS変数('選択背景'),
})

export const カードヘッダー = style({
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'space-between',
})

export const テーマ表示名 = style({
    fontSize: '13px',
    fontWeight: 600,
    color: エディターCSS変数('テキスト主'),
})

export const 選択バッジ = style({
    fontSize: '10px',
    padding: '2px 6px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('アクセント背景'),
    color: エディターCSS変数('アクセント文字白'),
    fontWeight: 'bold',
})

export const テーマ説明 = style({
    fontSize: '11px',
    color: エディターCSS変数('テキスト副'),
    lineHeight: '1.4',
})

export const パレットプレビュー = style({
    display: 'flex',
    gap: '4px',
    alignItems: 'center',
    marginTop: '4px',
})

export const 色スウォッチ = style({
    width: '16px',
    height: '16px',
    borderRadius: '3px',
    border: `1px solid ${エディターCSS変数('境界線薄')}`,
})
