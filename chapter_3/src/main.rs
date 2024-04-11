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
    println!("A quantidade de espaços é {spaces}")
}