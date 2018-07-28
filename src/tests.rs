#[cfg(test)]

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

#[test]
fn swipe_up() {
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
        row: vec![Some(4), Some(4), Some(4), Some(4)],
        length: 4,
    };

    let mut row_2_2 = Row {
        row: vec![None, None, None, None],
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

    field_1.swipe_up();
    assert_eq!(field_1, field_2);
}        

#[test]
fn swipe_down() {
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
        row: vec![Some(4), Some(4), Some(4), Some(4)],
        length: 4,
    };

    let mut row_2_2 = Row {
        row: vec![None, None, None, None],
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
                   row_2_2,
                   row_2_1],
        size: 4,
    };

    field_1.swipe_down();
    assert_eq!(field_1, field_2);
}

#[test]
fn insert(){
    use data::Field;
    
    let mut field = Field::new(4);
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
    field.print();
    field.insert_random();
}





