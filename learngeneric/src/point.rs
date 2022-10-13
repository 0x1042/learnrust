use tracing::info;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct PointV2<T1, T2> {
    x: T1,
    y: T2,
}

#[derive(Debug)]
struct PointV3<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointV3<T, U> {
    fn mixup<V, W>(self, other: PointV3<V, W>) -> PointV3<T, W> {
        PointV3 {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn test_point() {
    let ip: Point<i32> = Point { x: 1, y: 2 };

    let fp = Point { x: 1.23, y: 3.24 };

    info!("ip :{:?}", ip);
    info!("fp :{:?}", fp);

    let p = PointV2 {
        x: 5,
        y: "hello".to_string(),
    };
    info!("p :{:?}", p);

    let p1 = PointV3 { x: 5, y: 10 };
    let p2 = PointV3 {
        x: "Hello",
        y: '中',
    };

    let p3 = p1.mixup(p2);

    info!("p3:{:?}", p3);

    let p4: Point<f32> = Point {
        x: 5_f32,
        y: 10_f32,
    };
    info!("{}", p4.distance_from_origin())
}
