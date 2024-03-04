def sumar(x, y):
    """Suma x y y."""
    return x + y

def restar(x, y):
    """Resta y de x."""
    return x - y

def multiplicar(x, y):
    """Multiplica x por y."""
    return x * y

def dividir(x, y):
    """Divide x entre y. Asegura que y no sea cero."""
    if y == 0:
        return "Error: División por cero."
    else:
        return x / y

def main():
    while True:
        print("Operaciones disponibles:")
        print("1. Suma")
        print("2. Resta")
        print("3. Multiplicación")
        print("4. División")
        print("5. Salir")
        seleccion = input("Selecciona una operación (1/2/3/4/5): ")
        
        if seleccion == '5':
            print("Sesion finalizada...")
            break
        
        num1 = float(input("Ingresa el primer número: "))
        num2 = float(input("Ingresa el segundo número: "))
        
        if seleccion == '1':
            print("Resultado:", sumar(num1, num2))
        elif seleccion == '2':
            print("Resultado:", restar(num1, num2))
        elif seleccion == '3':
            print("Resultado:", multiplicar(num1, num2))
        elif seleccion == '4':
            print("Resultado:", dividir(num1, num2))
        else:
            print("Opción inválida. Por favor, intenta de nuevo.")

if __name__ == "__main__":
    main()
