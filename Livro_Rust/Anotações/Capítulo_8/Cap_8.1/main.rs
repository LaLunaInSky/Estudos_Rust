#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut vec_01: Vec<i32> = Vec::new();

    let vec_02 = vec![1, 2, 3, 4, 5];

    for número in 5..9 {
        vec_01.push(número);
    }

    println!(
        "vec_01 contém: {vec_01:?}\nvec_02 contém: {vec_02:?}"
    );

    //

    let terceira_posição_da_vec_02_por_indexação: &i32 = &vec_02[2];

    println!(
        "\nvec_02 valor da terceira posição por indexação: {terceira_posição_da_vec_02_por_indexação}"
    );

    let terceira_posição_da_vec_02_por_get_método: Option<&i32> = vec_02.get(2);

    match terceira_posição_da_vec_02_por_get_método {
        Some(terceira_posição_da_vec_02_por_get_método) => println!(
            "vec_02 valor da terceira posição por get método: {terceira_posição_da_vec_02_por_get_método}"
        ),
        None => println!(
            "Não foi encontrado o elemento da terceira posição de vec_02"
        ),
    }

    //

    // let _posição_não_exite_indexação = &vec_02[100];
    
    // println!(
    //     "\nvec_02 na posição 101 com indexação existe? {_posição_não_exite_indexação:?}"
    // );

    let _posição_não_exite_get_método = vec_02.get(100);

    println!(
        "\nvec_02 na posição 101 com get método existe? {_posição_não_exite_get_método:?}"
    );

    //
    let primeira_posição_do_vec_01 = &vec_01[0];

    // vec_01.push(6);

    println!(
        "\nvec_01 a primeira posição é {primeira_posição_do_vec_01}"
    );

    vec_01.push(6);

    println!(
        "vec_01 contém: {vec_01:?}"
    );

    //
    println!();
    
    let vec_03 = vec![100, 32, 57];
    
    for item in &vec_03 {
        print!(
            "{item} "
        );
    }

    println!();

    let mut vec_04 = vec![100, 32, 57];
    
    for item in &mut vec_04 {
        *item += 50;
    }

    for item in &vec_04 {
        print!(
            "{item} "
        );
    }

    println!();

    //
    let row_01 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!(
        "\nrow_01 contém os valores: {row_01:?}"
    );
}