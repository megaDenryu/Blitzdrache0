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

export const 見出し行 = style({
    display: 'flex',
    justifyContent: 'space-between',
    fontSize: '11px',
    fontWeight: 600,
    color: '#cbd5e1',
})

export const 補助テキスト = style({
    color: '#34d399',
    fontSize: '10px',
})

export const ボタングループ = style({
    display: 'grid',
    gridTemplateColumns: 'repeat(3, 1fr)',
    gap: '4px',
})

export const ブラシ種別ボタン = style({
    padding: '6px 4px',
    fontSize: '11px',
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

globalStyle(`${ブラシ種別ボタン}[data-selected="true"]`, {
    backgroundColor: 'rgba(6, 78, 59, 0.6)',
    borderColor: '#059669',
    color: '#6ee7b7',
})
