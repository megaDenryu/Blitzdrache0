declare module 'node:fs' {
    export function readFileSync(path: string, encoding: string): string
    export function writeFileSync(path: string, data: string, encoding: string): void
}

declare const process: {
    readonly argv: readonly string[]
    exit(code?: number): never
}
