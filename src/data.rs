
use std::mem::swap;

// Holds one row of Numbers or None if the specific field in the vec is empty
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Row {
    pub row: Vec<Option<usize>>,
    pub length: usize,
}

// Holds the whole field 
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Field {
    pub rows: Vec<Row>,
    pub size: usize,
}

// Implementation of
// - push_up DONE
// - push_up_single DONE
// - push_down DONE
// - push_down_single DONE
// - swipe_up DONE
// - swipe_up_single DONE
// - swipe_down DONE
// - swipe_down_single TODO
// - swipe_left DONE
// - swipe_right DONE
// - insert_random DONE
// - print DONE
impl Field {
    pub fn print(&self) {
        for r in &self.rows{
            r.print();
        }
    }
    
    pub fn new(size: usize) -> Field {
        let mut field = Vec::with_capacity(size);
        for _ in 0 .. size {
            field.push(Row::new(size));
        }
        Field {
            rows: field,
            size: size,
        }

    }

    pub fn insertable(&self) -> bool {
        for r in &self.rows {
            if r.insertable() {
                return true;
            }
        }
        return false;
    }
    
    pub fn insert_random(&mut self) -> bool {
        if self.insertable() {
            loop {
                let rand: usize = rand::random();
                let row_number = rand % self.size;
                if self.rows[row_number].insertable() {
                    self.rows[row_number].insert_random();
                    break;
                }       
            }
            return true;
        }
        return false;
    }

    pub fn swipe_left(&mut self) {
        for row in &mut self.rows {
            row.swipe_left();
        }
        
    }

    pub fn swipe_right(&mut self) {
        for row in &mut self.rows {
            row.swipe_right();
        }
        
    }

    fn swipe_up_single(&mut self, i: usize) {
        self.push_up_single(i);
        for j in 0 .. self.size - 1 {
            if self.rows[j].row[i] == self.rows[j+1].row[i] {
                match self.rows[j].row[i] {
                    Some(a) => {
                        self.rows[j].row[i] = Some(2 * a);
                        self.rows[j+1].row[i] = None;
                    },
                    None => {},
                }
                
            }
        }
        self.push_up_single(i);
        
    }

    pub fn swipe_up(&mut self) {
        for i in 0..self.size {
            self.swipe_up_single(i);
        }
    }

    fn swipe_down_single(&mut self, i: usize) {
        self.push_down_single(i);
        for j in 0 .. self.size - 1 {
            let index = self.size - 1 - j;
            if self.rows[index].row[i] == self.rows[index - 1].row[i] {
                match self.rows[index].row[i] {
                    Some(a) => {
                        self.rows[index].row[i] = Some(2 * a);
                        self.rows[index - 1].row[i] = None;
                    },
                    None => {},
                }
                
            }
        }
        self.push_down_single(i);
        
    }

    pub fn swipe_down(&mut self) {
        for i in 0..self.size {
            self.swipe_down_single(i);
        }
    }
    
    fn push_up_single(&mut self, i: usize) {
        let mut collected = Vec::with_capacity(self.size);
        for j in 0 .. self.size {
            let mut tmp = None;
            swap(&mut tmp, &mut self.rows[j].row[i]);
            if tmp != None {
                collected.push(tmp);
            } 
        }
        for j in 0 .. collected.len() {
            self.rows[j].row[i] = collected[j];
        }
    }

    pub fn push_up(&mut self){
        for i in 0..self.size {
            self.push_up_single(i);
        }
    }

    fn push_down_single(&mut self, i: usize) {
        let mut collected = Vec::with_capacity(self.size);
        for j in 0 .. self.size {
            let index = self.size - 1 - j;
            let mut tmp = None;
            swap(&mut tmp, &mut self.rows[index].row[i]);
            if tmp != None {
                collected.push(tmp);
            } 
        }
        for j in 0 .. collected.len() {
            let index = self.size - 1 - j;
            self.rows[index].row[i] = collected[j];
        }
    }

    pub fn push_down(&mut self){
        for i in 0..self.size {
            self.push_down_single(i);
        }
    }

}

// Implementation of
// - new DONE
// - insertable DONE
// - insert_random DONE
// - push_left DONE
// - push_right DONE
// - swipe_right DONE
// - swipe_left DONE
// - print DONE
impl Row {

    pub fn print(&self) {
        println!("{:?}", self.row);
    }
    
    pub fn new(length: usize) -> Row {
        let mut row = Vec::with_capacity(length);
        for _ in 0..length {
            row.push(None);
        }
        Row {
            row: row,
            length: length,
        }
    }

    pub fn insertable(&self) -> bool {
        for i in &self.row {
            if let None = i {
                return true;
            }
        }
        return false;
    }

    pub fn insert_random(&mut self) -> bool {
        if self.insertable() {
            loop {
                let random: usize = rand::random();
                let index: usize = random % self.length;
                let value;
                if rand::random() {
                    value = 2;
                } else {
                    value = 1;
                }
                match self.row[index] {
                    None => {
                        self.row[index] = Some(value);
                        break;
                    },
                    Some(_) => {
                        continue;
                    }
                }
            }
            
            return true;
        }
        return false;
    }

    pub fn push_right(&mut self) {
        let mut vec = Vec::new();
        let mut v_2   = Vec::new();
        
        swap(&mut vec, &mut self.row);

        for v in vec {
            if let Some(a) = v {
                v_2.push(Some(a));
            } else {
                self.row.push(None);
            }
        }
        
        for v in v_2 {
            self.row.push(v)
        }
    }

    pub fn push_left(&mut self) {
        let mut vec = Vec::new();
        let mut counter = 0;
        
        swap(&mut vec, &mut self.row);
        
        for v in vec {
            if let Some(a) = v {
                self.row.push(Some(a))
            }else {
                counter = counter + 1;
            }
        }

        for _ in 0..counter{
            self.row.push(None);
        }
        
    }
    

    pub fn swipe_right(&mut self) {
        self.push_right();
        
        for i in 0 .. self.length {
            let index = self.length - 1 - i;
            if index >= 1
                && self.row[index] == self.row[index - 1] {
                    match self.row[index] {
                        Some(a) => {
                            self.row[index] = Some(a * 2);
                            self.row[index - 1] = None;
                        },
                        None => {},
                    }
                }       
        }

        self.push_right();
    }

    pub fn swipe_left(&mut self) {
        self.push_left();
        
        for i in 0 .. self.length {
            let index = i;
            if index <= self.length - 2
                && self.row[index] == self.row[index + 1] {
                    match self.row[index] {
                        Some(a) => {
                            self.row[index] = Some(a * 2);
                            self.row[index + 1] = None;
                        },
                        None => {},
                    }
                }       
        }

        self.push_left();
    }
    
}


