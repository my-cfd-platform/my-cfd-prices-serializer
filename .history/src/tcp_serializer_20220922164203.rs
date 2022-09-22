use async_trait::async_trait;
use my_tcp_sockets::{
    socket_reader::{ReadBuffer, ReadingTcpContractFail, SocketReader},
    TcpSocketSerializer,
};

use super::bid_ask_contract::BidAskContract;

static CLCR: &[u8] = &[13u8, 10u8];
// const MAX_PACKET_CAPACITY: usize = 255;

pub struct SourceFeedSerializer {
    read_buffer: ReadBuffer,
}

impl SourceFeedSerializer {
    pub fn new() -> Self {
        Self {
            read_buffer: ReadBuffer::new(1024 * 24),
        }
    }
}

#[async_trait]
impl TcpSocketSerializer<BidAskContract> for SourceFeedSerializer {
    fn serialize(&self, _: BidAskContract) -> Vec<u8> {

        // let mut result = Vec::with_capacity(MAX_PACKET_CAPACITY);
        // contract.serialize(&mut result);
        // result.extend_from_slice(CLCR);
        // result
    }

    fn serialize_ref(&self, _: &BidAskContract) -> Vec<u8> {
        todo!()
        // let mut result = Vec::with_capacity(MAX_PACKET_CAPACITY);
        // contract.serialize(&mut result);
        // result.extend_from_slice(CLCR);
        // result
    }

    fn get_ping(&self) -> BidAskContract {
        return BidAskContract::Ping;
    }
    async fn deserialize<TSocketReader: Send + Sync + 'static + SocketReader>(
        &mut self,
        socket_reader: &mut TSocketReader,
    ) -> Result<BidAskContract, ReadingTcpContractFail> {
        let result = socket_reader
            .read_until_end_marker(&mut self.read_buffer, CLCR)
            .await?;

        let result = std::str::from_utf8(&result[..result.len() - CLCR.len()]).unwrap();

        Ok(BidAskContract::parse(result))
    }

    fn apply_packet(&mut self, _contract: &BidAskContract) -> bool {
        false
    }
}
