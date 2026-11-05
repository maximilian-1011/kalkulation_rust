pub fn preis_reduzierung(listenpreis: f64, rabatt: f64) -> f64 {
    let rabatt_decimal: f64 = rabatt / 100.0;
    let new_price: f64 = listenpreis - (listenpreis * rabatt_decimal);
    let clean_price: f64 = (new_price * 100.0).round() / 100.0;
    clean_price
}

pub fn bezugspreis(preis: f64, kosten: f64) -> f64 {
    let new_price: f64 = preis + kosten;
    let clean_price: f64 = (new_price * 100.0).round() / 100.0;
    clean_price
}
