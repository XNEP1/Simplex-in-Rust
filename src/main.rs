use std::io;
use prettytable::{Table, Row, Cell};

fn print_table(eq: &Vec<Vec<f64>>){
    let mut table = Table::new();
    for row in eq {
        let mut cells: Vec<Cell> = Vec::new();
        for c in row{
            cells.push(Cell::new(&c.to_string()));
        }
        table.add_row(Row::new(cells));
    }
    table.printstd();
}

fn scan<F>(s: &str) -> F
    where
        F: core::str::FromStr,
{
    let mut buffer = String::new();

    loop{
        print!("{}", s);
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada.");
        match buffer.trim().parse::<F>() {
            Ok(n) => {
                return n;
            }
            Err(_) => {
                println!("Entrada Invalida, digite um inteiro positivo.");
                continue;
            }
        }
    }
}

fn scan_eq(eq: &mut Vec<Vec<f64>>, n_x: &usize, s: &str){
    let mut buffer = String::new();

    'outer: loop{
        print!("{s}");
        let mut v: Vec<f64> = Vec::new();
        v.clear();
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Erro ao ler entrada.");
        let coefs = buffer.split(' ');
        for coef in coefs {
            match coef.trim().parse::<f64>() {
                Ok(n) => {
                    v.push(n);
                }
                Err(_) => {
                    println!("Entrada Invalida, digite um número.");
                    continue 'outer;
                }
            }
        }

        if v.len() != (n_x + 2) {
            println!("Quantidade errada de coeficientes.\n Lembre-se de digitar 0 onde não tem valor para algum X.");
            continue 'outer;
        }

        eq.push(v);
        break 'outer;
    }
}

fn it_simplex(col_pivo: usize, row_pivo: usize, eq: &mut Vec<Vec<f64>>){
    let old_pivo_row = &eq[row_pivo];
    let mut new_pivo_row: Vec<f64> = Vec::with_capacity( old_pivo_row.len() );
    
    // Atualiza a linha pivo
    for coef in old_pivo_row{
        new_pivo_row.push( coef / old_pivo_row[col_pivo] );
    }
    eq[row_pivo].copy_from_slice(&new_pivo_row);

    // Atualiza todas as outras linhas, incluindo a da função objetivo.
    for (i, row) in eq.iter_mut().enumerate() {
        if i == row_pivo {
          continue;
        }

        let mut updated_line: Vec<f64> = Vec::with_capacity( row.len() );
        for (ii, coef) in new_pivo_row.iter().enumerate() {
            let updated_coef = (*coef * row[col_pivo] * -1.0) + row[ii];
            updated_line.push( updated_coef );

        }

        row.copy_from_slice(&updated_line);
    }



}

fn main() {

    let n_x: usize;
    let n_eq: usize;
    let n_basicos: usize;

    n_x = scan::<usize>("Quantos X's há no sistema?\n");
    n_basicos = scan::<usize>("Quantos desses X's são básicos?\n");
    n_eq = scan::<usize>("Quantas equações limitadoras há no sistema?\n");
    
    let mut eq: Vec<Vec<f64>> = Vec::with_capacity(n_eq + 1);
    // eq.push(vec![1.0, -12.0, -60.0, 0.0, 0.0, 0.0 , 0.0]);
    // eq.push(vec![0.0, 15.0, 30.0, 1.0, 0.0, 0.0, 2160.0]);
    // eq.push(vec![0.0, 6.0, 45.0, 0.0, 1.0, 0.0, 1320.0]);
    // eq.push(vec![0.0, 6.0, 24.0, 0.0, 0.0, 1.0, 900.0]);
    scan_eq(&mut eq, &n_x, "Coeficientes da Equação objetivo\nZ X1 X2 .. Xn B\n");

    let mut n = 1;
    while eq.len() <= n_eq {
        scan_eq(&mut eq, &n_x, &format!("Coeficientes da Equação limitadora {n}\nZ X1 X2 .. Xn B\n"));
        n += 1;
    }

    print_table(&eq);

    loop {
        let vb = &(eq[0])[1..=n_basicos];
        let mut min = std::f64::INFINITY;
        let mut min_index = 0;
        for (i, c) in vb.iter().enumerate(){

            if *c >= 0.0{
                continue;
            }

            if *c < min {
                min_index = i;
                min = *c;
            }
        }

        if min == std::f64::INFINITY { 
            println!("Solução encontrada.");
            break;
        }
        let col_pivo = min_index + 1; // indice no slice + 1 = indice no vetor original

        let eq_lim = &eq[1..=n_eq];
        min = std::f64::INFINITY;
        min_index = 0;
        for (i, row) in eq_lim.iter().enumerate(){
            let b = row.last().unwrap();
            let pivo = row[col_pivo];

            if pivo <= 0.0{
                continue;
            }

            let div_aux = b/pivo;
            if div_aux < min {
                min_index = i;
                min = div_aux;
            }
        }

        if min == std::f64::INFINITY { 
            println!("Não existe máximo, pode aumentar infinitamente.");
            break;
        }
        let row_pivo = min_index + 1; // indice no slice + 1 = indice no vetor original

        it_simplex(col_pivo, row_pivo, &mut eq);
        print_table(&eq);
    }

    print_table(&eq);

    
    // print!("Insira a função objetivo conforme o exemplo.\n Z = 5x1 -19x2 \n");
    // let mut col_pivo: i8 = 0;

    // let eq_z: [f64; 7] = [1.0, -12.0, -60.0, 0.0, 0.0, 0.0 , 0.0];
    // let eq_x3: [f64; 7] = [0.0, 15.0, 30.0, 1.0, 0.0, 0.0, 2160.0];
    // let eq_x4: [f64; 7] = [0.0, 6.0, 45.0, 0.0, 1.0, 0.0, 1320.0];
    // let eq_x5: [f64; 7] = [0.0, 6.0, 24.0, 0.0, 0.0, 1.0, 900.0];

    // // Escolha a coluna pivo
    // let mut input1: f64 = 0.0; 
    // let mut input2 = String::new(); 
    // if scanf!("{}: {}", input1, input2).is_ok() {
    //     print!("{input2} > {input1}\n");
    // }
    // else {
    //     print!("input errado\n");
    // };
}
