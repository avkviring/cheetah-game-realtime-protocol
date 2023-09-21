use crate::disconnect::command::DisconnectHeader;
use crate::others::member_id::MemberAndRoomId;
use crate::others::rtt::RoundTripTimeHeader;
use crate::reliable::ack::header::AckHeader;
use crate::reliable::retransmit::header::RetransmitHeader;

pub type HeaderVec<T> = heapless::Vec<T, 10>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Headers {
    pub(crate) headers: HeaderVec<Header>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Header {
    MemberAndRoomId(MemberAndRoomId),

    Ack(AckHeader),

    Disconnect(DisconnectHeader),

    RoundTripTimeRequest(RoundTripTimeHeader),

    RoundTripTimeResponse(RoundTripTimeHeader),

    Retransmit(RetransmitHeader),

    Hello,
}

impl Headers {
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.headers.capacity() == self.headers.len()
    }

    pub fn add(&mut self, header: Header) {
        assert!(self.headers.push(header).is_ok(), "Headers vector overflow {:?}", self.headers);
    }

    pub fn add_if_not_present(&mut self, header: Header) {
        if self.headers.contains(&header) {
            return;
        }
        assert!(self.headers.push(header).is_ok(), "Headers vector overflow {:?}", self.headers);
    }

    pub fn find<T, F: FnMut(&Header) -> Option<&T>>(&self, p: F) -> HeaderVec<&T> {
        self.headers.iter().filter_map(p).collect()
    }

    pub fn first<T, F: FnMut(&Header) -> Option<&T>>(&self, p: F) -> Option<&T> {
        self.headers.iter().find_map(p)
    }
}
