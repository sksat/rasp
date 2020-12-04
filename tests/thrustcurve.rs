extern crate rasp;
extern crate ureq;

#[test]
fn k240() {
    let url = "https://www.thrustcurve.org/simfiles/5f4294d20002e90000000881/download/Hypertek_835CC125J-K240.eng";
    let k240 = ureq::get(url).call().into_string().unwrap();
    let k240 = rasp::from_str(&k240).unwrap();

    println!("{}", k240);

    assert_eq!(k240.name, "K240-835CC125J");
    assert_eq!(k240.manufacturer, "HyperTek")
}
