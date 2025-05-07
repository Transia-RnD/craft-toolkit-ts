import { InstanceParameterValue } from '@transia/xrpl'
import { iParameterFlag } from './iParameterFlag'
import { iParameterValue } from './iParameterValue'
import { iInstanceParameter } from './iInstanceParameter'
import { iParameterType } from './iParameterType'

export class iInstanceParameterValue {
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

  fromInstanceParameter(instanceParameter: iInstanceParameter, value: any) {
    this.flag = new iParameterFlag(instanceParameter.flag.value)
    this.type = new iParameterType(instanceParameter.type.value)
    this.value = new iParameterValue(value)
  }

  fromHex(flag: number, name: string, value: any) {
    this.flag = new iParameterFlag(flag)
    this.value = new iParameterValue(value)
  }

  toXrpl(): InstanceParameterValue {
    return {
      InstanceParameterValue: {
        ParameterFlag: this.flag.value,
        ParameterValue: {
          type: this.type.value,
          value: this.value.value,
        },
      },
    } as InstanceParameterValue
  }
}
