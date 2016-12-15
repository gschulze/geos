extern crate geos;
use geos::{version, init, finish, GeosGeom, GeosPrepGeom};

fn main() {
    init();
    println!("geos_c version: {}", version());
    let g1 = GeosGeom::new("POLYGON ((0 0, 0 5, 5 5, 5 0, 0 0))");
    let g2 = GeosGeom::new("POLYGON ((1 1, 1 3, 5 5, 5 0, 1 1))");
    let pg1 = GeosPrepGeom::new(&g1);
    let result = pg1.intersects(&g2);
    let result2 = pg1.contains(&g2.get_centroid());
    println!("Prepared geometry intersects test polygon : {:?}", result);
    println!("Prepared geometry contains centroid other polygon : {:?}", result2);
    println!("Prepared geometry intersects each geometry from a vec of GeosGeometry :");
    let vec_geoms = vec![
        GeosGeom::new("POINT (1.3 2.4)"),
        GeosGeom::new("POINT (2.1 0.3)"),
        GeosGeom::new("POINT (3.1 4.7)"),
        GeosGeom::new("POINT (0.4 4.1)")
        ];
    for geom in &vec_geoms {
        print!("{:?} ", pg1.intersects(&geom));
    }
    println!("");
    finish();
}
