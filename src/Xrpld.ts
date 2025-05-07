/*
Contains the operations that can are performed by the application.
*/
import { Client, SubmittableTransaction, validate } from '@transia/xrpl'
import { appTransaction } from './libs/xrpl-helpers/transaction'
import { SmartContractParams } from './types'
import { appLogger } from './libs/logger'

export class Xrpld {
  static async submit(
    client: Client,
    params: SmartContractParams
  ): Promise<any> {
    if (!params.tx) {
      throw Error('Missing tx parameter')
    }
    const builtTx = params.tx as SubmittableTransaction
    appLogger.debug(JSON.stringify(builtTx))

    validate(builtTx)
    const txResponse = await appTransaction(client, builtTx, params.wallet, {
      hardFail: true,
      count: 1,
      delayMs: 1000,
    })
    return txResponse?.result
  }
}
