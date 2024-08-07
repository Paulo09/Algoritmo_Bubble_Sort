



                                         Resolução em Rust, utilizando iteradores e Bubble Sort





Entendendo o Problema:

Leitura de arquivo: Abrir um arquivo e ler os números inteiros, linha por linha.
Armazenamento: Armazenar os números em um vetor (ou Vec em Rust).
Ordenação: Ordenar o vetor usando o algoritmo Bubble Sort.
Exibição: Iterar sobre o vetor ordenado e imprimir cada elemento.
Gravação: Gravar o vetor ordenado em um novo arquivo.
Código Rust:

Rust
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
Use o código com cuidado.

Explicação:

Importações: Importamos os módulos necessários para lidar com arquivos, buffers e operações de entrada/saída.
Função bubble_sort: Implementa o algoritmo Bubble Sort para ordenar o vetor de números.
Leitura do arquivo:
Abre o arquivo especificado.
Cria um buffer para leitura eficiente.
Lê cada linha do arquivo, converte para um número inteiro e adiciona ao vetor numbers.
Ordenação: Chama a função bubble_sort para ordenar o vetor.
Exibição: Utiliza um iterador para percorrer o vetor e imprimir cada número.
Gravação:
Cria um novo arquivo para escrita.
Utiliza um iterador para escrever cada número em uma nova linha do arquivo.
Pontos importantes:

Iteradores: O código utiliza iteradores para percorrer o vetor tanto para exibição quanto para gravação, proporcionando uma forma concisa e elegante de trabalhar com coleções.
Tratamento de erros: O código utiliza expect para tratar erros de I/O, garantindo que o programa termine de forma limpa em caso de falhas.
Eficiência: Embora o Bubble Sort seja um algoritmo simples, ele pode ser ineficiente para grandes conjuntos de dados. Para aplicações mais exigentes, algoritmos de ordenação mais eficientes como Merge Sort ou Quick Sort podem ser utilizados.
Flexibilidade: O código pode ser facilmente adaptado para ler e escrever diferentes tipos de dados e formatos de arquivo.
Observações:

Caminho do arquivo: Certifique-se de substituir "numbers.txt" pelo caminho correto do seu arquivo.
Formato do arquivo: Assumimos que o arquivo contém um número inteiro por linha. Para outros formatos, ajuste a leitura do arquivo.
Outros algoritmos de ordenação: Para aplicações mais exigentes, explore outros algoritmos de ordenação como Merge Sort, Quick Sort ou Heap Sort.
Este código fornece uma solução completa para a tarefa, utilizando iteradores e o algoritmo Bubble Sort para ordenar os números e gravar o resultado em um novo arquivo.
