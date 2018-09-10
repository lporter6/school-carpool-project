#[derive(Debug)]
pub struct Parent {
    age: Option<u32>,
    fname: String,
    lname: String,
    mname: Option<String>,
}
impl Parent{
    pub fn get_name(&self) -> String{
        let blank = String::from("");
        let name = format!("{}{}{}", self.fname, match self.mname{
                Some(ref x) => &format!(" {} ", x),
                None => &blank,
            }, self.lname);
        name
    }
    pub fn new(age :Option<u32>, fname: String, mname: Option<String>, lname: String) -> Parent{
        Parent {
            age,
            fname,
            lname,
            mname,
        }
    }
}
