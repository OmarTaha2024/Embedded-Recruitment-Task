use prost::Message;

/// Represents a simple echo message
#[derive(Clone, PartialEq, Message)]
pub struct EchoMessage {
    #[prost(string, tag = "1")]
    pub content: String,
}

/// Represents a request for adding two numbers
#[derive(Clone, PartialEq, Message)]
pub struct AddRequest {
    #[prost(int32, tag = "1")]
    pub a: i32,
    #[prost(int32, tag = "2")]
    pub b: i32,
}

/// Represents the response to an addition request
#[derive(Clone, PartialEq, Message)]
pub struct AddResponse {
    #[prost(int32, tag = "1")]
    pub result: i32,
}

/// Represents a message sent by the client
#[derive(Clone, PartialEq, Message)]
pub struct ClientMessage {
    #[prost(oneof = "client_message::Message", tags = "1, 2")]
    pub message: Option<client_message::Message>,
}

/// Nested module to define client message variants
pub mod client_message {
    use super::{AddRequest, EchoMessage};
    use prost::Oneof;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        EchoMessage(EchoMessage),
        #[prost(message, tag = "2")]
        AddRequest(AddRequest),
    }
}

/// Represents a message sent by the server
#[derive(Clone, PartialEq, Message)]
pub struct ServerMessage {
    #[prost(oneof = "server_message::Message", tags = "1, 2")]
    pub message: Option<server_message::Message>,
}

/// Nested module to define server message variants
pub mod server_message {
    use super::{AddResponse, EchoMessage};
    use prost::Oneof;

    #[derive(Clone, PartialEq, Oneof)]
    pub enum Message {
        #[prost(message, tag = "1")]
        EchoMessage(EchoMessage),
        #[prost(message, tag = "2")]
        AddResponse(AddResponse),
    }
}
