pub fn umschlagshäufigkeit(umsatz: f64, durchschnittlicher_lagerbestand: f64) -> f64 {
    let decimal = umsatz / durchschnittlicher_lagerbestand;
    (decimal * 100.0).round() / 100.0
}

pub fn durchschnittlicher_lagerbestand(bestände: Vec<f64>) -> f64 {
    let sum_of_months: f64 = bestände.iter().sum();
    let output: f64 = sum_of_months / (bestände.len() as f64);
    (output * 100.0).round() / 100.0
}

pub fn durchschnittliche_lagerdauer(umschlag: f64) -> f64 {
    let res = 360.0 / umschlag;
    (res * 100.0).round() / 100.0
}

pub fn lagerzinssatz(jahreszinssatz: f64, durchschnittliche_lagerdauer: f64) -> f64 {
    let output = (jahreszinssatz * durchschnittliche_lagerdauer) / 360.0;
    let res = (output * 100.0).round() / 100.0;
    println!("Lagerzinssatz: {res}%");
    res
}

pub fn lagerzins(lagerzinssatz: f64, durchschnittlicher_lagerbestand: f64) -> f64 {
    let output = (lagerzinssatz * durchschnittlicher_lagerbestand) / 100.0;
    let res = (output * 100.0).round() / 100.0;
    println!("Lagerzins: {res}€");
    res
}
