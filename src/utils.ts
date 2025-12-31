import fs from 'fs'
// import path from 'path'

export function readWasmFromContract(contractName: string): string {
  const projectPath = `contracts/${contractName}/target/wasm32v1-none/release/${contractName}.wasm`
  return wasmToHex(projectPath)
}

export function readWasmFromPath(path: string): string {
  return wasmToHex(path)
}

export function wasmToHex(path: string): string {
  const wasm = fs.readFileSync(path)
  return wasm.toString(`hex`).toUpperCase()
}
