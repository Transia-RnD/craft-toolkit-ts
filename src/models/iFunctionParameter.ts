import { Parameter } from '@transia/xrpl'
import { iParameterFlag } from './iParameterFlag'
import { iParameterType } from './iParameterType'

export class iFunctionParameter {
  flag: iParameterFlag
  type: iParameterType

  constructor(flag: iParameterFlag, type: iParameterType) {
    this.flag = flag
    this.type = type
  }

  fromHex(flag: number, name: string, value: string) {
    this.flag = new iParameterFlag(flag)
    this.type = new iParameterType(value)
  }

  toXrpl(): Parameter {
    return {
      Parameter: {
        ParameterFlag: this.flag.value,
        ParameterType: {
          type: this.type.value,
        },
      },
    } as Parameter
  }
}
