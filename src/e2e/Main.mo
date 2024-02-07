import BtcOrdinalsCanister "canister:btc_ordinals";

import Hex                 "Hex";

import Array               "mo:base/Array";
import Buffer              "mo:base/Buffer";
import Debug               "mo:base/Debug";
import Blob                "mo:base/Blob";
import ExperimentalCycles  "mo:base/ExperimentalCycles";

shared actor class Main() {

    type Satoshi = Nat64;

    type BitcoinNetwork = {
        #mainnet;
        #testnet;
    };

    type BitcoinAddress = Text;

    type BlockHash = Blob;

    type Outpoint = {
        txid : Blob;
        vout : Nat32
    };

    type Utxo = {
        outpoint: Outpoint;
        value: Satoshi;
        height: Nat32;
    };

    type GetUtxosRequest = {
        address : BitcoinAddress;
        network: BitcoinNetwork;
        filter: ?{
            #min_confirmations: Nat32;
            #page: Blob;
        };
    };

    type GetCurrentFeePercentilesRequest = {
        network: BitcoinNetwork;
    };

    type GetUtxosResponse = {
        utxos: [Utxo];
        tip_block_hash: BlockHash;
        tip_height: Nat32;
        next_page: ?Blob;
    };

    type GetBalanceRequest = {
        address : BitcoinAddress;
        network: BitcoinNetwork;
        min_confirmations: Nat32;
    };

    type SendTransactionRequest = {
        transaction: Blob;
        network: BitcoinNetwork;
    };

    type MillisatoshiPerByte = Nat64;

    // Bitcoin interface
    type BitcoinInterface = actor {
        bitcoin_get_balance                 : shared       (GetBalanceRequest)               -> async (Satoshi);
        bitcoin_get_balance_query           : shared query (GetBalanceRequest)               -> async (Satoshi);
        bitcoin_get_utxos                   : shared       (GetUtxosRequest)                 -> async (GetUtxosResponse);
        bitcoin_get_utxos_query             : shared query (GetUtxosRequest)                 -> async (GetUtxosResponse);
        bitcoin_send_transaction            : shared       (SendTransactionRequest)          -> async ();
        bitcoin_get_current_fee_percentiles : shared       (GetCurrentFeePercentilesRequest) -> async ([MillisatoshiPerByte]);
    };

    let management_canister : BitcoinInterface = actor("aaaaa-aa");
    
    public shared func get_sat_ranges(address: BitcoinAddress, exclude_common_ranges: Bool, max_response_kb: Nat64) : async BtcOrdinalsCanister.multi_ord_result {

        let utxos_request : GetUtxosRequest = {
            address;
            network = #mainnet;
            filter = null;
        };
        
        ExperimentalCycles.add(10_000_000_000); // 10 billions cycles are required for bitcoin_get_utxos
        let utxos_response = await management_canister.bitcoin_get_utxos(utxos_request);

        let utxos = Buffer.Buffer<BtcOrdinalsCanister.utxo>(utxos_response.utxos.size());
        for (utxo in Array.vals(utxos_response.utxos)) {
            utxos.add({
                txid = Hex.encode(Blob.toArray(utxo.outpoint.txid)); 
                vout = utxo.outpoint.vout;
            }); 
        };

        let sat_range_args = {
            function = #SatRange({
                utxos = Buffer.toArray(utxos);
                exclude_common_ranges = true;
            });
            providers = [];
            max_kb_per_item = ?max_response_kb;
        };
        ignore payCost(await BtcOrdinalsCanister.request_cost(sat_range_args), 0);
        await BtcOrdinalsCanister.request(sat_range_args)
    };

    public shared func test() : async () {

        let initial_balance = ExperimentalCycles.available();
        var total : Nat = 0;

        // TODO: SatRange is commented out because BitGem API has been publicly retired.
        // If you want to have access to BitGem endpoints, you need to self host it: https://github.com/BitGemTech/exotic-indexer.

//        // sat_range 
//        let sat_range_args : BtcOrdinalsCanister.ord_args = { 
//            function = #SatRange({
//                utxos = [{ txid = "0a4ae1923b59e545e82dc7067965fe02304635db665806dee76e7ead7e002d41"; vout = 1; }];
//                exclude_common_ranges = true;
//            });
//            providers = [];
//            max_kb_per_item = ?2;
//        };
//        total := payCost(await BtcOrdinalsCanister.request_cost(sat_range_args), total);
//        assertOk("SatRange", await BtcOrdinalsCanister.request(sat_range_args));

        // sat_info 
        let sat_info_args : BtcOrdinalsCanister.ord_args = { 
            function = #SatInfo({
                ordinal = 85000000000;
            });
            providers = [#Hiro];
            max_kb_per_item = ?1;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(sat_info_args), total);
        assertOk("SatInfo", await BtcOrdinalsCanister.request(sat_info_args));
        
        // sat_inscriptions
        let sat_inscriptions_args : BtcOrdinalsCanister.ord_args = { 
            function = #SatInscriptions({
                ordinal = 85000000000;
                offset = 0;
                limit = 5;
            });
            providers = [];
            max_kb_per_item = ?1;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(sat_inscriptions_args), total);
        assertOk("SatInscriptions", await BtcOrdinalsCanister.request(sat_inscriptions_args));

        // inscription_info 
        let inscription_info_args : BtcOrdinalsCanister.ord_args = { 
            function = #InscriptionInfo({
                inscription_id = "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0";
            });
            providers = [];
            max_kb_per_item = ?2;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(inscription_info_args), total);
        assertOk("InscriptionInfo", await BtcOrdinalsCanister.request(inscription_info_args));
        
        // inscription_content
        let inscription_content_args : BtcOrdinalsCanister.ord_args = { 
            function = #InscriptionContent({
                inscription_id = "38c46a8bf7ec90bc7f6b797e7dc84baa97f4e5fd4286b92fe1b50176d03b18dci0";
            });
            providers = [];
            max_kb_per_item = ?2;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(inscription_content_args), total);
        assertOk("InscriptionContent", await BtcOrdinalsCanister.request(inscription_content_args));
        
        // brc20_details
        let brc20_details_args : BtcOrdinalsCanister.ord_args = { 
            function = #Brc20Details({
                ticker = "ordi";
            });
            providers = [];
            max_kb_per_item = ?2;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(brc20_details_args), total);
        assertOk("Brc20Details", await BtcOrdinalsCanister.request(brc20_details_args));
        
        // brc20_holders
        let brc20_holders_args : BtcOrdinalsCanister.ord_args = { 
            function = #Brc20Holders({
                ticker = "ordi";
                offset = 5;
                limit = 5;
            });
            providers = [];
            max_kb_per_item = ?2;
        };
        total := payCost(await BtcOrdinalsCanister.request_cost(brc20_holders_args), total);
        assertOk("Brc20Holders", await BtcOrdinalsCanister.request(brc20_holders_args));

        let final_balance = ExperimentalCycles.available();
        Debug.print("Total cycles used: " # debug_show total);
        Debug.print("Initial cycles balance: " # debug_show initial_balance);
        Debug.print("Final cycles balance: " # debug_show final_balance);
        // TODO: Somehow in local environment, the motoko ExperimentalCycles.available() seems to always return 0 
        //assert (initial_balance == final_balance + total);
    };

    func assertOk<T>(method : Text, result : BtcOrdinalsCanister.multi_ord_result) {
        switch result {
            case (#Consistent(#Ok _)) {};
            case (#Consistent(#Err err)) {
                Debug.trap("received error for " # method # ": " # debug_show err);
            };
            case (#Inconsistent(results)) {
                for ({provider; result;} in results.vals()) {
                    switch result {
                        case (#Ok(_)) {};
                        case (#Err(err)) {
                            Debug.trap("received error in inconsistent results for " # debug_show provider # " " # method # ": " # debug_show err);
                        };
                    };
                };
            };
        };
    };

    func payCost(cost_result : BtcOrdinalsCanister.request_cost_result, accumulator: Nat) : Nat {
        switch cost_result {
            case (#Ok cost) { 
                ExperimentalCycles.add(cost);
                accumulator + cost
            };
            case (#Err err) { Debug.trap("failed to get request cycles cost: " # debug_show err); };
        };
    };

};