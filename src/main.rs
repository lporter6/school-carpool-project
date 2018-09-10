extern crate school_carpool_project;

use school_carpool_project::data::Parent;
fn main() {
    let parent = Parent::new(Some(32),String::from("William"), None, String::from("Porter"));
    println!("{:#?}", parent);
    println!("{}", parent.get_name());
}
