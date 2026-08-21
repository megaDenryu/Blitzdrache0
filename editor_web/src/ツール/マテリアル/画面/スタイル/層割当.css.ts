import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const 層割当行 = style({
    display: 'flex',
    alignItems: 'center',
    gap: '10px',
})

export const 層ラベル = style({
    width: '32px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト副'),
})

export const 層セレクト = style({
    flex: 1,
    padding: '5px 8px',
    fontSize: '12px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '3px',
})
