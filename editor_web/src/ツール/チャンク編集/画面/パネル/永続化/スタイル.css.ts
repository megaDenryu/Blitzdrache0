import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

export const 永続化枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    backgroundColor: エディターCSS変数('カード不透明背景'),
    borderRadius: '4px',
    border: `1px solid ${エディターCSS変数('境界線')}`,
    marginTop: '8px',
})
