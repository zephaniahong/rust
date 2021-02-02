#![warn(clippy::manual_flatten)]

fn main() {
    // Test for loop over implicitly adjusted `Iterator` with `if let` expression
    let x = vec![Some(1), Some(2), Some(3)];
    for n in x {
        if let Some(n) = n {
            println!("{}", n);
        }
    }

    // Test for loop over implicitly implicitly adjusted `Iterator` with `if let` statement
    let y: Vec<Result<i32, i32>> = vec![];
    for n in y.clone() {
        if let Ok(n) = n {
            println!("{}", n);
        };
    }

    // Test for loop over by reference
    for n in &y {
        if let Ok(n) = n {
            println!("{}", n);
        }
    }

    // Test for loop over `Iterator` with `if let` expression
    let z = vec![Some(1), Some(2), Some(3)];
    let z = z.iter();
    for n in z {
        if let Some(n) = n {
            println!("{}", n);
        }
    }

    // Using the `None` variant should not trigger the lint
    let z = vec![Some(1), Some(2), Some(3)];
    for n in z {
        if n.is_none() {
            println!("Nada.");
        }
    }

    // Using the `Err` variant should not trigger the lint
    for n in y.clone() {
        if let Err(e) = n {
            println!("Oops: {}!", e);
        }
    }

    // Having an else clause should not trigger the lint
    for n in y.clone() {
        if let Ok(n) = n {
            println!("{}", n);
        } else {
            println!("Oops!");
        }
    }

    // Using manual flatten should not trigger the lint
    for n in vec![Some(1), Some(2), Some(3)].iter().flatten() {
        println!("{}", n);
    }
}
