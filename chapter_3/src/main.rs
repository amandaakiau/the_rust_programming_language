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

    // numeric Operations
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

    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character type
    // os tipo caracter deve ser representado com aspas simples, ao contrário das strings que sao representadas com aspas duplas
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound types: tuples and arrays

    // tuples: possuem tamanho fixo
    let tup = (500, 6.4, 1); // diferentes valores nas tuplas nao precisam ser do mesmo tipo
    let (x, y, z) = tup; // isso aqui é chamado de "destructuring" pq quebra uma  tupla em três variáveis separadas x, y e z
    let five_hundred = tup.0; // voce pode tbm especificar a posição do valor na tupla, neste caso estamos chamando o valor de posição 0 

    // arrays: em rust eles tbm possuem tamanho fixo  
    let a = [1, 2, 3, 4, 5]; // ao contrario das tuplas, os valores no array devem ser do mesmo tipo

    /* Arrays are useful when you want your data allocated on the stack rather than the heap 
    or when you want to ensure you always have a fixed number of elements. */

    let a2 = [3; 5]; // voce pode inicializar um array que contem o mesmo valor em cada elemento especificando o valor e o tamanho do array
    // ou seja, isso é o mesmo que a2 = [3, 3, 3, 3, 3]

    // acessando valores em um array:
    let first = a[0];
    println!("O primeiro valor do array é {first}");


    another_function(5);

    let y = five();
    println!("O valor de y é {y}");


}


fn another_function(x: i32) {
    println!("O valor the x é: {x}");
}


 // uma função que retorna 5:
 fn five() -> i32 {  // voce deve especificar o tipo do valor de retorno com -> 
    5
}

/*/ para declarar uma função basta colocar fn e o nome da função em seguida (conforme acima), e se tiver parâmetros vc deve especificar os tipos
Observa-se tbm que a função another_function foi declarada depois da função main, em rust nao importa se você declara antes ou depois da main*/ 

/* -------------------- Statements and Expressions -----------------------

Statements are instructions that perform some action and do not return a value.
Expressions evaluate to a resultant value. Let’s look at some examples.
 */

