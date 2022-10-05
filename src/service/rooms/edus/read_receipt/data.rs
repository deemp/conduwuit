use ruma::{RoomId, events::receipt::ReceiptEvent, UserId, serde::Raw};
use crate::Result;

pub trait Data: Send + Sync {
    /// Replaces the previous read receipt.
    fn readreceipt_update(
        &self,
        user_id: &UserId,
        room_id: &RoomId,
        event: ReceiptEvent,
    ) -> Result<()>;

    /// Returns an iterator over the most recent read_receipts in a room that happened after the event with id `since`.
    fn readreceipts_since(
        &self,
        room_id: &RoomId,
        since: u64,
    ) -> Box<dyn Iterator<
        Item = Result<(
            Box<UserId>,
            u64,
            Raw<ruma::events::AnySyncEphemeralRoomEvent>,
        )>,
    >>;

    /// Sets a private read marker at `count`.
    fn private_read_set(&self, room_id: &RoomId, user_id: &UserId, count: u64) -> Result<()>;

    /// Returns the private read marker.
    fn private_read_get(&self, room_id: &RoomId, user_id: &UserId) -> Result<Option<u64>>;

    /// Returns the count of the last typing update in this room.
    fn last_privateread_update(&self, user_id: &UserId, room_id: &RoomId) -> Result<u64>;
}
