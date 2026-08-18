import { style, globalStyle } from '@vanilla-extract/css'

export const パネル = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '10px',
    padding: '12px',
    backgroundColor: 'rgba(2, 6, 23, 0.6)',
    borderRadius: '8px',
    border: '1px solid rgba(30, 41, 59, 0.8)',
})

export const 見出し = style({
    fontSize: '11px',
    fontWeight: 600,
    color: '#cbd5e1',
})

export const 材質グリッド = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(4, 1fr)',
    gap: '4px',
})

export const 材質ボタン = style({
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    padding: '6px 2px',
    fontSize: '10px',
    fontWeight: 500,
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#cbd5e1',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        backgroundColor: '#334155',
    },
})

globalStyle(`${材質ボタン}[data-selected="true"]`, {
    backgroundColor: 'rgba(22, 78, 99, 0.6)',
    borderColor: '#06b6d4',
    color: '#67e8f9',
})

export const ベイク区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: '1px solid #1e293b',
})

export const アクションボタン = style({
    padding: '6px 8px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#e2e8f0',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        backgroundColor: '#334155',
    },
})
