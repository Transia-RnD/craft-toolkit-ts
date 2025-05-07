import fs from 'fs'
import path from 'path'

export function readWasmFromContract(contractName: string): string {
  const buildPath = process.cwd() + '/' + 'build'
  const projectPath = `/${contractName}/wasm32v1-none/release`
  // const projectPath = `/${project}/wasm32-unknown-unknown/release`
  return wasmToHex(
    path.resolve(__dirname, `${buildPath}/${projectPath}/${contractName}.wasm`)
  )
}

export function readWasmFromPath(path: string): string {
  return wasmToHex(path)
}

export function wasmToHex(path: string): string {
  const wasm = fs.readFileSync(path)
  return wasm.toString(`hex`).toUpperCase()
}
