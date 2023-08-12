use {crate::*, gluesql_core::prelude::Value::*};

test_case!(stdev, async move {
    let g = get_tester!();

    g.run(
        "
    CREATE TABLE Item (
        id INTEGER,
        quantity INTEGER,
        age INTEGER NULL,
        total INTEGER,
    );
    ",
    )
    .await
    .unwrap();
    g.run(
        "
    INSERT INTO Item (id, quantity, age, total) VALUES
        (1, 10,   11, 1),
        (2,  0,   90, 2),
        (3,  9, NULL, 3),
        (4,  3,    3, 1),
        (5, 25, NULL, 1);
    ",
    )
    .await
    .unwrap();

    let test_cases = [
        (
            "SELECT STDEV(age) FROM Item",
            select_with_null!("STDEV(age)"; Null),
        ),
        (
            "SELECT STDEV(total) FROM Item",
            select!(
                "STDEV(total)"
                F64;
                0.8
            ),
        ),
    ];

    for (sql, expected) in test_cases {
        g.test(sql, Ok(expected)).await;
    }
});
