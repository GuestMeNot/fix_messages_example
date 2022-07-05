#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use fix_messages_example::sofh::SofhHeader;
    use test::Bencher;


    #[bench]
    fn sofh_creation_bench(bencher: &mut Bencher) {
        bencher.iter(|| {
            let _header = SofhHeader {
                message_length: 1,
                encoding_type: 2,
            };
        });
    }

    #[bench]
    fn sofh_encode_decode_bench(bencher: &mut Bencher) {
        let header = SofhHeader {
            message_length: 1,
            encoding_type: 2,
        };
        bencher.iter(|| {
            let encoded: [u8; 6] = header.into();
            let _decoded: SofhHeader = SofhHeader::from(&encoded);
        });
    }

    #[bench]
    fn sofh_encode_bench(bencher: &mut Bencher) {
        let header = SofhHeader {
            message_length: 1,
            encoding_type: 2,
        };
        bencher.iter(|| {
            let _encoded: [u8; 6] = header.into();
        });
    }

}