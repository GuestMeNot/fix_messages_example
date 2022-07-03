
This example crate shows the generation of [FIX](https://www.fixtrading.org/)
FIX message encoding and decoding using [code generation](https://github.com/ferrumfix/ferrumfix)
provided by the [`fefix`](https://crates.io/crates/fefix) crate.

### Quickfix

This example is intended to show a simple [`quickfix`](https://github.com/quickfix/quickfix)
compatible implementation of FIX messages. Each [ECN](https://www.investopedia.com/terms/e/ecn.asp)
interprets the FIX message fields, [Layer 1](https://www.fixtrading.org/standards/fix-session-layer-online/)
and message passing protocol uniquely.

### FIX Layer 1

Includes Session Management such as Logon, Logoff, Heartbeats, etc.

### Blocking 

FIX is inherently a blocking protocol. Messages have a built-in sequence number
and must be processed in that order. 

One can imagine modifications to this protocol which only blocks on sequence numbers 
by account.

### Message Passing after Session Setup

Here is a good example of message passing after session setup:

<https://www.fixsim.com/fix-protocol-tutorial>

For example, after a FIX [New Order](https://www.onixs.biz/fix-dictionary/4.4/msgType_D_68.html), 
one or more [Execution Reports](https://www.onixs.biz/fix-dictionary/4.4/msgType_8_8.html) messages 
may be returned to the caller.

[A complete list of FIX messages](https://www.onixs.biz/fix-dictionary/4.4/msgs_by_msg_type.html). 

### SOFH

Included is a [SOFH](https://www.fixtrading.org/standards/fix-sofh/) implementation.
A SOFH header is a fixed size and intended to be passed in before sending a message 
on the socket inform the receiver of the exact number of FIX message bytes to be 
read and the encoding mechanism of these bytes.


