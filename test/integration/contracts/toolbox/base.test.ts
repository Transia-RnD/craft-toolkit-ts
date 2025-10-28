// xrpl
import {
  ContractCall,
  ContractCreate,
  ContractFlags,
  convertStringToHex,
  xrpToDrops,
} from '@transia/xrpl'
import {
  // Testing
  XrplIntegrationTestContext,
  setupClient,
  teardownClient,
  serverUrl,
  // Main
  createContract,
  CreateContractParams,
  Xrpld,
  rpc,
  readWasmFromContract,
} from '../../../../dist/npm/src'
import {
  iInstanceParameter,
  iParameterFlag,
  iParameterName,
  iParameterType,
  iInstanceParameterValue,
  iParameterValue,
  iFunction,
  iFunctionName,
  iFunctionParameter,
  iParameter,
} from '../../../../dist/npm/src/models'

// Notes: Creating an immutable contract with a function that takes UINT8. Then "recreate" that contract with new function parameters. Its not creating a new contract account, just updating the existing one.

describe('base', () => {
  let testContext: XrplIntegrationTestContext
  let contractAccount: string

  beforeAll(async () => {
    testContext = await setupClient(serverUrl)
    console.log(testContext.client.connection.getUrl())

    const instanceParam1 = new iInstanceParameter(
      new iParameterFlag(65536),
      new iParameterName('616D6F756E74', true),
      new iParameterType('AMOUNT')
    )

    const instanceParamValue1 = new iInstanceParameterValue(
      instanceParam1.flag,
      instanceParam1.type,
      new iParameterValue(xrpToDrops('2000'))
    )

    const functionParam1 = new iFunctionParameter(
      new iParameterFlag(0),
      new iParameterName('account'),
      new iParameterType('ACCOUNT')
    )

    const functionParam2 = new iFunctionParameter(
      new iParameterFlag(0),
      new iParameterName('amount'),
      new iParameterType('AMOUNT')
    )

    const function1 = new iFunction(new iFunctionName('base'), [
      functionParam1,
      functionParam2,
    ])

    const builtTxn = {
      TransactionType: 'ContractCreate',
      Account: testContext.alice.classicAddress,
      ContractCode: readWasmFromContract('base'),
      Flags: ContractFlags.tfImmutable,
      InstanceParameters: [instanceParam1.toXrpl()],
      InstanceParameterValues: [instanceParamValue1.toXrpl()],
      Functions: [function1.toXrpl()],
      Fee: '2000000',
    } as ContractCreate
    console.log(JSON.stringify(builtTxn, null, 2))

    const { account } = (await createContract({
      client: testContext.client,
      wallet: testContext.alice,
      tx: builtTxn,
    } as CreateContractParams)) as any
    contractAccount = account as string
  })
  afterAll(async () => {
    await teardownClient(testContext)
  })

  it('basic contract', async () => {
    console.log(`Contract Account: ${contractAccount}`)
    const aliceWallet = testContext.alice

    const parameter1 = new iParameter(
      new iParameterFlag(0),
      new iParameterType('ACCOUNT'),
      new iParameterValue(aliceWallet.classicAddress)
    )

    const parameter2 = new iParameter(
      new iParameterFlag(0),
      new iParameterType('AMOUNT'),
      new iParameterValue(xrpToDrops('1'))
    )

    const builtTx: ContractCall = {
      TransactionType: 'ContractCall',
      Account: aliceWallet.classicAddress,
      ContractAccount: contractAccount,
      ComputationAllowance: 1000000,
      FunctionName: convertStringToHex('base'),
      Parameters: [parameter1.toXrpl(), parameter2.toXrpl()],
      Fee: '200000',
    }
    console.log(JSON.stringify(builtTx, null, 2))

    const result = await Xrpld.submit(testContext.client, {
      wallet: aliceWallet,
      tx: builtTx,
    })
    console.log(result)

    const contractInfo = await rpc(testContext.client, {
      command: 'contract_info',
      contract_account: contractAccount,
      account: aliceWallet.classicAddress,
    })
    console.log(contractInfo)
  })
})
