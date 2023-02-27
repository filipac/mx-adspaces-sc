import {
    Address, AddressType,
    BigUIntType,
    BinaryCodec, BooleanType,
    ContractCallPayloadBuilder, ContractFunction,
    FieldDefinition, Query, SmartContract, StringValue, Struct,
    StructType,
    U64Type
} from "@multiversx/sdk-core/out";
import { ApiNetworkProvider } from "@multiversx/sdk-network-providers/out";
import BigNumber from "bignumber.js";
import { GraphQLClient, gql } from 'graphql-request'

const add = 'erd1ex7u5wkseyl2e3ytzh2sjvrrt4azjgxyghuctvf5d2hr2vkdeg8qh5zh50'
const contractAddress = 'erd1qqqqqqqqqqqqqpgqzq3sve9cavz8dq00smeul6ne8j0w6428eg8q9crjxs'

// @ts-ignore
process.env["NODE_TLS_REJECT_UNAUTHORIZED"] = 0;

(async () => {
    let networkProvider = new ApiNetworkProvider("https://devnet-api.multiversx.com");


    try {
        let tx = await networkProvider.getTransaction('706d6ec90baff5547293ec632087974a70526561f9d49ad5f0644f81c567865e')
        let buySpaceEvent = tx.logs.events.find(e => e.identifier === 'buySpace' && e.topics.length === 5)
        const data = {
            event: buySpaceEvent.topics[0].toString(),
            spaceName: buySpaceEvent.topics[1].toString(),
        }
        const address = new Address(buySpaceEvent.topics[2].valueOf())
        data['address'] = address.bech32()

        const paid = new BinaryCodec().decodeTopLevel(buySpaceEvent.topics[3].valueOf(), new BigUIntType());
        data['paid'] = parseFloat(new BigNumber(paid.valueOf()).dividedBy(10 ** 6).toString()).toFixed(2)

        const paid_until = new BinaryCodec().decodeTopLevel(buySpaceEvent.topics[4].valueOf(), new BigUIntType());
        data['paid_until'] = new Date(paid_until.valueOf() * 1000).toISOString()

        console.log(data)

        const testEvent = tx.logs.events.find(e => e.identifier === 'buySpace' && e.topics.length === 2)
        const data2 = {
            event: testEvent.topics[0].toString(),
            name: testEvent.topics[1].toString(),
        }

        console.log(data2)
    } catch (e) {
        console.log(e, e.inner)
    }


    // networkProvider.queryContract(new ContractQ)
})()
