#include <stdio.h>

float sumar(float x, float y) {
    return x + y;
}

float restar(float x, float y) {
    return x - y;
}

float multiplicar(float x, float y) {
    return x * y;
}

float dividir(float x, float y) {
    if(y != 0.0) {
        return x / y;
    } else {
        printf("Error: División por cero.\n");
        return 0.0;
    }
}

int main() {
    int seleccion;
    float num1, num2, resultado;

    do {
        printf("Operaciones disponibles:\n");
        printf("1. Suma\n");
        printf("2. Resta\n");
        printf("3. Multiplicación\n");
        printf("4. División\n");
        printf("5. Salir\n");
        printf("Selecciona una operación (1/2/3/4/5): ");
        scanf("%d", &seleccion);
        
        if(seleccion == 5) break;

        printf("Ingresa el primer número: ");
        scanf("%f", &num1);
        printf("Ingresa el segundo número: ");
        scanf("%f", &num2);

        switch(seleccion) {
            case 1:
                resultado = sumar(num1, num2);
                printf("Resultado: %.2f\n", resultado);
                break;
            case 2:
                resultado = restar(num1, num2);
                printf("Resultado: %.2f\n", resultado);
                break;
            case 3:
                resultado = multiplicar(num1, num2);
                printf("Resultado: %.2f\n", resultado);
                break;
            case 4:
                resultado = dividir(num1, num2);
                if(num2 != 0.0) {
                    printf("Resultado: %.2f\n", resultado);
                }
                break;
            default:
                printf("Opción inválida. Por favor, intenta de nuevo.\n");
        }
    } while(seleccion != 5);

    printf("Sesion finalizada...\n");
    return 0;
}
