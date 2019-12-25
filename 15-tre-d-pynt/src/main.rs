use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
struct Coord {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone)]
struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone)]
struct Triangle {
    p1: Coord,
    p2: Coord,
    p3: Coord,
}

#[derive(Debug, Clone)]
struct Julepynt {
    triangles: Vec<Triangle>,
}

impl Vector {
    fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - other.y * self.z,
            y: self.x * other.z - other.z * self.z,
            z: self.x * other.y - other.x * self.y,
        }
    }

    fn dot(&self, other: &Vector) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl Triangle {
    // volume of the pyramid formed from <0, 0, 0> to our triangle in mm
    fn volume_from_origin(&self) -> f32 {
        let p = Vector {
            x: self.p2.x - self.p1.x,
            y: self.p2.y - self.p1.y,
            z: self.p2.z - self.p1.z,
        };
        let q = Vector {
            x: self.p3.x - self.p1.x,
            y: self.p3.y - self.p1.y,
            z: self.p3.z - self.p1.z,
        };
        let r = Vector {
            x: 0.0 - self.p1.x,
            y: 0.0 - self.p1.y,
            z: 0.0 - self.p1.z,
        };
        let vol = (1.0 / 6.0) * q.cross(&p).dot(&r);
        vol.abs()
    }
}

impl Julepynt {
    fn volume_in_ml(&self) -> f32 {
        let volume_in_mm = self
            .triangles
            .iter()
            .map(|t| t.volume_from_origin())
            .sum::<f32>();
        volume_in_mm / 1000.0
    }
}

fn parse_triangles(model: &str) -> Vec<Triangle> {
    model
        .lines()
        .map(|l| {
            let coords: Vec<f32> = l
                .split(",")
                .map(|c| c.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();
            Triangle {
                p1: Coord {
                    x: coords[0],
                    y: coords[1],
                    z: coords[2],
                },
                p2: Coord {
                    x: coords[3],
                    y: coords[4],
                    z: coords[5],
                },
                p3: Coord {
                    x: coords[6],
                    y: coords[7],
                    z: coords[8],
                },
            }
        })
        .collect()
}

fn count_julepynt(ts: &Vec<Triangle>) -> usize {
    // Logic: If the point doesn't already exist, it's unique
    let mut coords: HashSet<String> = HashSet::new();

    for t in ts.iter() {
        let p1 = format!("{:?}", t.p1);
        let p2 = format!("{:?}", t.p2);
        let p3 = format!("{:?}", t.p3);
        coords.insert(p1);
        coords.insert(p2);
        coords.insert(p3);
    }
    coords.len()
}

fn main() {
    let model = fs::read_to_string("MODEL.CSV").unwrap();
    let triangles = parse_triangles(&model);
    println!("calculating the volume of {} pyramids", triangles.len());
    let julepynt = Julepynt { triangles };
    let total_vol = julepynt.volume_in_ml();
    println!("{}", total_vol);
    println!("unique points: {}", count_julepynt(&julepynt.triangles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_julepynt_volume() {
        let model =
            "0,0,0,10,0,0,0,0,10\n0,0,0,0,0,10,0,10,0\n0,0,0,0,10,0,10,0,0\n10,0,0,0,10,0,0,0,10";
        let triangles = parse_triangles(&model);
        let julepynt = Julepynt { triangles };
        assert_eq!(format!("{:.3}", julepynt.volume_in_ml()), "0.167");
    }
}
