extern crate rasp;

#[test]
fn a8() {
    let tc_str = r#"; Estes A8 RASP.ENG file made from NAR published data
; File produced March 3, 2011
; The total impulse, peak thrust, average thrust and burn time are
; the same as the averaged static test data on the NAR web site in
; the certification file. The curve drawn with these data points is as
; close to the certification curve as can be with such a limited
; number of points (32) allowed with wRASP up to v1.6.
 A8 18 70 0 .00384 .01389 E
0.016 1.080
0.036 0.293
0.050 0.259
0.060 0.357
0.070 0.419
0.080 0.355
0.102 0.781
0.124 1.443
0.152 2.966
0.170 4.426
0.202 7.160
0.224 9.051
0.246 9.555
0.256 9.317
0.276 7.958
0.294 6.108
0.326 4.235
0.350 3.813
0.376 3.997
0.410 3.926
0.452 3.724
0.474 4.046
0.502 4.092
0.532 4.146
0.534 0.000"#;

    let engine = rasp::from_str(tc_str).unwrap();

    println!("{}", engine);

    assert_eq!(engine.name, "A8");
    assert_eq!(engine.diameter, 18);
    assert_eq!(engine.length, 70);
    assert_eq!(engine.delay, None);
    assert_eq!(engine.weight_propellant, 0.00384);
    assert_eq!(engine.weight_init, 0.01389);
    assert_eq!(engine.manufacturer, "E");

    let first = &engine.thrust.curve[0];
    assert_eq!(first.time, 0.016);
    assert_eq!(first.thrust, 1.080);

    let last = &engine.thrust.curve.last().unwrap();
    assert_eq!(last.time, 0.534);
    assert_eq!(last.thrust, 0.0);
}
