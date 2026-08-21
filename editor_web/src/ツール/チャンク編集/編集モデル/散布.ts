import type { チャンク座標, 散布の個体, 散布の設定, チャンクの道路, 建物の配置 } from '../../../生成/編集資源契約.ts'
import { 決定的乱数生成器 } from './乱数.ts'
import { ポアソンディスク候補を生成する, type 散布点 } from './ポアソンディスク候補生成.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

export type { 散布点 }

// 散布を避ける帯とは、1本の道路の中心線の標本点列と、その道路の散布除外バッファの2乗を
// 組にしたもののことである。除外バッファは道路ごとに違うため、道路ごとに組で持つ。
interface 散布を避ける帯 {
    readonly 標本点列: ReadonlyArray<Vector3>
    readonly 除外距離の2乗: number
}

// ポアソンディスク標本化による植生等の散布計算を決定的に実行する。
export class 散布 {
    public static readonly 最大散布数 = 1500

    // 散布点へ安定識別子を与え、保存とベイクが運ぶ散布の個体一覧へ確定させる。
    // 識別子にチャンク座標を混ぜるのは、焼く側が識別子から向きと大きさを導くためである。
    // 添字だけで作ると、どのチャンクでも同じ添字の木がまったく同じ姿になる。
    public static 個体一覧を確定する(
        座標: チャンク座標,
        設定: 散布の設定,
        一辺のメートル: number,
        道路一覧: ReadonlyArray<チャンクの道路>,
        建物一覧: ReadonlyArray<建物の配置>,
    ): Array<散布の個体> {
        return this.標本化を計算する(設定, 一辺のメートル, 道路一覧, 建物一覧).map((点, 添字) => ({
            安定識別子: `散布-x${座標.x}z${座標.z}-${添字}`,
            チャンク中心からの東メートル: 点.x,
            チャンク中心からの南メートル: 点.z,
        }))
    }

    // 設定と除外要素に基づきポアソンディスク散布点を決定的に生成する。
    public static 標本化を計算する(
        設定: 散布の設定,
        一辺のメートル: number,
        道路一覧: ReadonlyArray<チャンクの道路>,
        建物一覧: ReadonlyArray<建物の配置>,
    ): Array<散布点> {
        if (設定.最小間隔メートル <= 0) {
            throw new Error(`最小間隔メートルは正の数でなければならない: ${設定.最小間隔メートル}`)
        }
        const 乱数 = new 決定的乱数生成器(設定.乱数の種)
        const 候補点列 = ポアソンディスク候補を生成する(設定.最小間隔メートル, 一辺のメートル, this.最大散布数, 乱数)
        const 避ける帯一覧 = 道路一覧.map((道路) => this.避ける帯を作る(道路)).filter((帯) => 帯.標本点列.length > 0)

        const 散布結果: Array<散布点> = []
        for (const 候補 of 候補点列) {
            if (散布結果.length >= this.最大散布数) break
            if (this.道路を避けるか(候補, 避ける帯一覧)) continue
            if (this.建物を避けるか(候補, 建物一覧)) continue
            散布結果.push(候補)
        }
        return 散布結果
    }

    private static 道路を避けるか(候補: 散布点, 避ける帯一覧: ReadonlyArray<散布を避ける帯>): boolean {
        for (const 帯 of 避ける帯一覧) {
            for (const sp of 帯.標本点列) {
                const dx = 候補.x - sp.x
                const dz = 候補.z - sp.z
                if (dx * dx + dz * dz < 帯.除外距離の2乗) return true
            }
        }
        return false
    }

    private static 建物を避けるか(候補: 散布点, 建物一覧: ReadonlyArray<建物の配置>): boolean {
        for (const 建物 of 建物一覧) {
            const dx = 候補.x - 建物.位置.x
            const dz = 候補.z - 建物.位置.z
            const 除外距離 = 建物.基礎半径メートル + 1.0
            if (dx * dx + dz * dz < 除外距離 * 除外距離) return true
        }
        return false
    }

    private static 避ける帯を作る(道路: チャンクの道路): 散布を避ける帯 {
        const 除外距離の2乗 = 道路.散布除外バッファメートル * 道路.散布除外バッファメートル
        if (道路.制御点列.length < 2) return { 標本点列: [], 除外距離の2乗 }
        const 頂点列 = 道路.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
        const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
        return { 標本点列: 曲線.getSpacedPoints(道路.細分割数 * 2), 除外距離の2乗 }
    }
}
