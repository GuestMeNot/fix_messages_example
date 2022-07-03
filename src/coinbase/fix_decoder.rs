use fefix::prelude::*;
use fefix::tagvalue::{Config, Decoder, Encoder};

/// The `coinbase_fix_types` module is generated!
pub use crate::coinbase::coinbase_fix_types;

/// quickfix is a standard FIX library written in C.
/// Each ECN that uses FIX will have a slightly different interpretation of the specification.
const QUICKFIX_SPEC: &str = include_str!("../../resources/coinbase/coinbase_quickfix.xml");

/// Load the Coinbase FIX Spec. into the returned [Decoder].
pub fn coinbase_quickfix_decoder() -> Decoder<Config> {
    let fix_dictionary = Dictionary::from_quickfix_spec(QUICKFIX_SPEC).unwrap();
    let mut decoder = Decoder::<Config>::new(fix_dictionary);
    decoder.config_mut().set_separator(b'|');
    decoder
}

/// Load the example Coinbase FIX Spec. into the returned [Encoder].
///
pub fn coinbase_quickfix_encoder() -> Encoder<Config> {
    let mut config = Config::default();
    config.set_separator(b'|');
    // Perhaps this return value be cached in a global variable or Thread local.
    Encoder::<Config>::new(config)
}

#[cfg(test)]
mod tests {
    use crate::coinbase::{
        coinbase_fix_types, coinbase_quickfix_decoder, coinbase_quickfix_encoder,
    };
    use fefix::tagvalue::FieldAccess;

    const INDICATION_OF_INTEREST_FIX_MSG: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

    /// This test checks that FIX coinbase code generation minimally works.
    #[test]
    fn test_coinbase_decoder_indication_of_interest() {
        let mut decoder = coinbase_quickfix_decoder();
        let msg = decoder
            .decode(INDICATION_OF_INTEREST_FIX_MSG)
            .expect("Invalid FIX IndicationOfInterest message");
        assert_eq!(
            msg.fv(coinbase_fix_types::BEGIN_STRING),
            Ok(coinbase_fix_types::BeginString::Fix42)
        );
        assert_eq!(
            msg.fv(coinbase_fix_types::MSG_TYPE),
            Ok(coinbase_fix_types::MsgType::IndicationOfInterest)
        );
    }

    /// This test checks that FIX coinbase encoding and decoding minimally works.
    #[test]
    fn test_coinbase_fix_encode_decode() {
        let mut encoder = coinbase_quickfix_encoder();
        let mut buffer = Vec::new();
        let mut msg = encoder.start_message(b"FIX.4.2", &mut buffer, b"ExecutionReport");
        msg.set(
            coinbase_fix_types::MSG_TYPE,
            coinbase_fix_types::MsgType::ExecutionReport,
        );
        msg.set(coinbase_fix_types::MSG_SEQ_NUM, 215 as i32);
        msg.set(coinbase_fix_types::SENDER_COMP_ID, "CLIENT12");
        msg.set(coinbase_fix_types::TARGET_COMP_ID, "B");
        msg.set(coinbase_fix_types::ACCOUNT, "Yogi Bear");
        msg.set(coinbase_fix_types::CL_ORD_ID, "13346");
        msg.set(
            coinbase_fix_types::HANDL_INST,
            coinbase_fix_types::HandlInst::AutomatedExecutionOrderPrivateNoBrokerIntervention,
        );
        msg.set(
            coinbase_fix_types::ORD_TYPE,
            coinbase_fix_types::OrdType::Limit,
        );
        msg.set(coinbase_fix_types::PRICE, 150.08);
        msg.set(coinbase_fix_types::SIDE, coinbase_fix_types::Side::Buy);
        msg.set(
            coinbase_fix_types::TIME_IN_FORCE,
            coinbase_fix_types::TimeInForce::ImmediateOrCancel,
        );
        let encoded = msg.wrap();

        let mut decoder = coinbase_quickfix_decoder();
        let decoded_msg = decoder
            .decode(encoded)
            .expect("Invalid FIX ExecutionReport message");

        assert_eq!(
            decoded_msg.fv(coinbase_fix_types::BEGIN_STRING),
            Ok(coinbase_fix_types::BeginString::Fix42)
        );
        assert_eq!(
            decoded_msg.fv(coinbase_fix_types::MSG_TYPE),
            Ok(coinbase_fix_types::MsgType::ExecutionReport)
        );
        assert_eq!(decoded_msg.fv(coinbase_fix_types::MSG_SEQ_NUM), Ok(215));
        assert_eq!(decoded_msg.fv(coinbase_fix_types::PRICE), Ok(150.08));
    }
}
