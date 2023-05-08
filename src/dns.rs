
use trust_dns_proto::{error, op, rr};

pub fn build_query(qid: u16, domain_name: &str) -> error::ProtoResult<op::Message> {
    let mut q = op::Message::new();

    q.set_id(qid);

    q.set_message_type(op::MessageType::Query);
    q.set_op_code(op::OpCode::Query);
    q.set_recursion_desired(true);

    let domain_name = rr::domain::Name::from_ascii(domain_name)?;
    let query_record = op::Query::query(
        domain_name,
        rr::record_type::RecordType::A,
    );

    q.add_query(query_record);

    Ok(q)
}


pub fn parse_query(buf: &[u8]) -> error::ProtoResult<op::Message> {
    let r = op::Message::from_vec(buf)?;
    Ok(r)
}


// Parsed Parsed query from 8.8.8.8:53: Message {
    // header: Header {
        // id: 6,
        // message_type: Response,
        // op_code: Query,
        // authoritative: false,
        // truncation: false,
        // recursion_desired: true,
        // recursion_available: true,
        // authentic_data: false,
        // checking_disabled: false,
        // response_code: NoError,
        // query_count: 1,
        // answer_count: 1,
        // name_server_count: 0,
        // additional_count: 0
    // }, queries: [
        // Query { 
            // name: Name("openintel.nl."),
            // query_type: A, 
            // query_class: IN
        // }
    // ], answers: [
        // Record {
            // name_labels: Name("openintel.nl."),
            // rr_type: A,
            // dns_class: IN,
            // ttl: 600,
            // rdata: Some(A(145.97.20.4))
        // }
    // ], name_servers: [],
    // additionals: [],
    // signature: [],
    // edns: None
// }

pub fn response_to_csv(r: op::Message) {
    if r.response_code() == op::ResponseCode::NoError {
        for answer in r.answers() {
            println!(
                "{},{},{},{},{}",
                answer.name(),
                answer.rr_type(),
                answer.dns_class(),
                answer.ttl(),
                answer.data().unwrap(),
            );
        }

    } else {
        eprintln!("DNS Response error: {:?} - {:?}", r.response_code(), r);
    }
}