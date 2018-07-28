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
        assert_eq!(row_1.row, row_2.row);
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
        assert_eq!(row_1.row, row_2.row);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, None, Some(4), Some(4)],
            length: 4,
        };
        row_1.swipe_right();
        assert_eq!(row_1.row, row_2.row);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(1)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![None, Some(2), Some(4), Some(1)],
            length: 4,
        };
        row_1.swipe_right();
        assert_eq!(row_1.row, row_2.row);

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
        assert_eq!(row_1.row, row_2.row);
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
        assert_eq!(row_1.row, row_2.row);

        let mut row_1 = Row {
            row: vec![Some(2), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(4), Some(4), None, None],
            length: 4,
        };
        row_1.swipe_left();
        assert_eq!(row_1.row, row_2.row);

        let mut row_1 = Row {
            row: vec![Some(1), Some(2), Some(2), Some(2)],
            length: 4,
        };
        let row_2 = Row {
            row: vec![Some(1), Some(4), Some(2), None],
            length: 4,
        };
        row_1.swipe_left();
        assert_eq!(row_1.row, row_2.row);

    }

}

mod data {
    use std::mem::swap;

    #[derive(Debug)]
    pub struct Row {
        pub row: Vec<Option<usize>>,
        pub length: usize,
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
