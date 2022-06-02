use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum Message {
    SendAuthCode { discord_id : u64, code : String }, 
}


/* use std::rc::Rc;


#[derive(Serialize, Deserialize, Debug)]
pub enum TcpMessageType{
    MsgSendAuthCode, 
}

/*pub struct TcpMessage<'a> {
    m_type : TcpMessageType, 
    m_inner : &'adyn MsgInner<'a> 
}

impl Serialize for TcpMessage<'_>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Color", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

pub trait MsgInner<'a> : Serialize + Deserialize<'a> + std::fmt::Debug{}
*/ 
#[derive(Serialize, Deserialize, Debug)]
pub struct MsgSendAuthCode{
    discord_user_id : u64, 
    code : String, 
}

// impl MsgInner<'_> for MsgSendAuthCode {} */ 