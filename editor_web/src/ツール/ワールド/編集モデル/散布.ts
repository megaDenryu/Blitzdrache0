import type { 散布の設定, チャンクの道路, 建物の配置 } from '../../../生成/編集資源契約.ts'
import { 決定的乱数生成器 } from './乱数.ts'
import { ポアソンディスク候補を生成する, type 散布点 } from './ポアソンディスク候補生成.ts'
import { CatmullRomCurve3, Vector3 } from 'three'

export type { 散布点 }

// ポアソンディスク標本化による植生等の散布計算を決定的に実行する。
export class 散布 {
    public static readonly 最大散布数 = 1500

    // 設定と除外要素に基づきポアソンディスク散布点を決定的に生成する。
    public static 標本化を計算する(
        設定: 散布の設定,
        一辺のメートル: number,
        道路: チャンクの道路 | null,
        建物一覧: ReadonlyArray<建物の配置>,
    ): Array<散布点> {
        if (設定.最小間隔メートル <= 0) {
            throw new Error(`最小間隔メートルは正の数でなければならない: ${設定.最小間隔メートル}`)
        }
        const 乱数 = new 決定的乱数生成器(設定.乱数の種)
        const 候補点列 = ポアソンディスク候補を生成する(設定.最小間隔メートル, 一辺のメートル, this.最大散布数, 乱数)
        const 道路標本点列 = this.道路標本点列を取得する(道路)
        const 道路バッファ自乗 = 道路 !== null ? 道路.散布除外バッファメートル * 道路.散布除外バッファメートル : 0

        const 散布結果: Array<散布点> = []
        for (const 候補 of 候補点列) {
            if (散布結果.length >= this.最大散布数) break
            let 除外対象 = false
            if (道路 !== null && 道路標本点列.length > 0) {
                for (const sp of 道路標本点列) {
                    const dx = 候補.x - sp.x
                    const dz = 候補.z - sp.z
                    if (dx * dx + dz * dz < 道路バッファ自乗) {
                        除外対象 = true
                        break
                    }
                }
            }
            if (!除外対象) {
                for (const 建物 of 建物一覧) {
                    const dx = 候補.x - 建物.位置.x
                    const dz = 候補.z - 建物.位置.z
                    const 除外距離 = 建物.基礎半径メートル + 1.0
                    if (dx * dx + dz * dz < 除外距離 * 除外距離) {
                        除外対象 = true
                        break
                    }
                }
            }
            if (!除外対象) {
                散布結果.push(候補)
            }
        }
        return 散布結果
    }

    private static 道路標本点列を取得する(道路: チャンクの道路 | null): Array<Vector3> {
        if (道路 === null || 道路.制御点列.length < 2) return []
        const 頂点列 = 道路.制御点列.map((p) => new Vector3(p.x, p.y, p.z))
        const 曲線 = new CatmullRomCurve3(頂点列, false, 'centripetal')
        return 曲線.getSpacedPoints(道路.細分割数 * 2)
    }
}
