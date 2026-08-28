import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../../境界/index.ts'

// 右サイドバーのスロットに収まる大域インスペクターパネルの枠。設定は数が増えうるため、
// この枠の中だけが縦にスクロールする(設計正本の判断14)。
export const インスペクター枠 = style({
    width: '100%',
    height: '100%',
    overflowY: 'auto',
    backgroundColor: エディターCSS変数('パネル背景'),
    padding: '16px',
    boxSizing: 'border-box',
    display: 'flex',
    flexDirection: 'column',
    gap: '14px',
    color: エディターCSS変数('テキスト主'),
})
