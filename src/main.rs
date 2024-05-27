use std::{process::exit, usize};

#[derive(Debug, Clone)]
struct Vector {
    header: String,
    data: Vec<f64>,
}
#[derive(Debug, Clone)]
struct Centro {
    headers: Vec<String>,
    valores: Vec<f64>,
}

struct Solución {
    centros: usize,
    distancias: f64,   
    iteraciones: usize,
    vec_pertenencia_datos_centros: Vec<Vec<u8>>,
    coor_centros: Vec<Centro>,
}

const FILENAME: &str = "km_lab.csv";
const PATH: &str = "src/";
const SOL_PATH: &str = "src/sol/";
const VERBOSE: bool = true;
const MEDOIDES: bool = false;

fn main() {
    //LEEMOS EL ARCHIVO
    let mut headers:Vec<String> = Vec::new();
    let mut data:Vec<Vector> = Vec::new();
    read_csv(&mut data, &mut headers, format!("{}{}", PATH, FILENAME).as_str());

    // println!("Headers: {:?}", headers); //TODELETE
    // println!("Data: {:?}", data); //TODELETE

    let (minimos, maximos) = minimos_maximos(&data);
    
    println!("-----------------------Minimos y maximos:---------------------------"); //TODELETE
    for (i, v) in data.iter().enumerate() {
        println!("{}: Min: {}, Max: {}", v.header, minimos[i], maximos[i]); //TODELETE
    }

    //NORMALIZAMOS LOS DATOS
    //OBTENEMOS LA PENDIENTE M Y EL INTERCEPTO B

    let (m, b) = get_ms_and_bs(&data, &minimos, &maximos);
    
    println!("");
    println!("-----------------------M y B:---------------------------"); //TODELETE
    for (i, v) in data.iter().enumerate() {
        println!("{}: m: {}, b: {}", v.header, m[i], b[i]); //TODELETE
    }

    //APLICAMOS LA NORMALIZACIÓN
    let data_norm:Vec<Vector> = normalization(&data, &minimos, &maximos);
    println!("");
    println!("-----------------------Datos normalizados:---------------------------"); //TODELETE
    for (_i, v) in data_norm.iter().enumerate() {
        println!("{}: {:?}", v.header, v.data); //TODELETE
    }

    //----------------------------CALCULOS----------------------------
    //Para N centros desde 2 hasta N
    let mut sol_anterior_centros:f64;
    let mut sol_centros: f64 = 0.0;
    let mut n: usize = 2;
    let mut cont_iter_centros: usize = 2;
    let mut end_iter_centros: bool = false;
    //VECTORES INTERNOS
    let mut vec_pertenencia_datos_centros: Vec<Vec<u8>> = Vec::new();

    let mut soluciones: Vec<Solución> = Vec::new();

    //Mientras la solución sea menor a la anterior seguimos iterando
    //CANTIDAD DE CENTROS
    while !end_iter_centros{
        let mut end_iter_iterations: bool = false;

        //ITERACIONES DE KMEANS
        let mut sol_anterior_kmeans: f64;
        let mut sol_kmeans: f64 = 0.0;
        let mut cont_iter_kmeans: usize = 0;
        let mut centros:Vec<Centro> = Vec::new();

        sol_anterior_centros= sol_centros;

        while !end_iter_iterations{

            if VERBOSE {
                println!("\n\n------------------------Centros {cont_iter_centros}:---------------------------"); //TODELETE
            }

            sol_anterior_kmeans = sol_kmeans;
            // println!(""); //TODELETE    
            if VERBOSE {
                println!("---------Iteración: {cont_iter_kmeans}--------------"); //TODELETE
            }

            //CENTROS
            //SI LA SOLUCIÓN ES 0, FIJAMOS LOS CENTROS EN 1/N DE LOS DATOS
            if sol_anterior_kmeans == 0.0 {
                //RECORREMOS LOS CENTROS
                for i in 0..n {
                    centros.push(Centro { headers: headers.clone(), valores: Vec::new() });
                    //RECORREMOS LOS VALORES DE LOS CENTROS
                    for (_j, _v) in data_norm.clone().iter().enumerate() {                        
                        let val = (1.0 / (n-1) as f64) * i as f64;
                        centros[i].valores.push(val);
                    }

                }
            }
            if VERBOSE {
                for (i, v) in centros.iter().enumerate() {
                    println!("Centro {}: {:?}", i, v.valores); //TODELETE
                }    
            }
            
            //Calculamos la distancia de cada punto a cada centro
            let distancias: Vec<Vec<f64>> = get_distancias_euclideanas(data_norm.clone(), &centros, n, cont_iter_kmeans);
            if VERBOSE{
                println!("");
                println!("-----------------------Distancias {n}-{cont_iter_kmeans}:---------------------------"); //TODELETE
                for (i, v) in distancias.iter().enumerate() {
                    println!("Centro {}:  {:?}", i, v); //TODELETE
                }
            }

            // let dist_min: Vec<f64> = get_minimos(distancias[0].clone(), distancias[1].clone());
            let dist_min: Vec<f64> = get_minimos(distancias.clone());
            if VERBOSE {
                println!("");
                println!("-----------------------Distancia minima {n}-{cont_iter_kmeans}:---------------------------"); //TODELETE
                println!("{:?}", dist_min); //TODELETE
            }

            //SUMA DE DISTANCIAS MINIMAS
            let suma_dist_min: f64 = dist_min.iter().sum();
            if VERBOSE {
                println!("");
                println!("-----------------------Suma de distancias minimas {n}-{cont_iter_kmeans}:---------------------------"); //TODELETE
                println!("{}", suma_dist_min); //TODELETE
            }

            //CATEGORIZAMOS LOS DATOS EN UN CENTRO
            //RECORREMOS LOS CENTROS
            vec_pertenencia_datos_centros = get_pertenencia_datos_centros(&distancias, &dist_min, &centros);
            if VERBOSE {
                println!("");
                println!("-----------------------Pertenencia de los datos a los centros {n}-{cont_iter_kmeans}:---------------------------"); //TODELETE
                for (i, v) in vec_pertenencia_datos_centros.iter().enumerate() {
                    let sum: u8 = v.iter().sum();
                    println!("Centro {}: {:?}. Sum: {sum}", i, v); //TODELETE
                }
            }

            //SETTEAMOS LOS NUEVOS CENTROS
            let mut nuevos_centros: Vec<Centro> = get_nuevos_centros(&data_norm, &vec_pertenencia_datos_centros.clone(), &centros, headers.clone());

            // IF MEDOIDES: TOMAMOS LOS NUEVOS CENTROS Y LOS MANDAMOS A MEDOIDES
            if MEDOIDES {
                nuevos_centros = get_centros_medoides(nuevos_centros, data_norm.clone(), vec_pertenencia_datos_centros.clone(), headers.clone());
            }
                //EL RESULTADO REESCRIBIRÁ LOS NUEVOS CENTROS
            // exit(0);
            //IF NO MEDOIDOES: CONTINUAMOS CON LOS NUEVOS CENTROS 

            
            //ACTUALIZAMOS LA SOLUCIÓN DE LAS ITERACIONES
            sol_kmeans = suma_dist_min;
            
            //COMPROBAMOS SI LA SOLUCIÓN ES MEJOR QUE LA ANTERIOR (CRITERIO DE PARO)
            if sol_kmeans < sol_anterior_kmeans || sol_anterior_kmeans == 0.0{
                //SI LA SOLUCIÓN ES MEJOR QUE LA ANTERIOR, SEGUIMOS ITERANDO
                end_iter_iterations = false;
                //SETTEAMOS LOS NUEVOS CENTROS SI LA SOLUCIÓN ES MEJOR O ES LA PRIMERA
                centros = nuevos_centros.clone();

                //IMPRIMIMOS LOS NUEVOS CENTROS
                if VERBOSE{
                    println!("-----------------------Nuevos centros {n}-{cont_iter_kmeans}:---------------------------"); //TODELETE
                    for (i, v) in nuevos_centros.iter().enumerate() {
                        println!("Centro {}: {:?}", i, v.valores); //TODELETE
                    }
                }

                //ACTUALIZAMOS LA SOLUCIÓN DE LOS CENTROS
                sol_centros = sol_kmeans;
            } else { //SI LA SOLUCIÓN NO ES MEJOR QUE LA ANTERIOR, TERMINAMOS LAS ITERACIONES
                end_iter_iterations = true;
            }
            cont_iter_kmeans += 1;
            
        }

        //SI LA SOLUCIÓN ES MEJOR QUE LA ANTERIOR O ES LA PRIMERA, GUARDAMOS LA SOLUCIÓN
        if sol_centros < sol_anterior_centros || sol_anterior_centros == 0.0 {
            end_iter_centros = false;
            n += 1;

            //GUARDAMOS LA SOLUCIÓN
            let sol = Solución { 
                centros: cont_iter_centros, 
                distancias: sol_kmeans, 
                iteraciones: cont_iter_kmeans, 
                vec_pertenencia_datos_centros: vec_pertenencia_datos_centros.clone(),
                coor_centros: centros.clone(),
            };
            soluciones.push(sol);

        } else { //SI LA SOLUCIÓN NO ES MEJOR QUE LA ANTERIOR, TERMINAMOS LAS ITERACIONES
            end_iter_centros = true;
        }
        //ACTUALIZAMOS EL CONTADOR DE ITERACIONES DE CENTROS
        cont_iter_centros += 1;
    }



    //IMPRIMIMOS LA SOLUCIÓN FINAL
    println!("");
    println!("-----------------------Soluciones:---------------------------"); //TODELETE
    for (i, v) in soluciones.iter().enumerate() {
        println!("\nSolución {}: Centros: {}, Distancias: {}, Iteraciones: {}", i, v.centros, v.distancias, v.iteraciones); //TODELETE
        //Vemos los centros
        for (j, _v) in v.coor_centros.iter().enumerate() {
            println!("Centro {}: {:?}", j, _v.valores); //TODELETE
        }
        //Vemos la pertenencia de los datos a los centros
        for (j, _v) in v.vec_pertenencia_datos_centros.iter().enumerate() {
            let sum: u8 = _v.iter().sum();
            println!("Items en Centro {}: {:?}. Sum: {sum}", j, _v); //TODELETE
        }
    }


}


//HAY QUE HACER QUE EL CENTRO APARENTE FORME PARTE DE LOS MEDOIDES (UN PUSH)
// nuevos_centros = get_centros_medoides(nuevos_centros, data_norm, vec_pertenencia_datos_centros, headers);
fn get_centros_medoides(nuevos_centros: Vec<Centro>, 
    data_norm: Vec<Vector>, 
    vec_pertenencia_datos_centros: Vec<Vec<u8>>, 
    headers: Vec<String>) -> Vec<Centro> {

    let mut medoides: Vec<Centro> = Vec::new();

    //OBTENEMOS LAS DISTANCIAS DE LOS DATOS A LOS CENTROS
    let distancias_centros: Vec<Vec<f64>> = get_distancias_euclideanas(data_norm.clone(), &nuevos_centros, nuevos_centros.len(), 0);

    // println!("\n Distancias: {:?}", distancias_centros); //TODELETE

    //RECORREMOS LOS CENTROS
    for i_centro in 0..nuevos_centros.len() {
        //BUSCAMOS EL MÍNIMO DE LAS DISTANCIAS DE LOS DATOS A LOS CENTROS
        let mut min: f64 = f64::INFINITY;
        let mut index: usize = 0;

        for (i, v) in distancias_centros[i_centro].iter().enumerate() {
            // if *v < min  && vec_pertenencia_datos_centros[i_centro][i] == 1{
            if *v < min{
                min = *v;
                index = i;
            }
        }
        //GUARDAMOS EL MEDOIDE
        let mut coor_medoides: Vec<f64> = Vec::new();
        for i in 0..data_norm.len() {
            coor_medoides.push(data_norm[i].data[index]);
        }

        medoides.push(Centro { headers: headers.clone(), valores: coor_medoides});
    }
    //IMPRIMIMOS LOS MEDOIDES
    println!("");
    println!("-----------------------Medoides:---------------------------"); //TODELETE
    for (i, v) in medoides.iter().enumerate() {
        println!("Medoide {}: {:?}", i, v.valores); //TODELETE
    }

    medoides
}



fn get_nuevos_centros(data_norm: &Vec<Vector>, vec_pertenencia_datos_centros: &Vec<Vec<u8>>, centros: &Vec<Centro>, headers: Vec<String>) -> Vec<Centro> {
    let mut nuevos_centros: Vec<Centro> = Vec::new();
    //RECORREMOS LOS CENTROS
    for i_centros in 0..centros.len() {
        nuevos_centros.push(Centro { headers: headers.clone(), valores: Vec::new() });
        //RECORREMOS LOS VALORES DE LOS CENTROS
        for i_val_centro in 0..centros[i_centros].valores.len() {
            let mut res: f64;
            //SUMA PRODUCTO DE LOS DATOS CON LA PERTENENCIA DE LOS DATOS A LOS CENTROS        
            res = suma_producto_f64(data_norm[i_val_centro].data.clone(), vec_pertenencia_datos_centros[i_centros].clone());
            //SUMA DEL VALOR DEL CENTRO ANTERIOR
            res += centros[i_centros].valores[i_val_centro];
            //DIVIDIMOS ENTRE LA SUMA DE LA PERTENENCIA DE LOS DATOS A LOS CENTROS
            let mut suma_pertenencia: u32 = 0;
            for i in 0..vec_pertenencia_datos_centros[i_centros].len() {
                suma_pertenencia += vec_pertenencia_datos_centros[i_centros][i] as u32;
            }
            res /= suma_pertenencia as f64 +1 as f64;

            nuevos_centros[i_centros].valores.push(res);
        }
    }
    nuevos_centros
}

fn get_pertenencia_datos_centros(distancias: &Vec<Vec<f64>>, dist_min: &Vec<f64>, centros: &Vec<Centro>) -> Vec<Vec<u8>> {
    let mut vec_pertenencia_datos_centros: Vec<Vec<u8>> = Vec::new();
    for i_centro in 0..centros.len() {
        let mut vec_pertenencia_centro: Vec<u8> = Vec::new();
        for i_distancias_minimas in 0..dist_min.len() {
            let mut pertenencia: u8 = 0;
            if distancias[i_centro][i_distancias_minimas] == dist_min[i_distancias_minimas] {
                pertenencia = 1;
            }
            vec_pertenencia_centro.push(pertenencia);
        }
        vec_pertenencia_datos_centros.push(vec_pertenencia_centro);
    }
    vec_pertenencia_datos_centros
}

fn get_distancias_euclideanas(data_norm: Vec<Vector>, centros: &Vec<Centro>, _n:usize, _i: usize)-> Vec<Vec<f64>>{
    let mut distancias: Vec<Vec<f64>> = Vec::new();
    //RECORREMOS LOS CENTROS
    
    for centro in centros.iter(){
        let mut vec_centro: Vec<f64> = Vec::new();
        //RECORREMOS TODOS LOS DATOS DE LA COLUMNA (USAMOS UN ITERADOR ENTONCES DA IGUAL CUAL COLUMNA TOMAR)
        for i_dato in 0..data_norm[0].data.len() { //0 a 20 en el ejemplo
            let mut dist: f64 = 0.0;
            //RECORREMOS EL N DE LAS COLUMNAS
            for i_col in 0..data_norm.len() { // 0 a 3 en el ejemplo
                dist += (data_norm[i_col].data[i_dato] - centro.valores[i_col]).powi(2);
                
            }
            dist = dist.sqrt();
            vec_centro.push(dist);
            // print!("{}, ", dist); //TODELETE
            
        }

        distancias.push(vec_centro);
    }
    distancias
}

//ESTO SACA EL MÍNIMO DE DOS VECTORES, NECESTIAMOS UNA FUNCIÓN QUE HAGA LO MISMO PERO CON N VECTORES
// fn _get_minimos(vec_a: Vec<f64>, vec_b: Vec<f64>) -> Vec<f64> {
//     let mut minimos:Vec<f64> = Vec::new();
//     for i in 0..vec_a.len() {
//         minimos.push(vec_a[i].min(vec_b[i]));
//     }
//     minimos
// }
fn get_minimos(vectores: Vec<Vec<f64>>) -> Vec<f64> {
    let mut minimos:Vec<f64> = Vec::new();
    for i in 0..vectores[0].len() {
        let mut min: f64 = f64::INFINITY;
        for j in 0..vectores.len() {
            if vectores[j][i] < min {
                min = vectores[j][i];
            }
        }
        minimos.push(min);
    }
    minimos
}


fn suma_producto_f64(vec_a: Vec<f64>, vec_b: Vec<u8>) -> f64{
    let mut suma: f64 = 0.0;
    for i in 0..vec_a.len() {
        suma += vec_a[i] * vec_b[i] as f64;
    }
    suma
}

fn normalization(data: &Vec<Vector>, minimos: &Vec<f64>, maximos: &Vec<f64>) -> Vec<Vector> {
    let mut data_norm:Vec<Vector> = Vec::new();
    for (i, v) in data.iter().enumerate() {
        data_norm.push(Vector { header: v.header.clone(), data: Vec::new() });
        for d in v.data.iter(){
            data_norm[i].data.push((d - minimos[i]) / (maximos[i] - minimos[i]));
        }
    }
    data_norm
}

fn get_ms_and_bs(data: &Vec<Vector>, minimos: &Vec<f64>, maximos: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut m:Vec<f64> = Vec::new();
    let mut b:Vec<f64> = Vec::new();

    for i in 0..data.len() {
        m.push(1.0 / (maximos[i] - minimos[i]));
        b.push(-minimos[i] / (maximos[i] - minimos[i]));
    }

    (m, b)
}
fn minimos_maximos(data: &Vec<Vector>) -> (Vec<f64>, Vec<f64>) {
    let mut minimos:Vec<f64> = Vec::new();
    let mut maximos:Vec<f64> = Vec::new();
    for v in data.iter() {
        minimos.push(v.data.iter().fold(f64::INFINITY, |a, &b| a.min(b)));
        maximos.push(v.data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)));
    }
    (minimos, maximos)
}

fn read_csv(data: &mut Vec<Vector>, headers: &mut Vec<String>, path: &str) {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .unwrap();

    //Obtenemos los headers
    let _headers = rdr.headers().unwrap().clone();
    for h in _headers.iter() {
        headers.push(h.to_string());
    }

    //Obtenemos los datos
    for (i, result) in rdr.records().enumerate() {
        let record = result.unwrap();
        for (j, col ) in record.iter().enumerate() {
            if i == 0 {
                data.push(Vector { header: headers[j].to_string(), data: Vec::new() });
            }
            data[j].data.push(col.parse::<f64>().unwrap());
        }
    }
}
