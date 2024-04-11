fn main() {
    /*/ por padrão as variáveis em rust são imutáveis
    para declarar uma variável mutável vc deve explicitar com let mut */
    let mut x = 5;
    let y = 10;
    println!("The value of x is {x} and y is {y}");
    x = 6;
    println!("Now the value of x is {x} and y is {y}");

    // as constantes serão sempre imutáveis e você deve especificar o tipo (no caso abaixo o tipo é u32)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of const is {THREE_HOURS_IN_SECONDS}");

    /*/ shadowing é a declaração de uma nova variável com o mesmo nome */
    let z = 5;
    let z = z + 1;

    // inner shawoing - declarão de nova variável com o mesmo nome dentro de um escopo específico
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z out is {z}");

    /*/ shadowing vs mutable variable 
    shadowing é diferente de fazer uma variável ser mutável
    pois no caso do shadowing vc deve declarar a variável novamente com o let, vc não pode simplesmente atribuir um novo valor como z = 10 por exemplo
    usando o let vc pode fazer algumas transformações na variável (como o seu tipo) e garantir que ela é imutável após essas transformações*/
    
    // supondo que voce crie um programa que pergunta ao usuario quantos espaços ele gostaria de incluir entre um texto
    // e depois vc precisa armazenar esse input como um numero
    // vc poderia fazer assim:

    let spaces = "    ";
    let spaces = spaces.len();
    println!("A quantidade de espaços é {spaces}.");

    // float and integer types 
    let a = -2; // i32 is default (numeros positivos e negativos)
    let b: u64 = 27; // explicit u64 (apenas numeros positivos - unsigned)
    let w = 2.5; // f64 is default
    let k: f32 = 3.5; // explicit f32

    println!("Integer i32 type is {a} and u64 type is {b} ");
    println!("Float f64 type is {w} and f32 type is {k} ");

    // operações matematicas
    let sum = 10 + 1;
    let difference = 10 - 1;
    let multiplication = 10 * 1;
    let quotient = 10/2;
    let truncated = -5/3; 
    let remainder = 43 % 5; 

    println!("Resultados: 
            soma: {sum}, 
            subtração: {difference},
            multiplicação: {multiplication},
            divisão: {quotient}, 
            divisão com resultado inteiro: {truncated},
            resto da divisão: {remainder}");

}