use std::io;

fn sumar(x: f32, y: f32) -> f32 {
    x + y
}

fn restar(x: f32, y: f32) -> f32 {
    x - y
}

fn multiplicar(x: f32, y: f32) -> f32 {
    x * y
}

fn dividir(x: f32, y: f32) -> Option<f32> {
    if y != 0.0 {
        Some(x / y)
    } else {
        None
    }
}

fn main() {
    loop {
        println!("Operaciones disponibles:");
        println!("1. Suma");
        println!("2. Resta");
        println!("3. Multiplicación");
        println!("4. División");
        println!("5. Salir");
        
        println!("Selecciona una operación (1/2/3/4/5):");
        let mut seleccion = String::new();
        io::stdin().read_line(&mut seleccion).expect("Fallo al leer la línea");
        
        if seleccion.trim() == "5" {
            println!("Sesion finalizada..");
            break;
        }
        
        println!("Ingresa el primer número:");
        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).expect("Fallo al leer la línea");
        let num1: f32 = num1.trim().parse().expect("Por favor, ingresa un número");
        
        println!("Ingresa el segundo número:");
        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).expect("Fallo al leer la línea");
        let num2: f32 = num2.trim().parse().expect("Por favor, ingresa un número");
        
        match seleccion.trim() {
            "1" => println!("Resultado: {}", sumar(num1, num2)),
            "2" => println!("Resultado: {}", restar(num1, num2)),
            "3" => println!("Resultado: {}", multiplicar(num1, num2)),
            "4" => match dividir(num1, num2) {
                Some(resultado) => println!("Resultado: {}", resultado),
                None => println!("Error: División por cero."),
            },
            _ => println!("Opción inválida. Por favor, intenta de nuevo."),
        }
    }
}
