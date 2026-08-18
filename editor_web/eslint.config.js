// 層と依存の向きを機械強制するための設定。
// 参照: `_doc/設計/ゲーム開発用エディター基盤.md`「層の定義」。
//
// 「知ってよい相手」の表をそのまま eslint-plugin-boundaries の規則へ写す。
// 各層は自分自身の型も allow に含める（同じ層のファイルどうしが互いを
// 参照するのは正当なため）。
//
// import/no-cycle は値レベルの循環importを落とす保険として併用する。
// 注意: eslint-plugin-import の no-cycle は `import type` だけで構成される
// 循環を検出しない（型のみの相互参照で層が循環する場合は boundaries/element-types が
// 層をまたぐ辺そのものを禁じるため、そちらが実効的な検出を担う）。
import js from '@eslint/js'
import boundaries from 'eslint-plugin-boundaries'
import importPlugin from 'eslint-plugin-import'
import tseslint from 'typescript-eslint'

const 生成 = '生成'
const 境界 = '境界'
const 編集モデル = '編集モデル'
const 操作コマンド = '操作コマンド'
const 画面 = '画面'
const ツールルート = 'ツールルート'
const 入り口 = '入り口'
const ヘッドレス = 'ヘッドレス'
const コンポジションルート = 'main'
// SengenUI・VscodeShellLayout・SengenThreeはいずれもGitサブモジュールであり、
// 本リポジトリの層の対象外（触ってはいけない範囲）。各層の表の「知ってよい相手」に
// 現れる箇所だけからの参照を許す外部部品として扱う。
const SengenUI = 'SengenUI'
const VscodeShellLayout = 'VscodeShellLayout'
const SengenThree = 'SengenThree'

export default tseslint.config(
    {
        ignores: ['dist/**', 'src/生成/**', 'submodules/**', 'node_modules/**', 'eslint.config.js'],
    },
    js.configs.recommended,
    ...tseslint.configs.recommended,
    {
        files: ['src/**/*.ts'],
        plugins: {
            boundaries,
            import: importPlugin,
        },
        settings: {
            'import/resolver': {
                typescript: {
                    project: './tsconfig.json',
                },
            },
            'boundaries/include': ['src/**/*.ts'],
            'boundaries/elements': [
                { type: 生成, mode: 'full', pattern: ['src/生成/**'] },
                { type: 境界, mode: 'full', pattern: ['src/境界/**'] },
                { type: 編集モデル, mode: 'full', pattern: ['src/ツール/*/編集モデル/**'] },
                { type: 操作コマンド, mode: 'full', pattern: ['src/ツール/*/操作コマンド/**'] },
                { type: 画面, mode: 'full', pattern: ['src/ツール/*/画面/**'] },
                // ツール直下（編集モデル・操作コマンド・画面のいずれにも属さないファイル）は
                // そのツールのコンポジションルートとして、境界・操作コマンド・編集モデル・画面の
                // 全部を知ってよい。参照: `_doc/設計/ゲーム開発用エディター基盤.md`「層の定義」の
                // 「ツールルート」の行。
                { type: ツールルート, mode: 'full', pattern: ['src/ツール/*/*.ts'] },
                { type: 入り口, mode: 'full', pattern: ['src/入り口/**'] },
                { type: ヘッドレス, mode: 'full', pattern: ['src/ヘッドレス/**'] },
                { type: コンポジションルート, mode: 'full', pattern: ['src/main.ts'] },
                { type: SengenUI, mode: 'full', pattern: ['submodules/SengenUI/**'] },
                { type: VscodeShellLayout, mode: 'full', pattern: ['submodules/VscodeShellLayout/**'] },
                { type: SengenThree, mode: 'full', pattern: ['submodules/SengenThree/**'] },
            ],
        },
        rules: {
            'boundaries/no-unknown': 'error',
            'boundaries/no-unknown-files': 'error',
            'boundaries/element-types': [
                'error',
                {
                    default: 'disallow',
                    rules: [
                        { from: [生成], allow: [生成] },
                        { from: [境界], allow: [境界, 生成] },
                        { from: [編集モデル], allow: [編集モデル, 生成] },
                        { from: [操作コマンド], allow: [操作コマンド, 編集モデル, 境界, 生成] },
                        { from: [画面], allow: [画面, 操作コマンド, 編集モデル, 境界, 生成, SengenUI, SengenThree] },
                        {
                            from: [ツールルート],
                            allow: [ツールルート, 境界, 操作コマンド, 編集モデル, 画面, 生成, SengenUI, SengenThree],
                        },
                        { from: [入り口], allow: [入り口, 境界, ツールルート, SengenUI, VscodeShellLayout] },
                        { from: [ヘッドレス], allow: [ヘッドレス, 境界, 操作コマンド, 編集モデル, 生成] },
                        {
                            from: [コンポジションルート],
                            allow: [
                                コンポジションルート,
                                入り口,
                                境界,
                                ツールルート,
                                画面,
                                操作コマンド,
                                編集モデル,
                                生成,
                                SengenUI,
                                VscodeShellLayout,
                                SengenThree,
                            ],
                        },
                    ],
                },
            ],
            'import/no-cycle': ['error', { maxDepth: Infinity }],
        },
    },
)
