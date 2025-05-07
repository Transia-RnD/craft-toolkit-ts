import { TYPE_ID_TO_STRING } from '@transia/ripple-binary-codec/dist/types/serialized-type'

export class iParameterType {
  value: string

  constructor(value: string) {
    this.value = value
  }

  static from(value: string): iParameterType {
    if (!Object.values(TYPE_ID_TO_STRING).includes(value)) {
      throw new Error(`Invalid iParameterType: ${value}`)
    }

    return new iParameterType(value)
  }
}
