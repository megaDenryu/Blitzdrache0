import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const ヒント枠 = style({
    padding: '8px 12px',
    fontSize: '11px',
    lineHeight: '1.4',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '8px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    color: エディターCSS変数('テキスト薄'),
})

export const 強調 = style({
    color: エディターCSS変数('アクセント文字'),
    fontWeight: 'bold',
})

export const ラベル = style({
    color: エディターCSS変数('テキスト主'),
    fontWeight: 'bold',
})
