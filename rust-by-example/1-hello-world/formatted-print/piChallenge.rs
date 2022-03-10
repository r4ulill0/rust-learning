/* Programa que permita variar los
 * decimales del número pi que se van a imprimir
 *
 * Ej. Precision 3, resultado 3.141
 */

fn main() {
    let pi = 3.141592;
    let max_precision = 3;
    println!("{pi:.precision$}", pi=pi, precision=max_precision);
}
