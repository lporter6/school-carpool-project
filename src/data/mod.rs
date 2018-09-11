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
        let middle = match self.mname{
            Some(ref x) => &x,
            None => &blank,
        };
        let mut name = String::from("");
        if middle == &blank {
            name = format!("{} {}",self.fname,self.lname);
        }else{
            name = format!("{} {} {}",self.fname, middle, self.lname);
        }
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

struct Student {
    fname: String,
    mname: Option<String>,
    lname: String,
    grade: Grade,
    scores: Scores,
}

enum Grade {
    Kindergarden,
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelveth,
}

struct Scores{
    years: Vec<Table>,
}
struct Table {
    grade: Grade,
    scores: Vec<ClassScore>,
}
struct ClassScore{
    class: String,
    categories: Vec<Category>,
}
struct Category{
    name: String,
    grades: Vec<u8>,
    weight: u8,
}
impl Scores{
    pub fn new(years: Vec<Table>) -> Scores{
        Scores{
            years,
        }
    }
}
impl Table {
    pub fn new(grade: Grade, scores: Vec<ClassScore>) -> Table{
        Table {
            grade,
            scores,
        }
    }
}
impl ClassScore{
    pub fn new(class: String, categories: Vec<Category>) -> ClassScore{
        ClassScore {
            class,
            categories,
        }
    }
}
impl Category {
    pub fn new(name: String, grades: Vec<u8>, weight: u8) -> Category{
        Category {
            name,
            grades,
            weight,
        }
    }
}
