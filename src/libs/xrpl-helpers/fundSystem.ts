import { AccountSetAsfFlags, Client, Wallet } from '@transia/xrpl'
import {
  Account,
  ICXAH,
  IC,
  fund,
  trust,
  pay,
  balance,
  limit,
  accountSet,
  sell,
} from '../xrpl-helpers'
import { appLogger } from '../logger'

/**
 * This function will fund a new wallet on the Hooks Local Ledger.
 *
 * @returns {Wallet}
 */
export async function fundSystem(
  client: Client,
  wallet: Wallet,
  ic: IC
): Promise<void> {
  const userAccounts = [
    'gw',
    'alice',
    'bob',
    'carol',
    'dave',
    'elsa',
    'frank',
    'grace',
    'heidi',
    'ivan',
    'judy',
  ]
  const userWallets = userAccounts.map((account) => new Account(account))
  const USD = ic as IC

  // FUND GW
  const gw = userWallets[0]
  const gwNativeBalance = await balance(client, gw.wallet.classicAddress)
  if (gwNativeBalance == 0) {
    appLogger.debug(`SETUP GW: ${gwNativeBalance}`)
    await fund(client, wallet, new ICXAH(10000000), gw.wallet.classicAddress)
    await accountSet(client, gw.wallet, AccountSetAsfFlags.asfDefaultRipple)
    await sell(client, USD.set(20000), gw.wallet, 0.8)
  }

  const needsFunding = []
  const needsLines = []
  const needsIC = []

  for (let i = 1; i < userWallets.length; i++) {
    const wallet = userWallets[i]
    const address = wallet.wallet.classicAddress

    const nativeBalance = await balance(client, address)
    if (nativeBalance < 10000000000) {
      appLogger.debug(`${address} NEEDS FUNDING: ${nativeBalance}`)
      needsFunding.push(address)
    }

    const usdLimit = await limit(client, address, USD)
    if (usdLimit < 100000) {
      appLogger.debug(`${address} NEEDS TRUST: ${usdLimit}`)
      needsLines.push(wallet.wallet)
    }

    const usdBalance = await balance(client, address, USD)
    if (usdBalance < 10000) {
      appLogger.debug(`${address} NEEDS IC: ${usdBalance}`)
      needsIC.push(address)
    }
  }

  appLogger.debug(`FUNDING: ${needsFunding.length}`)
  appLogger.debug(`TRUSTING: ${needsLines.length}`)
  appLogger.debug(`PAYING: ${needsIC.length}`)

  await fund(client, wallet, new ICXAH(20000), ...needsFunding)
  await trust(client, USD.set(100000), ...needsLines)
  await pay(client, USD.set(50000), gw.wallet, ...needsIC)
}
