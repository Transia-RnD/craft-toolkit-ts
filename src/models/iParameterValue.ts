export class iParameterValue {
  value: any

  constructor(value: any) {
    this.value = value
  }

  static from(value: string): iParameterValue {
    return new iParameterValue(value)
  }
}
