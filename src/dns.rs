
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
