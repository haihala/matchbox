use ggrs::{Message, PlayerType};

use crate::WebRtcSocket;

impl WebRtcSocket {
    /// Returns a Vec of connected peers as [`ggrs::PlayerType`]
    #[must_use]
    pub fn players(&self) -> Vec<PlayerType<String>> {
        // needs to be consistent order across all peers
        self.connected_peers()
            .iter()
            .map(|id| PlayerType::Remote(id.clone()))
            .chain([PlayerType::Local].into_iter())
            .collect()
    }
}

impl ggrs::NonBlockingSocket<String> for WebRtcSocket {
    fn send_to(&mut self, msg: &Message, addr: &String) {
        let buf = bincode::serialize(&msg).unwrap();
        let packet = buf.into_boxed_slice();
        self.send(packet, addr);
    }

    fn receive_all_messages(&mut self) -> Vec<(String, Message)> {
        let mut messages = vec![];
        for (id, packet) in self.receive().into_iter() {
            let msg = bincode::deserialize(&packet).unwrap();
            messages.push((id, msg));
        }
        messages
    }
}
