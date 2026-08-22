import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

// 平面図の1升目は、中央の升目そのものと、それを囲む4つの面のボタンで組む。
// 3×3の格子で並べるのは、面のボタンの位置がそのまま建物のどの面かを表すためである
// (上が正面、下が背面、左が左面、右が右面)。
export const 平面図 = style({
    display: 'inline-grid',
    gap: '4px',
    padding: '12px',
    backgroundColor: エディターCSS変数('パネル背景'),
    border: `1px solid ${エディターCSS変数('境界線')}`,
    borderRadius: '4px',
})

export const 升目枠 = style({
    display: 'grid',
    gridTemplateColumns: '14px 44px 14px',
    gridTemplateRows: '14px 44px 14px',
    gap: '2px',
})

export const 升目中央 = style({
    gridColumn: '2',
    gridRow: '2',
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    fontSize: '10px',
    lineHeight: 1.2,
    cursor: 'pointer',
    borderRadius: '3px',
    color: エディターCSS変数('テキスト主'),
    backgroundColor: エディターCSS変数('カード背景'),
    border: `1px solid ${エディターCSS変数('カード枠線')}`,
    selectors: {
        '&[data-升目あり="false"]': {
            backgroundColor: 'transparent',
            borderStyle: 'dashed',
            color: エディターCSS変数('テキスト薄'),
        },
        '&[data-根="true"]': {
            borderColor: エディターCSS変数('選択枠線'),
            borderWidth: '2px',
        },
    },
})

export const 面ボタン = style({
    cursor: 'pointer',
    borderRadius: '2px',
    border: 'none',
    padding: '0',
    backgroundColor: エディターCSS変数('境界線薄'),
    selectors: {
        '&[data-壁="平壁"]': { backgroundColor: '#9ca3af' },
        '&[data-壁="窓壁"]': { backgroundColor: '#38bdf8' },
        '&[data-壁="扉枠付きの壁"]': { backgroundColor: '#f59e0b' },
        '&[data-壁="継ぎ口"]': { backgroundColor: 'transparent' },
    },
})

export const 正面ボタン = style({ gridColumn: '2', gridRow: '1' })
export const 背面ボタン = style({ gridColumn: '2', gridRow: '3' })
export const 左面ボタン = style({ gridColumn: '1', gridRow: '2' })
export const 右面ボタン = style({ gridColumn: '3', gridRow: '2' })
