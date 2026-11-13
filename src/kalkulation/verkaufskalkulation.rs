use crate::kalkulation::beschaffung;

pub fn zieleinkaufspreis(listenpreis: f64, rabatt: f64) -> f64 {
    if rabatt <= 0.0 {
        println!("Zieleinkaufspreis: {listenpreis}");
        listenpreis
    } else {
        let res = beschaffung::preis_reduzierung(listenpreis, rabatt);
        println!("Zieleinkaufspreis: {res}");
        res
    }
}

pub fn bareinkaufspreis(zieleinkaufspreis: f64, skonto: f64) -> f64 {
    if skonto <= 0.0 {
        println!("Barkeinkaufspreis: {zieleinkaufspreis}");
        zieleinkaufspreis
    } else {
        let result = beschaffung::preis_reduzierung(zieleinkaufspreis, skonto);
        println!("Barkeinkaufspreis: {result}");
        result
    }
}

fn beschaffungspreis(bareinkaufspreis: f64, bezugskosten: f64) -> f64 {
    println!(
        "Bezugspreis: {}",
        beschaffung::bezugspreis(bareinkaufspreis, bezugskosten)
    );
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
    println!();
    let zieleinkaufspreis = zieleinkaufspreis(listenpreis, rabatt);
    let bareinkaufspreis = bareinkaufspreis(zieleinkaufspreis, skonto);
    let bezugspreis = beschaffungspreis(bareinkaufspreis, bezugskosten);
    let selbstkostenpreis = selbstokstenpreis(bezugspreis, handlungskosten);
    let nettopreis = nettoverkaufspreis(selbstkostenpreis, gewinnzuschlag);
    let bruttopreis = bruttoverkaufspreis(nettopreis, umsatzsteuer);

    println!("\nDas Ergebnis ist: {bruttopreis}€\n");
}

pub fn vereinfacht(kalkulationszuschlag: f64, bezugspreis: f64) -> f64 {
    let kalkulationsfaktor = 1.0 + (kalkulationszuschlag.round() / 100.0);
    let res = (bezugspreis * kalkulationsfaktor * 100.0).round() / 100.0;
    println!("Bruttoverkaufspreis: {res}");
    res
}

pub fn kalkulationsfaktor(bezugspreis: f64, bruttoverkaufspreis: f64) -> f64 {
    let kf = ((bruttoverkaufspreis / bezugspreis) * 100.0).round() / 100.0;
    println!("Kalkulationsfaktor: {kf}");
    kf
}

pub fn kalkulationszuschlag(bezugspreis: f64, bruttoverkaufspreis: f64) -> f64 {
    let zuschlag: f64 = ((bruttoverkaufspreis - bezugspreis) / bezugspreis) * 100.0;
    let res: f64 = (zuschlag * 100.0).round() / 100.0;
    println!("kalkulationszuschlag: {res}");
    res
}

pub fn rückwärtskalkulation(
    bruttoverkaufspreis: f64,
    umsatzsteuer: f64,
    gewinnzuschlag: f64,
    handlungskosten: f64,
    bezugskosten: f64,
    skonto: f64,
    rabatt: f64,
) -> f64 {
    let nettoverkaufspreis: f64 =
        ((bruttoverkaufspreis / (1.0 + (umsatzsteuer / 100.0))) * 100.0).round() / 100.0;
    println!("Nettoverkaufspreis: {nettoverkaufspreis}");
    let selbstkostenpreis: f64 =
        ((nettoverkaufspreis / (1.0 + (gewinnzuschlag / 100.0))) * 100.0).round() / 100.0;
    println!("Selbstkostenpreis: {selbstkostenpreis}");
    let bezugspreis: f64 =
        ((selbstkostenpreis / (1.0 + (handlungskosten / 100.0))) * 100.0).round() / 100.0;
    println!("Bezugspreis: {bezugspreis}");
    let bareinkaufspreis: f64 = ((bezugspreis - bezugskosten) * 100.0).round() / 100.0;
    println!("Bareinkaufspreis: {bareinkaufspreis}");
    let zeileinkaufspreis: f64 =
        ((bareinkaufspreis / (1.0 - (skonto / 100.0))) * 100.0).round() / 100.0;
    println!("Zieleinkaufspreis: {zeileinkaufspreis}");
    let listenpreis: f64 = ((zeileinkaufspreis / (1.0 - (rabatt / 100.0))) * 100.0).round() / 100.0;
    println!("Listenpreis: {listenpreis}");
    listenpreis
}
