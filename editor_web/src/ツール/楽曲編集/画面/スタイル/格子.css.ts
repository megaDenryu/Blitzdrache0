import { style } from '@vanilla-extract/css'
import { エディターCSS変数 } from '../../../../境界/index.ts'

export const 格子枠 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '2px',
    overflowX: 'auto',
})

export const 行枠 = style({
    display: 'flex',
    alignItems: 'center',
    height: '20px',
})

export const 行見出し = style({
    width: '100px',
    minWidth: '100px',
    flexShrink: 0,
    paddingRight: '8px',
    fontSize: '11px',
    fontFamily: 'monospace',
    color: エディターCSS変数('テキスト副'),
    textAlign: 'right',
    boxSizing: 'border-box',
    overflow: 'hidden',
    textOverflow: 'ellipsis',
    whiteSpace: 'nowrap',
})

export const 升目列 = style({
    display: 'flex',
    flex: 1,
    height: '100%',
    minWidth: '640px',
})

export const 升目 = style({
    flex: 1,
    height: '100%',
    boxSizing: 'border-box',
    borderTop: `1px solid ${エディターCSS変数('境界線薄')}`,
    borderBottom: `1px solid ${エディターCSS変数('境界線薄')}`,
    borderRight: `1px solid ${エディターCSS変数('境界線薄')}`,
    borderLeft: `1px solid ${エディターCSS変数('境界線薄')}`,
    backgroundColor: エディターCSS変数('パネル背景'),
    borderRadius: '1px',
    selectors: {
        '&[data-boundary="measure"]': {
            borderLeft: `2px solid ${エディターCSS変数('境界線')}`,
        },
        '&[data-boundary="beat"]': {
            borderLeft: `1px solid ${エディターCSS変数('テキスト薄')}`,
        },
        '&[data-allowed="false"]': {
            backgroundColor: エディターCSS変数('非活性背景'),
            opacity: 0.4,
        },
        '&[data-kind="start"][data-follow="true"]': {
            backgroundColor: エディターCSS変数('アクセント背景'),
            boxShadow: 'inset 0 0 0 1px rgba(255, 255, 255, 0.3)',
            borderRadius: '2px 0 0 2px',
            opacity: 1,
        },
        '&[data-kind="hold"][data-follow="true"]': {
            backgroundColor: エディターCSS変数('アクセントホバー'),
            opacity: 0.85,
        },
        '&[data-kind="start"][data-follow="false"]': {
            backgroundColor: エディターCSS変数('危険ボタン背景'),
            boxShadow: 'inset 0 0 0 1px rgba(255, 255, 255, 0.3)',
            borderRadius: '2px 0 0 2px',
            opacity: 1,
        },
        '&[data-kind="hold"][data-follow="false"]': {
            backgroundColor: エディターCSS変数('危険ボタンホバー'),
            opacity: 0.85,
        },
        // 打点の枠線は data-kind の側が使うため、再生位置の印は外側の輪郭で描いて重ならないようにする。
        '&[data-playhead="true"]': {
            outline: `2px solid ${エディターCSS変数('アクセント文字')}`,
            outlineOffset: '-1px',
            zIndex: 1,
        },
    },
})
