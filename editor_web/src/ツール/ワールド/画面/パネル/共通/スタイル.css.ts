import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const 行コンテナ = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '4px',
})

export const ラベル行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    color: エディターCSS変数('テキスト薄'),
})

export const 値ラベル = style({
    fontFamily: 'monospace',
    color: エディターCSS変数('テキストコード'),
})

export const スライダー入力 = style({
    width: '100%',
    accentColor: エディターCSS変数('アクセントホバー'),
    cursor: 'pointer',
})
