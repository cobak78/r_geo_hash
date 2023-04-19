pub fn distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64, unit: &str) -> f64 {
    let theta = lon1 - lon2;
    let dist = (lat1.to_radians().sin() * lat2.to_radians().sin()
        + lat1.to_radians().cos() * lat2.to_radians().cos() * theta.to_radians().cos())
    .acos();
    let dist = dist.to_degrees();
    let miles = dist * 60.0 * 1.1515;

    match unit.to_uppercase().as_str() {
        "K" | "KM" => miles * 1.609344,
        "MN" | "N" => miles * 0.8684,
        _ => miles,
    }
}

pub fn get_geo_hash_precision(geo_bound_box: &[(f64, f64); 2], squares: i32, unit: &str) -> i32 {
    let remainder = squares % 2;
    let x_quotient = squares / 2;
    let y_quotient = x_quotient / 2;

    if 0 != remainder {
        panic!("geoHash divisions must be multiple of 2");
    }

    // get x distance
    let x_dist = distance(
        geo_bound_box[0].0,
        geo_bound_box[0].1,
        geo_bound_box[1].0,
        geo_bound_box[1].1,
        unit,
    ) * 1000.0;

    // get y distance
    let y_dist = distance(
        geo_bound_box[0].0,
        geo_bound_box[0].1,
        geo_bound_box[1].0,
        geo_bound_box[1].1,
        unit,
    ) * 1000.0;

    // geohash distances
    let x_geohash_dist = x_dist / x_quotient as f64;
    let y_geohash_dist = y_dist / y_quotient as f64;

    get_precision_from_area(x_geohash_dist, y_geohash_dist)
}

fn get_precision_from_area(width: f64, height: f64) -> i32 {
    // precision map
    let precision_map: Vec<(f64, f64)> = vec![
        (5009400.0, 4992000.6),
        (1252300.0, 624100.0),
        (156500.0, 156000.0),
        (39100.0, 19500.0),
        (4900.0, 4900.0),
        (1200.0, 609.4),
        (152.9, 152.4),
        (38.2, 19.0),
        (4.8, 4.8),
        (1.2, 0.595),
        (0.14, 0.149),
        (0.037, 0.019),
    ];

    for (key, value) in precision_map.iter().enumerate() {
        if width > value.0 || height > value.1 {
            return key as i32 + 1;
        }
    }

    return 12;
}

#[test]
fn test_distance() {
    const TOP_LEFT: [f64; 2] = [41.5926176, 2.1693494];
    const BOTTOM_RIGHT: [f64; 2] = [41.3326176, 2.0693494];
    const OPEN_TOP_LEFT: [f64; 2] = [45.5926176, 2.5693494];
    const OPEN_BOTTOM_RIGHT: [f64; 2] = [35.3326176, 1.5693494];
    const KM_DISTANCE: f64 = 30.086138600645093;
    const KM_DISTANCE_OPEN: f64 = 1143.9106866431741;
    const MILES_DISTANCE: f64 = 18.694659812100515;
    const MILES_DISTANCE_OPEN: f64 = 710.7931471724964;
    const NAUTICAL_MILES_DISTANCE: f64 = 16.234442580828087;
    const NAUTICAL_MILES_DISTANCE_OPEN: f64 = 617.2527690045958;

    let result = distance(
        TOP_LEFT[0],
        TOP_LEFT[1],
        BOTTOM_RIGHT[0],
        BOTTOM_RIGHT[1],
        "K",
    );
    assert_eq!(KM_DISTANCE, result);

    let result_open = distance(
        OPEN_TOP_LEFT[0],
        OPEN_TOP_LEFT[1],
        OPEN_BOTTOM_RIGHT[0],
        OPEN_BOTTOM_RIGHT[1],
        "K",
    );
    assert_eq!(KM_DISTANCE_OPEN, result_open);
    assert!(result < result_open);

    let result = distance(
        TOP_LEFT[0],
        TOP_LEFT[1],
        BOTTOM_RIGHT[0],
        BOTTOM_RIGHT[1],
        "M",
    );
    assert_eq!(MILES_DISTANCE, result);

    let result_open = distance(
        OPEN_TOP_LEFT[0],
        OPEN_TOP_LEFT[1],
        OPEN_BOTTOM_RIGHT[0],
        OPEN_BOTTOM_RIGHT[1],
        "M",
    );
    assert_eq!(MILES_DISTANCE_OPEN, result_open);
    assert!(result < result_open);

    let result = distance(
        TOP_LEFT[0],
        TOP_LEFT[1],
        BOTTOM_RIGHT[0],
        BOTTOM_RIGHT[1],
        "N",
    );
    assert_eq!(NAUTICAL_MILES_DISTANCE, result);

    let result_open = distance(
        OPEN_TOP_LEFT[0],
        OPEN_TOP_LEFT[1],
        OPEN_BOTTOM_RIGHT[0],
        OPEN_BOTTOM_RIGHT[1],
        "N",
    );
    assert_eq!(NAUTICAL_MILES_DISTANCE_OPEN, result_open);
    assert!(result < result_open);
}
