import { InstanceParameter } from '@transia/xrpl'
import { iParameterFlag } from './iParameterFlag'
import { iParameterType } from './iParameterType'

export class iInstanceParameter {
  flag: iParameterFlag
  type: iParameterType

  constructor(flag: iParameterFlag, type: iParameterType) {
    this.flag = flag
    this.type = type
  }

  fromHex(flag: number, value: string) {
    this.flag = new iParameterFlag(flag)
    this.type = new iParameterType(value)
  }

  toXrpl(): InstanceParameter {
    return {
      InstanceParameter: {
        ParameterFlag: this.flag.value,
        ParameterType: {
          type: this.type.value,
        },
      },
    } as InstanceParameter
  }
}
