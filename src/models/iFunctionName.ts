import { convertHexToString, convertStringToHex } from '@transia/xrpl'

export class iFunctionName {
  value: string
  isHex: boolean

  constructor(value: string, isHex?: boolean) {
    this.value = value
    this.isHex = isHex ? isHex : false
  }

  static fromHex(hexValue: string): iFunctionName {
    return new iFunctionName(convertHexToString(hexValue))
  }

  toHex(): string {
    return convertStringToHex(this.value)
  }
}
