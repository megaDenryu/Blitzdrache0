import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 演奏の操作帯は格子のすぐ上に常設する。パネルの奥へ入れず、開閉もしない。
export const 操作帯枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '10px 12px',
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    borderRadius: '6px',
})

export const 操作帯の行 = style({
    display: 'flex',
    alignItems: 'center',
    flexWrap: 'wrap',
    gap: '10px',
})

export const 拍毎分の欄 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '8px',
    minWidth: '260px',
    flex: 1,
})

export const 拍毎分のつまみ = style({
    flex: 1,
    minWidth: '120px',
    accentColor: エディターCSS変数('アクセント背景'),
})

export const 項目の名前 = style({
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
    whiteSpace: 'nowrap',
})

export const 位置の表示 = style({
    fontFamily: 'monospace',
    fontSize: '12px',
    padding: '3px 8px',
    borderRadius: '4px',
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線薄')}`,
    color: エディターCSS変数('テキスト主'),
    minWidth: '150px',
    textAlign: 'center',
})

export const 範囲の選択 = style({
    padding: '4px 8px',
    fontSize: '12px',
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    color: エディターCSS変数('テキスト主'),
})
