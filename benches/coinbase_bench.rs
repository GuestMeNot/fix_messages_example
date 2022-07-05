#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;
    use fix_messages_example::coinbase::{coinbase_fix_types, coinbase_quickfix_decoder, coinbase_quickfix_encoder};

    const INDICATION_OF_INTEREST_FIX_MSG: &[u8] = b"8=FIX.4.2|9=97|35=6|49=BKR|56=IM|34=14|52=20100204-09:18:42|23=115685|28=N|55=SPMI.MI|54=2|44=2200.75|27=S|25=H|10=248|";

    #[bench]
    fn sofh_encode_decode_bench(bencher: &mut Bencher) {
        let mut decoder = coinbase_quickfix_decoder();
        let mut encoder = coinbase_quickfix_encoder();
        bencher.iter(|| {
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

            let _decoded_msg = decoder
                .decode(encoded)
                .expect("Invalid FIX ExecutionReport message");
        });
    }

    #[bench]
    fn coinbase_decode_bench(bencher: &mut Bencher) {
        let mut decoder = coinbase_quickfix_decoder();
        bencher.iter(|| {
            let _msg = decoder
                .decode(INDICATION_OF_INTEREST_FIX_MSG)
                .expect("Invalid FIX IndicationOfInterest message");
        });
    }

}