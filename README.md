# ⚪ Ordinal canister

[Ordinal](https://ordinals.com/) [theory](https://docs.ordinals.com/) ascribes numismatic worth to satoshis, enabling their collection and trade as intriguing items. Each individual satoshi can bear arbitrary content inscriptions, resulting in distinct digital artifacts native to Bitcoin. These artifacts can be stored in Bitcoin wallets and exchanged through Bitcoin transactions. The inscriptions mirror the durability, immutability, security, and decentralization inherent in Bitcoin.

The [ordinal canister](https://dashboard.internetcomputer.org/canister/tn6q3-wqaaa-aaaap-abuca-cai) enables the retrieval of ordinals and their corresponding inscriptions, making them easily accessible on the IC. 

## 🔎 How does it work ?

The [Internet Computer](https://internetcomputer.org/) enable canisters to directly make calls to HTTP(S) servers external to the blockchain. This capability is achieved through a mechanism known as [HTTP outcalls](https://internetcomputer.org/docs/current/developer-docs/integrations/https-outcalls/https-outcalls-how-it-works).

The ordinal canister uses this mechanism to access and aggregate data from various ordinal APIs. Currently, two providers, [Hiro](https://docs.hiro.so/ordinals/) and [Bitgem](https://docs.bitgem.tech/), are employed for this purpose.

## 📜 Public interface

To query ordinal information from the ordinal canister, you have two options: use one of the specific named functions or employ the generic request method.

### Specific functions

```
bitgem_sat_range         : (sat_range_args)           -> (sat_range_result);
bitgem_sat_info          : (sat_info_args)            -> (sat_info_result);
hiro_sat_info            : (sat_info_args)            -> (sat_info_result);
hiro_sat_inscriptions    : (sat_inscriptions_args)    -> (hiro_sat_inscriptions_result);
hiro_inscription_info    : (inscription_info_args)    -> (hiro_sat_inscription_result);
hiro_inscription_content : (inscription_content_args) -> (hiro_inscription_content_result);
hiro_brc20_details       : (brc20_details_args)       -> (brc20_details_result);
hiro_brc20_holders       : (brc20_holders_args)       -> (brc20_holders_result);
```
(See btc_ordinals.did for the types definition)

Each function is prefixed by the provider used to retrieve the associated data. In contrast to the generic request method, these functions have a fixed maximum KB per item (required by the HTTP outcall). They provide an intuitive way to query ordinal information.

### The generic `request` method

```
request                  : (ord_args)       -> (multi_ord_result);
```
where 
```
  type ord_args = record {
    function: ord_function;
    providers: vec provider;
    max_kb_per_item: opt nat64;
  };
  type provider = variant {
    Hiro;
    Bitgem;
  };
  type ord_function = variant {
    SatRange:           record { utxo           : utxo;           };
    SatInfo:            record { ordinal        : nat64;          };
    SatInscriptions:    record { ordinal        : nat64;          };
    InscriptionInfo:    record { inscription_id : inscription_id; };
    InscriptionContent: record { inscription_id : inscription_id; };
    Brc20Details:       record { ticker         : ticker;         };
    Brc20Holders:       record { ticker         : ticker;         };
  };
  type multi_ord_result = variant {
    Consistent: response_result;
    Inconsistent: vec record { provider: provider; result: response_result; };
  };
```

Here, args is a record with fields for the function and maximum KB per item. The function variant can be one of the specific functions listed above.

This method allows querying the same data as the specific functions, offering the flexibility to choose the provider and override the maximum KB per item. Note if the list of providers is left empty, all available providers are taken.

The request method supports querying ordinal information through multiple ordinal APIs (if available), returning a multi_ord_result. This result can be either Consistent or Inconsistent depending on whether the outcomes are the same or different across different APIs.

Currently, the only function that can be queried through more than one API (and hence potentially returning an inconsistent result) is the SatInfo function.

### The `request_cost` method

```
request_cost : (ord_args) -> (request_cost_result);
```

This function takes the same arguments as the `request` method. It returns how many cycles you need to send with the request for the given `ord_args`. Providing more cycles will succeed. Providing less cycles will return an error.

## 🔧 Deploy the smart contract locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id={local_btc_ordinals}`.

## 🙋 Examples (local replica)

```bash
# Get information for the satoshi 85000000000 via the bitgem provider
dfx canister call btc_ordinals bitgem_sat_info '(record { ordinal = 85000000000 })' --with-cycles 1000000000 --wallet $(dfx identity get-wallet)
# Get the inscriptions associated to the satoshi 947410401228752 via the hiro provider (no control over query options or max kb per item)
dfx canister call btc_ordinals hiro_sat_inscriptions '(record { ordinal = 947410401228752; limit = 10; offset = 0;})' --with-cycles 1000000000 --wallet $(dfx identity get-wallet)

# To get the request cost
dfx canister call btc_ordinals request_cost '(record { function = variant { SatInscriptions = record { ordinal = 947410401228752; offset = 10; limit = 1; } }; providers = vec { variant { Hiro } }; max_kb_per_item = opt 2; })'

# Get the last inscription associated with the satoshi 947410401228752, specifying a max of 2KB per item.
dfx canister call btc_ordinals request '(record { function = variant { SatInscriptions = record { ordinal = 947410401228752; offset = 10; limit = 1; } }; providers = vec { variant { Hiro } }; max_kb_per_item = opt 2; })' --with-cycles ${REQUEST_COST} --wallet $(dfx identity get-wallet)
```

See the `EXAMPLES` file for more.

## 💾 Default Maximum Kilobytes per Item

>SatRange: 1 KiB
>SatInfo: 1 KiB
>SatInscriptions: 2 KiB
>InscriptionInfo: 2 KiB
>InscriptionContent: 5 KiB (chosen arbitrarily)
>Brc20Details: 2 KiB
>Brc20Holders: 1 KiB

These values (except for InscriptionContent) were determined by examining several responses and selecting the next kilobyte as the maximum allowed.

## 🦺 Pending improvements (TODO in the code)

- [ ] Store the results in a stable data structure to avoid having to make the same http outcall over and over for the same data
- [ ] Run the send requests in parallel in the `request` function

## ℹ️ Notes

- If you get the error `HttpSendError = record { rejection_code = variant { SysFatal } }`, it's probably that the max_kb_per_item is too low compared to the size of the returned response
- For an unknown reason, when running the e2e `get_sat_ranges` function locally, the replica returns the error: `reject code CanisterReject, reject message Canister ghsi2-tqaaa-aaaan-aaaca-cai not found, error code None`

## 🙏 Credits

- This development is sponsored by a [bounty](https://forum.dfinity.org/t/open-bnt-9-ordinals-canister/21769) offered by [DFinity](https://dfinity.org/)
- Inspiration from the [EVM-RPC canister](https://github.com/internet-computer-protocol/ic-eth-rpc) and [exchange rate canister](https://github.com/dfinity/exchange-rate-canister)
