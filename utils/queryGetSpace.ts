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
process.env["NODE_TLS_REJECT_UNAUTHORIZED"] = 0

function getStructure() {
    return new StructType('AdvertiseSpace', [
        new FieldDefinition('owner', '', new AddressType()),
        new FieldDefinition('paid_amount', '', new BigUIntType()),
        new FieldDefinition('paid_until', '', new BigUIntType()),
        new FieldDefinition('is_new', '', new BooleanType()),
    ])
}

(async () => {
    let networkProvider = new ApiNetworkProvider("https://devnet-api.multiversx.com");

    try {
        const query = gql`
  query adSpaceInfo($spaceName: String!) {
    adSpaceInfo(spaceName: $spaceName)
  }
`
        let client = new GraphQLClient('https://blog.test/gql')
        client.setHeader('Authorization', 'Bearer ');

        const resp = await client.request(query, {
            spaceName: 'top'
        })

        console.log(resp.adSpaceInfo)

        const attributesBuffer = Buffer.from(resp.adSpaceInfo, 'base64');
        const codec = new BinaryCodec();
        const [decoded] = codec.decodeNested<Struct>(attributesBuffer, getStructure());
        const owner: BigNumber.Instance = decoded.getFieldValue('owner');
        const paid_amount: BigNumber.Instance = decoded.getFieldValue('paid_amount');
        const paid_until: BigNumber.Instance = decoded.getFieldValue('paid_until');

        console.log({
            owner: owner.toString(),
            paid_amount: parseFloat(paid_amount.div(new BigNumber(10).pow(6)).toString()),
            paid_until: parseInt(paid_until.toString()),
            is_new: decoded.getFieldValue('is_new'),
        })
    } catch (e) {
        console.log(e, e.inner)
    }


    // networkProvider.queryContract(new ContractQ)
})()
