use lv03::Lv03;

// Use wit_bindgen to generate the bindings from the component model to Rust.
// For more information see: https://github.com/bytecodealliance/wit-bindgen/
wit_bindgen::generate!({
    path: "..",
    world: "convert",
});

struct GuestComponent;

export!(GuestComponent);

impl Guest for GuestComponent {
    fn to_wgs84(coord: Lv03coord) -> (f64, f64, f64) {
        let lv03 = Lv03::new(coord.x, coord.y, coord.z).unwrap();
        let wgs84 = lv03.to_wgs84();
        let msg = format!("{:?}", wgs84);
        host::log(&msg);
        return (wgs84.longitude, wgs84.latitude, wgs84.altitude);
    }
}
