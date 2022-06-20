use std::collections::HashMap;

#[derive(Debug)]
pub struct School<T> {
    database: HashMap<String, T>,
}

impl<T> School<T>
where
    T: Clone + Ord,
{
    pub fn new() -> Self {
        Self {
            database: HashMap::new(),
        }
    }

    pub fn add(&mut self, student: &str, grade: T) {
        self.database.insert(String::from(student), grade);
    }

    pub fn grades(&self) -> Vec<T> {
        let mut _grades: Vec<T> = self.database.values().cloned().collect();
        _grades.sort();
        _grades.dedup();
        _grades
    }

    pub fn grade(&self, grade: T) -> Vec<String> {
        let _students = &self.database;
        let mut result = Vec::<String>::new();

        for (_student, _grade) in _students {
            if *_grade == grade {
                result.push(_student.to_string())
            }
        }
        result.sort();
        result
    }

    /// Just for testing
    ///
    /// Please ignore this function
    pub fn with_template(template: HashMap<String, T>) -> Self {
        Self { database: template }
    }
}