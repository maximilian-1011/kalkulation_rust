pub fn umschlagshäufigkeit(umsatz: f64, durchschnittlicher_lagerbestand: f64) -> f64 {
    let decimal = umsatz / durchschnittlicher_lagerbestand;
    (decimal * 100.0).round() / 100.0
}

pub fn durchschnittlicher_lagerbestand(anfangsbestand: f64, monatsbestände: Vec<f64>) -> f64 {
    let sum_of_months: f64 = monatsbestände.iter().sum();
    let total: f64 = anfangsbestand + sum_of_months;
    let output: f64 = total / (monatsbestände.len() as f64 + 1.0);
    (output * 100.0).round() / 100.0
}
