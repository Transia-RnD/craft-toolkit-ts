import { Function as XrplFunction } from '@transia/xrpl'
import { iFunctionName } from './iFunctionName'
import { iFunctionParameter } from './iFunctionParameter'

export class iFunction {
  name: iFunctionName
  parameters: iFunctionParameter[]

  constructor(name: iFunctionName, parameters: iFunctionParameter[]) {
    this.name = name
    this.parameters = parameters
  }

  fromHex(name: string, parameters: iFunctionParameter[]) {
    this.name = new iFunctionName(name)
    this.parameters = parameters
  }

  toXrpl(): XrplFunction {
    return {
      Function: {
        FunctionName: !this.name.isHex ? this.name.toHex() : this.name.value,
        Parameters: this.parameters.map((p) => p.toXrpl()),
      },
    } as XrplFunction
  }
}
