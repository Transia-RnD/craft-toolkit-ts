import { Function as XrplFunction } from '@transia/xrpl'
import { iFunctionName } from './iFunctionName'
import { iFunctionParameter } from './iFunctionParameter'

export class iFunction {
  name: iFunctionName
  parameters?: iFunctionParameter[] | null

  constructor(name: iFunctionName, parameters?: iFunctionParameter[] | null) {
    this.name = name
    this.parameters = parameters
  }

  fromHex(name: string, parameters: iFunctionParameter[]) {
    this.name = new iFunctionName(name)
    this.parameters = parameters
  }

  toXrpl(): XrplFunction {
    const f = {
      Function: {
        FunctionName: !this.name.isHex ? this.name.toHex() : this.name.value,
      },
    }
    if (this.parameters && this.parameters.length > 0) {
      // @ts-expect-error -- ignore
      f.Function.Parameters = this.parameters.map((p) => p.toXrpl())
    }
    return f as XrplFunction
  }
}
