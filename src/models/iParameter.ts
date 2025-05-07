import { Parameter } from '@transia/xrpl'
import { iParameterValue } from './iParameterValue'
import { iParameterType } from './iParameterType'
import { iParameterFlag } from './iParameterFlag'

export class iParameter {
  flag: iParameterFlag
  type: iParameterType
  value: iParameterValue

  constructor(
    flag: iParameterFlag,
    type: iParameterType,
    value: iParameterValue
  ) {
    this.flag = flag
    this.type = type
    this.value = value
  }

  fromHex(flag: number, type: string, value: string) {
    this.flag = new iParameterFlag(flag)
    this.type = new iParameterType(type)
    this.value = new iParameterValue(value)
  }

  toXrpl(): Parameter {
    return {
      Parameter: {
        ParameterFlag: this.flag.value,
        ParameterValue: {
          type: this.type.value,
          value: this.value.value,
        },
      },
    } as Parameter
  }
}
