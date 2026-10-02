pub fn zieleinkaufspreis(listenpreis: f64, rabatt: f64) -> f64 {
    if rabatt <= 0.0 {
        println!("Zieleinkaufspreis: {listenpreis}");
        listenpreis
    } else {
        let rabatt_in_euro = listenpreis * (rabatt / 100.0);
        let output = listenpreis - rabatt_in_euro;
        let res = (output * 100.0).round() / 100.0;
        println!("Zieleinkaufspreis: {res}");
        res
    }
}

pub fn bareinkaufspreis(zieleinkaufspreis: f64, skonto: f64) -> f64 {
    if skonto <= 0.0 {
        println!("Barkeinkaufspreis: {zieleinkaufspreis}");
        zieleinkaufspreis
    } else {
        let rabatt_in_euro = zieleinkaufspreis * (skonto / 100.0);
        let output = zieleinkaufspreis - rabatt_in_euro;
        let result = (output * 100.0).round() / 100.0;
        println!("Barkeinkaufspreis: {result}");
        result
    }
}

fn beschaffungspreis(bareinkaufspreis: f64, bezugskosten: f64) -> f64 {
    println!("Bezugspreis: {}", bareinkaufspreis + bezugskosten);
    bareinkaufspreis + bezugskosten
}

pub fn selbstokstenpreis(beschaffungspreis: f64, handlungskosten: f64) -> f64 {
    if handlungskosten == 0.0 {
        println!("Selbstkostenpreis: {beschaffungspreis}");
        beschaffungspreis
    } else {
        let handlungskosten_decimal = handlungskosten / 100.0;
        let handlungskosten_euro = beschaffungspreis * handlungskosten_decimal;
        let output = beschaffungspreis + handlungskosten_euro;
        let res = (output * 100.0).round() / 100.0;
        println!("Selbstkostenpreis: {res}");
        res
    }
}

pub fn nettoverkaufspreis(selbstkotenpreis: f64, gewinnzuschlag: f64) -> f64 {
    if gewinnzuschlag == 0.0 {
        println!("Nettoverkaufspreis: {selbstkotenpreis}");
        selbstkotenpreis
    } else {
        let gewin_euro = selbstkotenpreis * (gewinnzuschlag / 100.0);
        let output = selbstkotenpreis + gewin_euro;
        let res = (output * 100.0).round() / 100.0;
        println!("Nettoverkaufspreis: {res}");
        res
    }
}

pub fn bruttoverkaufspreis(nettoverkaufspreis: f64, umsatzsteuer: f64) -> f64 {
    if umsatzsteuer == 0.0 {
        println!("Bruttoverkaufspreis: {nettoverkaufspreis}");
        nettoverkaufspreis
    } else {
        let steuer_euro = nettoverkaufspreis * (umsatzsteuer / 100.0);
        let ouptut = nettoverkaufspreis + steuer_euro;
        let res = (ouptut * 100.0).round() / 100.0;
        println!("Bruttoverkaufspreis: {res}");
        res
    }
}

pub fn vollständigekalkulation(
    listenpreis: f64,
    rabatt: f64,
    skonto: f64,
    bezugskosten: f64,
    handlungskosten: f64,
    gewinnzuschlag: f64,
    umsatzsteuer: f64,
) {
    let zieleinkaufspreis = zieleinkaufspreis(listenpreis, rabatt);
    let bareinkaufspreis = bareinkaufspreis(zieleinkaufspreis, skonto);
    let bezugspreis = beschaffungspreis(bareinkaufspreis, bezugskosten);
    let selbstkostenpreis = selbstokstenpreis(bezugspreis, handlungskosten);
    let nettopreis = nettoverkaufspreis(selbstkostenpreis, gewinnzuschlag);
    let bruttopreis = bruttoverkaufspreis(nettopreis, umsatzsteuer);

    println!("\nDas Ergebnis ist: {bruttopreis}€\n");
}
