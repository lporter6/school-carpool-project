extern crate school_carpool_project;

use school_carpool_project::data::Parent;
fn main() {
    let parent = Parent::new(Some(32),String::from("William"), Some(String::from("Logan")), String::from("Porter"));
    println!("{:#?}", parent);
}
