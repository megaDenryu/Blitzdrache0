import { style } from '@vanilla-extract/css'

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

export const アクション区画 = style({
    display: 'flex',
    flexDirection: 'column',
    gap: '6px',
    paddingTop: '8px',
    borderTop: '1px solid #1e293b',
})

export const 切土盛土ボタン = style({
    padding: '6px 8px',
    fontSize: '11px',
    fontWeight: 500,
    borderRadius: '4px',
    border: '1px solid rgba(217, 119, 6, 0.8)',
    backgroundColor: 'rgba(180, 83, 9, 0.8)',
    color: '#ffffff',
    cursor: 'pointer',
    transition: 'all 0.15s ease',
    ':hover': {
        backgroundColor: '#d97706',
    },
})

export const 行ボタン群 = style({
    display: 'flex',
    gap: '8px',
})

export const 削除ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: '1px solid #9f1239',
    backgroundColor: '#4c0519',
    color: '#fda4af',
    cursor: 'pointer',
    ':disabled': {
        opacity: 0.3,
        cursor: 'not-allowed',
    },
})

export const 副ボタン = style({
    flex: 1,
    padding: '4px 8px',
    fontSize: '11px',
    borderRadius: '4px',
    border: '1px solid #334155',
    backgroundColor: '#1e293b',
    color: '#cbd5e1',
    cursor: 'pointer',
    ':hover': {
        backgroundColor: '#334155',
    },
})
