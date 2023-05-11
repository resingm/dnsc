
use std::net::SocketAddr;

use trust_dns_proto::{error, op, rr};

use crate::util;

/// TODO: Add documentation
pub fn build_query(qid: u16, domain_name: &str, query_type: rr::record_type::RecordType) -> error::ProtoResult<op::Message> {
    let mut q = op::Message::new();

    q.set_id(qid);

    q.set_message_type(op::MessageType::Query);
    q.set_op_code(op::OpCode::Query);
    q.set_recursion_desired(true);

    let domain_name = rr::domain::Name::from_ascii(domain_name)?;
    let query_record = op::Query::query(
        domain_name,
        query_type,
    );

    q.add_query(query_record);

    Ok(q)
}

/// TODO: Add documentation
pub fn map_return_code(rcode: &op::ResponseCode) -> &str {
    match rcode {
        op::ResponseCode::NoError => "NOERROR",     //  0
        op::ResponseCode::FormErr => "FORMERR",     //  1 Format Error    
        op::ResponseCode::ServFail => "SERVFAIL",   //  2 Server Failure
        op::ResponseCode::NXDomain => "NXDomain",   //  3 Non-Existant domain
        op::ResponseCode::NotImp => "NOTIMP",       //  4 Not implenmented
        op::ResponseCode::Refused => "REFUSED",     //  5 Query refused
        op::ResponseCode::YXDomain => "YXDOMAIN",   //  6 Name that should not exist, does exist
        op::ResponseCode::YXRRSet => "YXRRSET",     //  7 RR set that should not exist, does exist
        op::ResponseCode::NXRRSet => "NXRRSET",     //  8 RR set does not exist, but should
        op::ResponseCode::NotAuth => "NOTAUTH",     //  9 Server not authorized for the zone
        op::ResponseCode::NotZone => "NOTZONE",     // 10 Name not in zone
        // 11 - 15 is unassigned
        op::ResponseCode::BADVERS => "BADVERS",     // 16
        op::ResponseCode::BADSIG => "BADSIG",       // 16
        op::ResponseCode::BADKEY => "BADKEY",       // 17
        op::ResponseCode::BADTIME => "BADTIME",     // 18
        op::ResponseCode::BADMODE => "BADMODE",     // 19
        op::ResponseCode::BADNAME => "BADNAME",     // 20
        op::ResponseCode::BADALG => "BADALG",       // 21
        op::ResponseCode::BADTRUNC => "BADTRUNC",   // 22
        op::ResponseCode::BADCOOKIE => "BADCOOKIE", // 23
        op::ResponseCode::Unknown(_) => "UNKNOWN",  // 
    }
}

// Add documentation
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

/// TODO: Add documentation
pub fn response_to_csv(src_addr: SocketAddr, r: op::Message) {
    let (q_name, q_type, q_class) = if r.query_count() > 0 {
        let q = r.query().into_iter().next().unwrap();
        (q.name().to_string(), q.query_type().to_string(), q.query_class().to_string())
    } else {
        (String::from(""), String::from(""), String::from(""))
    };

    if r.response_code() == op::ResponseCode::NoError {
        for answer in r.answers() {
            util::log(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}",
                src_addr.ip().to_string(),
                src_addr.port().to_string(),
                q_name,
                q_type,
                q_class,
                map_return_code(&r.response_code()),
                answer.name(),
                answer.rr_type(),
                answer.dns_class(),
                answer.ttl(),
                answer.data().unwrap(),
            ));
        }
    } else {
        util::log(&format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            src_addr.ip().to_string(),
            src_addr.port().to_string(),
            q_name,
            q_type,
            q_class,
            r.response_code().to_string(),
            "",
            "",
            "",
            "",
            "",
        ));

        util::err(&format!("DNS Response error: {:?} - {:?}", r.response_code(), r));
    }
}