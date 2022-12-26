pub(crate) mod structs;

use structs::done::Done;
use structs::pending::Pending;
use structs::on_hold::OnHold;

pub enum ItemTypes {
    OnHold(OnHold),
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(item_type: &str, item_title: &str) -> Result<ItemTypes, &'static str> {
    match item_type {
        "pending" => {
            let pending_item = Pending::new(item_title);
            Ok(ItemTypes::Pending(pending_item))
        },
        "done" => {
            let done_item = Done::new(item_title);
            Ok(ItemTypes::Done(done_item))
        },
        "on_hold" => {
            let on_hold_item = OnHold::new(item_title);
            Ok(ItemTypes::OnHold(on_hold_item))
        },
        _ => {
            Err("Unknown item type")
        }
    }
}