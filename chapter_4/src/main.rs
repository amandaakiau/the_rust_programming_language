fn main() {

    //  -------- VARIABLE SCOPE ----------


    {                      // s is not valid here, it’s not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid




    // --------- String type ---------------
    // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. 
    // Esse tipo é ideal quando não sabemos o valor exato que vai ser armazenado, por exemplo, quando solicitamos input do usuário
    let _s = String::from("hello");

    // This kind of string can be mutated
    let mut _s = String::from("hello");

    _s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", _s); 

    // quando você diz que s2 = s1, s1 automaticamente para de existir, isso é uma das estratégias de rust para evitar o problema nomeado double free error, 
    // que é o caso de de duas variaveis tentarem liberar o mesmo espaço na memoria.
    // "Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.""

    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1);   isso levaria a um erro, pois s1 nao existe mais


    // Se você precisar realmente criar uma copia do conteúdo na memória, você pode utilizar o método clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Com inteiros você nao precisa utilizar o clone, isso porque o tipo inteiro possui um tamanho conhecido no momento da compilação (ao contrario do tipo String)
    // e por conta disso o tipo inteiro é armazenado inteiramente na stack, entao copia dos valores é algo facil de ser realizado
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);


    // Ownership and Functions

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);



} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.


  fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}