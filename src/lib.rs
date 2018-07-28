#![feature(extern_prelude)]
extern crate rand;

#[cfg(test)]
mod tests {

    #[test]
    fn insertable() {
        use data::Row;
        let mut row_1 = Row {
            row: vec![None, Some(1), None, Some(2)],
            length: 4,
        };
        assert_eq!(row_1.insertable(), true);

        let mut row_1 = Row {
            row: vec![Some(1), Some(1), Some(1), Some(2)],
            length: 4,
        };
        assert_eq!(row_1.insertable(), false);

    }
        
    
    #[test]
    fn push_right() {
        use data::Row;
        
        let mut row_1 = Row {
            row: vec![None, Some(1), None, Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, None, Some(1), Some(2)],
            length: 4,
        };
        row_1.push_right();
        assert_eq!(row_1, row_2);
    }

    #[test]
    fn swipe_right() {
        use data::Row;
        
        let mut row_1 = Row {
            row: vec![None, Some(2), None, Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, None, None, Some(4)],
            length: 4,
        };
        row_1.swipe_right();
        assert_eq!(row_1, row_2);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, None, Some(4), Some(4)],
            length: 4,
        };
        row_1.swipe_right();
        assert_eq!(row_1, row_2);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(1)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, Some(2), Some(4), Some(1)],
            length: 4,
        };
        row_1.swipe_right();
        assert_eq!(row_1, row_2);

    }

    
    #[test]
    fn push_left() {
        use data::Row;
        
        let mut row_1 = Row {
            row: vec![None, Some(1), None, Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(1), Some(2), None, None],
            length: 4,
        };
        row_1.push_left();
        assert_eq!(row_1, row_2);
    }


    #[test]
    fn swipe_left() {
        use data::Row;
        
        let mut row_1 = Row {
            row: vec![None, Some(2), None, Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(4), None, None, None],
            length: 4,
        };
        row_1.swipe_left();
        assert_eq!(row_1, row_2);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(4), Some(4), None, None],
            length: 4,
        };
        row_1.swipe_left();
        assert_eq!(row_1, row_2);

        let mut row_1 = Row {
            row: vec![Some(1), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(1), Some(4), Some(2), None],
            length: 4,
        };
        row_1.swipe_left();
        assert_eq!(row_1, row_2);

    }

    #[test]
    fn push_up() {
        use data::Row;
        use data::Field;

        let mut row_1_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_1_2 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_1_3 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_1_4 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_2_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_2_2 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_2_3 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_2_4 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };
    

        let mut field_1 = Field {
            rows: vec![row_1_1,
                       row_1_2,
                       row_1_3,
                       row_1_4],
            size: 4,
        };
        let mut field_2 = Field {
            rows: vec![row_2_1,
                       row_2_2,
                       row_2_3,
                       row_2_4],
            size: 4,
        };

        field_1.push_up();
        assert_eq!(field_1, field_2);
    }        

    #[test]
    fn push_down() {
        use data::Row;
        use data::Field;

        let mut row_1_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_1_2 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_1_3 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_1_4 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_2_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_2_2 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };

        let mut row_2_3 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };

        let mut row_2_4 = Row {
            row: vec![None, None, None, None],
            length: 4,
        };
    

        let mut field_1 = Field {
            rows: vec![row_1_1,
                       row_1_2,
                       row_1_3,
                       row_1_4],
            size: 4,
        };
        let mut field_2 = Field {
            rows: vec![row_2_3,
                       row_2_4,
                       row_2_1,
                       row_2_2],
            size: 4,
        };

        field_1.push_down();
        assert_eq!(field_1, field_2);
    }        


    

}

mod data {
    use std::mem::swap;

    #[derive(Debug, PartialEq)]
    pub struct Row {
        pub row: Vec<Option<usize>>,
        pub length: usize,
    }

    #[derive(Debug, PartialEq)]
    pub struct Field {
        pub rows: Vec<Row>,
        pub size: usize,
    }

    impl Field {
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

        pub fn push_up_single(&mut self, i: usize) {
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

        pub fn push_down_single(&mut self, i: usize) {
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

    impl Row {
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

}
