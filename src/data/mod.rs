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
    K,
    1,
    2,
    3,
    4,
    5,
    6,
    7,
    8,
    9,
    10,
    11,
    12,
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
    years: Vec<Table>,
}
impl Table {
    pub fn new(grade: Grade, scores: Vec<ClassScore>){
        Table {
            grade,
            scores,
        }
    }
}
impl ClassScore{
    pub fn new(class: String, categories: Vec<Category>){
        ClassScore {
            class,
            categories,
        }
    }
}
impl Category {
    pub fn new(name: String, grades: Vec<u8>, weight: u8){
        Category {
            name,
            grades,
            weight,
        }
    }
}
