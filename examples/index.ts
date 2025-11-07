import {
  Client,
  ContractCall,
  ContractCreate,
  ContractFlags,
  convertStringToHex,
  Wallet,
  xrpToDrops,
} from '@transia/xrpl'
import {
  createContract,
  CreateContractParams,
  readWasmFromPath,
  rpc,
  Xrpld,
} from '@transia/craft-toolkit-ts'
import {
  iInstanceParameter,
  iParameterFlag,
  iParameterType,
  iInstanceParameterValue,
  iParameterValue,
  iFunction,
  iFunctionName,
  iFunctionParameter,
  iParameter,
} from '@transia/craft-toolkit-ts'
import 'dotenv/config'

class ContractManager {
  private client: Client
  private aliceWallet: Wallet
  private bobWallet: Wallet
  private contractId: string | null = null
  private contractAccount: string | null = null

  constructor(alice: Wallet, bob: Wallet) {
    this.client = new Client('wss://alphanet.nerdnest.xyz')
    this.aliceWallet = alice
    this.bobWallet = bob
    this.contractId = ''
    this.contractAccount = ''
  }

  async connect() {
    await this.client.connect()
  }

  async disconnect() {
    await this.client.disconnect()
  }

  async fund() {
    const aliceResponse = await this.client.fundWallet(
      new Wallet('', '', {
        masterAddress: this.aliceWallet.classicAddress,
      }),
      {
        faucetHost: 'alphanet.faucet.nerdnest.xyz',
        faucetPath: '/accounts',
      }
    )
    console.log('Fund response:', aliceResponse)
    const bobResponse = await this.client.fundWallet(
      new Wallet('', '', {
        masterAddress: this.bobWallet.classicAddress,
      }),
      {
        faucetHost: 'alphanet.faucet.nerdnest.xyz',
        faucetPath: '/accounts',
      }
    )
    console.log('Fund response:', bobResponse)
  }

  async getContraactSle(): Promise<any> {
    try {
      const contractSle = await this.client.request({
        command: 'ledger_entry',
        index: this.contractId as string,
      })
      return contractSle
    } catch (error) {
      console.error('Error fetching contract SLE:', error)
      return null
    }
  }

  async createContract() {
    const contractSle = await this.getContraactSle()

    // The Contract Ledger Entry
    console.log(JSON.stringify(contractSle, null, 2))
    if (contractSle) {
      return
    }

    const instanceParam1 = new iInstanceParameter(
      new iParameterFlag(65536),
      new iParameterType('AMOUNT')
    )

    const instanceParamValue1 = new iInstanceParameterValue(
      instanceParam1.flag,
      instanceParam1.type,
      new iParameterValue(xrpToDrops('20'))
    )

    const functionParam1 = new iFunctionParameter(
      new iParameterFlag(0),
      new iParameterType('UINT8')
    )

    const function1 = new iFunction(new iFunctionName('base'), [functionParam1])

    const builtTxn: ContractCreate = {
      TransactionType: 'ContractCreate',
      Account: this.aliceWallet.classicAddress,
      ContractCode: readWasmFromPath(
        '/Users/darkmatter/projects/ledger-works/craft-toolkit-ts/build/base/wasm32v1-none/release/base.wasm'
      ),
      Flags: ContractFlags.tfImmutable,
      InstanceParameters: [instanceParam1.toXrpl()],
      InstanceParameterValues: [instanceParamValue1.toXrpl()],
      Functions: [function1.toXrpl()],
      Fee: '2000000',
    }

    const { id, account } = (await createContract({
      client: this.client,
      wallet: this.aliceWallet,
      tx: builtTxn,
    } as CreateContractParams)) as any

    this.contractId = id
    this.contractAccount = account

    console.log('Contract Account:', account)
    console.log('Contract ID:', this.contractId)
  }

  async callContract() {
    if (!this.contractAccount) {
      throw new Error('Contract not created yet.')
    }

    const parameter1 = new iParameter(
      new iParameterFlag(0),
      new iParameterType('UINT8'),
      new iParameterValue(1)
    )

    const builtTx: ContractCall = {
      TransactionType: 'ContractCall',
      Account: this.bobWallet.classicAddress,
      ContractAccount: this.contractAccount,
      ComputationAllowance: 1000000,
      FunctionName: convertStringToHex('base'),
      Parameters: [parameter1.toXrpl()],
      Fee: '5000',
    }
    console.log(JSON.stringify(builtTx, null, 2))

    // Replace Xrpld.submit and rpc with your actual implementation
    const result = await Xrpld.submit(this.client, {
      wallet: this.bobWallet,
      tx: builtTx,
    })
    console.log(result)

    const contractInfo = await rpc(this.client, {
      command: 'contract_info',
      contract_account: this.contractAccount,
      account: this.bobWallet.classicAddress,
    })
    console.log(contractInfo)
  }
}

async function main() {
  const aliceWallet = Wallet.fromSeed('sEdSScfGpPRUnEBcUjR2aFdcnkhDhx8')
  const bobWallet = Wallet.fromSeed('sEdVZZwXj3Y4f2X7PVK2x1dNQBz9N53')
  const manager = new ContractManager(aliceWallet, bobWallet)
  await manager.connect()
  await manager.fund()
  await manager.createContract()
  await manager.callContract()
  await manager.disconnect()
}

main()
