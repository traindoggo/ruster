// module
//
// クレート内のコードをグループ化するもの
mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // 当然非公開
    // front_of_house::hosting::seat_at_table();
}
