import Nat8 "mo:base/Nat8";
import Char "mo:base/Char";
import Array "mo:base/Array";

// From https://github.com/aviate-labs/encoding.mo/blob/main/src/Hex.mo
module {

    private let base : Nat8   = 16;
    private let hex  : [Char] = [
        '0', '1', '2', '3', 
        '4', '5', '6', '7', 
        '8', '9', 'a', 'b', 
        'c', 'd', 'e', 'f',
    ];
    
    public type Hex = Text;

    // Converts a byte to its corresponding hexidecimal format.
    public func encodeByte(n : Nat8) : Hex {
        let c0 = hex[Nat8.toNat(n / base)];
        let c1 = hex[Nat8.toNat(n % base)];
        Char.toText(c0) # Char.toText(c1);
    };

    // Converts an array of bytes to their corresponding hexidecimal format.
    public func encode(ns : [Nat8]) : Hex {
        Array.foldRight<Nat8, Hex>(
            ns, 
            "", 
            func(n : Nat8, acc : Hex) : Hex {
                acc # encodeByte(n);
            },
        );
    };

};