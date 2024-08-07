use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs::OpenOptions;

fn bubble_sort(numbers: &mut Vec<i32>) {
    let n = numbers.len();
    for _ in 0..n {
        for j in 0..n - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let file_path = "numbers.txt"; // Substitua pelo caminho do seu arquivo

    // Ler os números do arquivo
    let file = File::open(file_path).expect("Não foi possível abrir o arquivo");
    let reader = BufReader::new(file);

    let mut numbers: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let number: i32 = line.parse().unwrap();
        numbers.push(number);
    }

    // Ordenar os números usando Bubble Sort
    bubble_sort(&mut numbers);

    // Exibir os números ordenados
    println!("Números ordenados:");
    for number in &numbers {
        println!("{}", number);
    }

    // Gravar os números ordenados em um novo arquivo
    let output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("sorted_numbers.txt")
        .expect("Não foi possível criar o arquivo de saída");

    for number in &numbers {
        writeln!(output_file, "{}", number).expect("Erro ao escrever no arquivo");
    }
}
