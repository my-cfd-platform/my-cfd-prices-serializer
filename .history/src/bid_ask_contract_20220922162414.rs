use super::BidAskDateTime;

#[derive(Debug, Clone)]
pub enum BidAskContract {
    Ping,
    Pong,
    BidAsk(BidAsk),
}

impl BidAskContract {
    pub fn is_ping(&self) -> bool {
        match self {
            BidAskContract::Ping => true,
            _ => false,
        }
    }

    pub fn parse(src: &str) -> Self {
        if src == "PING" {
            return Self::Ping;
        }
        if src == "PONG" {
            return Self::Pong;
        }

        Self::BidAsk(BidAsk::parse(src).unwrap())
    }

    pub fn serialize(&self, dest: &mut Vec<u8>) {
        match self {
            BidAskContract::Ping => dest.extend_from_slice(b"PING"),
            BidAskContract::Pong => dest.extend_from_slice(b"PONG"),
            BidAskContract::BidAsk(bid_ask) => bid_ask.serialize(dest),
        }
    }

    pub fn is_bid_ask(&self) -> bool {
        match self {
            BidAskContract::Ping => false,
            BidAskContract::Pong => false,
            BidAskContract::BidAsk(_) => true,
        }
    }
}

impl my_tcp_sockets::tcp_connection::TcpContract for BidAskContract {
    fn is_pong(&self) -> bool {
        match self {
            BidAskContract::Pong => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BidAsk {
    pub date_time: BidAskDateTime,
    pub id: String,
    pub bid: f64,
    pub ask: f64,
}

impl BidAsk {
    pub fn parse(src: &str) -> Option<Self> {
        let mut date_time = None;
        let mut id = None;
        let mut bid = None;
        let mut ask = None;
        let mut no = 0;

        for line in src.split(' ') {
            match no {
                0 => id = Some(line.to_string()),
                1 => {
                    date_time = BidAskDateTime::parse(line).into();
                }
                2 => bid = Some(line.parse::<f64>().unwrap()),
                3 => ask = Some(line.parse::<f64>().unwrap()),
                _ => {}
            }
            no += 1;
        }

        let date_time = date_time?;
        let id = id?;
        let bid = bid?;
        let ask = ask?;

        Self {
            date_time,
            id,
            bid,
            ask,

        }
        .into()
    }

    // pub fn serialize(&self, dest: &mut Vec<u8>) {
    //     self.date_time.serialize(dest);

    //     dest.push(' ' as u8);
    //     dest.extend_from_slice(self.id.as_bytes());
    //     dest.push(' ' as u8);

    //     dest.extend_from_slice(self.bid.to_string().as_bytes());
    //     dest.push(' ' as u8);
    //     dest.extend_from_slice(self.ask.to_string().as_bytes());
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let result =
            BidAsk::parse("GBPUSD 20220921123348100 1.13401000 1.13408000").unwrap();
        // let mut result_str = Vec::new();
        // result.serialize(&mut result_str);

        let date_time = result.date_time.unwrap_as_our_date();

        assert_eq!(result.id, "GBPUSD");
        assert_eq!(result.bid.to_string(), "1.13401");
        assert_eq!(result.ask.to_string(), "1.13408");
        assert_eq!("2022-09-21T12:33:48.100", &date_time.to_rfc3339()[..23]);

        // let result_str = String::from_utf8(result_str).unwrap();

        // assert_eq!(
        //     "GBPUSD 20220921123348100 1.13401000 1.13408000",
        //     result_str
        // );
    }

    #[test]
    fn test_our_time_parse() {
        let src = "GBPUSD 20220921123348000 1.13401000 1.13408000";
        let result = BidAsk::parse(src).unwrap();
        // let mut result_str = Vec::new();
        // result.serialize(&mut result_str);
        let date_time = result.date_time.unwrap_as_our_date();

        assert_eq!(result.id, "GBPUSD");
        assert_eq!(result.bid.to_string(), "1.13401");
        assert_eq!(result.ask.to_string(), "1.13408");
        assert_eq!("2022-09-21T12:33:48+00", &date_time.to_rfc3339()[..22]);

        // let result_str = String::from_utf8(result_str).unwrap();

        // assert_eq!(
        //     "GBPUSD 20220921123348100 1.13401000 1.13408000",
        //     result_str
        // );
    }
    #[test]
    fn test_our_with_zero_ms() {
        let src = "GBPUSD 20220921123348000 1.13401000 1.13408000";
        let result = BidAsk::parse(src).unwrap();
        // let mut result_str = Vec::new();
        // result.serialize(&mut result_str);
        let date_time = result.date_time.unwrap_as_our_date();

        assert_eq!(result.id, "GBPUSD");
        assert_eq!(result.bid.to_string(), "1.13401");
        assert_eq!(result.ask.to_string(), "1.13408");
        assert_eq!("2022-09-21T12:33:48", &date_time.to_rfc3339()[..19]);

        // let result_str = String::from_utf8(result_str).unwrap();

        // assert_eq!(
        //     "GBPUSD 20220921123348100 1.13401000 1.13408000",
        //     result_str
        // );
    }
}
