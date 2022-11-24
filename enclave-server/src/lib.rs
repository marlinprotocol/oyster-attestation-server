use nsm_io::{Request, Response};
use serde_bytes::ByteBuf;

pub fn get_attestation_doc(pub_key: [u8;32]) -> Vec<u8> {
    let nsm_fd = nsm_driver::nsm_init();

    let public_key = ByteBuf::from(pub_key);
    let hello = ByteBuf::from("");

    let request = Request::Attestation {
        public_key: Some(public_key),
        user_data: Some(hello),
        nonce: None,
    };

    let response = nsm_driver::nsm_process_request(nsm_fd, request);

    nsm_driver::nsm_exit(nsm_fd);

    match response {
        Response::Attestation { document } => document,
        _ => panic!("nsm driver returned invalid response: {:?}", response),
    }
}
