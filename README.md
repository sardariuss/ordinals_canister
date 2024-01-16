# Ordinal canister

[Ordinal](https://ordinals.com/) [theory](https://docs.ordinals.com/) ascribes numismatic worth to satoshis, enabling their collection and trade as intriguing items. Each individual satoshi can bear arbitrary content inscriptions, resulting in distinct digital artifacts native to Bitcoin. These artifacts can be stored in Bitcoin wallets and exchanged through Bitcoin transactions. The inscriptions mirror the durability, immutability, security, and decentralization inherent in Bitcoin.

The [ordinal canister](https://dashboard.internetcomputer.org/canister/tn6q3-wqaaa-aaaap-abuca-cai) enables the retrieval of ordinals and their corresponding inscriptions, making them easily accessible on the IC. 

## 🔎 How does it work ?

The [Internet Computer](https://internetcomputer.org/) enable canisters to directly make calls to HTTP(S) servers external to the blockchain. This capability is achieved through a mechanism known as [HTTP outcalls](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-how-it-works).

The ordinal canister uses this mechanism to access and aggregate data from various ordinal APIs. Currently, two providers, [Hiro](https://docs.hiro.so/) and [Bitgem](https://docs.bitgem.tech/), are employed for this purpose.

## 📜 Public interface

To query ordinal information from the ordinal canister, you have two options: use one of the specific named functions or employ the generic request method.

### Specific functions

```
  bitgem_sat_range         : (utxo)           -> (sat_range_result);
  bitgem_sat_info          : (nat64)          -> (sat_info_result);
  hiro_sat_info            : (nat64)          -> (sat_info_result);
  hiro_sat_inscriptions    : (nat64)          -> (hiro_sat_inscriptions_result);
  hiro_inscription_info    : (inscription_id) -> (hiro_sat_inscription_result);
  hiro_inscription_content : (inscription_id) -> (hiro_inscription_content_result);
  hiro_brc20_details       : (ticker)         -> (brc20_details_result);
  hiro_brc20_holders       : (ticker)         -> (brc20_holders_result);
```

Each function is prefixed by the provider used to retrieve the associated data. In contrast to the generic request method, these functions have a fixed maximum KB per item (required by the HTTP outcall) and fixed query options (if applicable). They provide an intuitive way to query ordinal information.

### The generic `request` method

```
request                  : (args)           -> (multi_ord_result);
```
where 
```
  type args = record {
    function: function;
    query_options: opt query_options;
    max_kb_per_item: opt nat64;
  };
  type function = variant {
    SatRange:           record { utxo           : utxo;           };
    SatInfo:            record { ordinal        : nat64;          };
    SatInscriptions:    record { ordinal        : nat64;          };
    InscriptionInfo:    record { inscription_id : inscription_id; };
    InscriptionContent: record { inscription_id : inscription_id; };
    Brc20Details:       record { ticker         : ticker;         };
    Brc20Holders:       record { ticker         : ticker;         };
  };
  type query_options = record {
    limit: nat64;
    offset: nat64;
  };
```

Here, args is a record with fields for the function, query options, and maximum KB per item. The function variant can be one of the specific functions listed above. The query_options record includes limits and offsets.

This method allows querying the same data as the specific functions, offering the flexibility to override default parameters such as maximum KB per item, query limit, and offset.

The request method supports querying ordinal information through multiple ordinal APIs (if available), returning a multi_ord_result. This result can be either Consistent or Inconsistent depending on whether the outcomes are the same or different across different APIs.

```
type multi_ord_result = variant {
  Consistent: response_result;
  Inconsistent: vec record { provider: provider; result: response_result; };
};
```

Currently, the only function that can be queried through more than one API (and hence potentially returning an inconsitent result) is the SatInfo function.

## 🔧 Deploy the smart contract locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id={local_btc_ordinals}`.

## 🦺 Pending improvements (TODO in the code)

- [ ] Store the results in a stable data structure to avoid having to make the same http outcall over and over for the same data
- [ ] Verify the base cycles costs and computation performed to obtain the cost of processing an http outcall
- [ ] Create an end-to-end test canister to thoroughly validate the `btc_ordinals` canister

## 🙏 Credits

- This development is sponsored by a [bounty](https://forum.dfinity.org/t/open-bnt-9-ordinals-canister/21769) offered by [DFinity](https://dfinity.org/)
- Inspiration from the [EVM-RPC canister](https://github.com/internet-computer-protocol/ic-eth-rpc) and [exchange rate canister](https://github.com/dfinity/exchange-rate-canister)
