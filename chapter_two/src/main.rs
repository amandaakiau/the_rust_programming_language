/*/ 

A biblioteca io vem da biblioteca standard, conhecida como std 

A biblioteca std:io fornece vários recursos úteis, 
incluindo a capacidade de aceitar input do usuário.

*/

use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    /*/ cria uma variável mutável vinculada 
    a uma instância nova e vazia de uma String */
    let mut guess = String::new();

    // chama a função stdin() do módulo io que permitirá lidar com o input do usuário
    io::stdin()

        /*/ o método read_line é responsável por obter o input do usuário
        e recebe como argumento " &mut guess " para indicar em qual variável será armazenado o valor recebido
        o & indica que o argumento é uma referencia, oferecendo assim uma maneira de permitir que várias partes do seu código acessem um dado sem precisar copiar esse dado na memória várias vezes.

        **Uma das principais vantagens do Rust é a facilidade e segurança ao usar referências**
        
        assim como as variáveis, as referencias por padrão são imutáveis, por isso pra indicar que é mutável deve escrever &mut
        */
        .read_line(&mut guess)

        /*/ O metodo expect trata se houver erro no resultado do metodo read_line
        então, caso haja erro no resultado, será printado na tela a mensagem.
        Observa-se também que essas 3 linhas de códigos é 1 "ação lógica", 
        por exemplo, poderia ser escrito tudo junto: io::stdin().read_line(&mut guess).expect("Failed to read line");
        mas fica mais legível separado em linhas distintas e com identação */
        .expect("Failed to read line");

    // imprime o valor da variável guess    
    println!("You guessed: {guess}");
}