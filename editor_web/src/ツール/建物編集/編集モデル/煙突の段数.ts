// 壁の外面へ立てる煙突が何段積むか。零段の煙突は「壁の外面へ立てるのに1段も無い」という半端な状態に
// なるため、生成の時点で拒む。判定の正本はサーバーの`煙突の段数`であり、この型はその条件を画面の側で先に守る。
// 参照: `crates/blitz_assembly/src/building_grid/ornament.rs`
export class 煙突の段数 {
    private constructor(public readonly 段数: number) {}

    public static 生成する(段数: number): 煙突の段数 | undefined {
        return Number.isInteger(段数) && 段数 >= 1 ? new 煙突の段数(段数) : undefined
    }

    // 画面が既定で選んでいる段数。1段は壁の外面へ差し込むだけで、上へ積み足す段を持たない。
    public static 既定を作る(): 煙突の段数 {
        return new 煙突の段数(1)
    }

    public 同じか(相手: 煙突の段数): boolean {
        return this.段数 === 相手.段数
    }
}

// 画面が選択肢として並べる段数。屋根を貫いて上へ出る姿を見るのに3段まであれば足りる。
export const 選べる煙突の段数: readonly 煙突の段数[] = [1, 2, 3].flatMap((段数) => {
    const 値 = 煙突の段数.生成する(段数)
    return 値 === undefined ? [] : [値]
})
