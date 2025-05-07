import { Parameter } from '@transia/xrpl'
import { iParameterValue } from './iParameterValue'
import { iParameterType } from './iParameterType'

export class iParameter {
  type: iParameterType
  value: iParameterValue

  constructor(type: iParameterType, value: iParameterValue) {
    this.type = type
    this.value = value
  }

  fromHex(type: string, value: string) {
    this.type = new iParameterType(type)
    this.value = new iParameterValue(value)
  }

  toXrpl(): Parameter {
    return {
      Parameter: {
        ParameterValue: {
          type: this.type.value,
          value: this.value.value,
        },
      },
    } as Parameter
  }
}
