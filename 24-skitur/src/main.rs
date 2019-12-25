use gnuplot::{Caption, Color, Figure, LineWidth};
use std::fs;

fn plot_tur(tur: &str, filename: &str) {
    let mut x = Vec::new();
    let mut y = Vec::new();
    for coord in tur.lines() {
        let coord = coord.trim().split(",").collect::<Vec<&str>>();
        x.push(coord[0].parse::<u32>().unwrap());
        y.push(coord[1].parse::<u32>().unwrap());
    }

    let mut fg = Figure::new();
    fg.axes2d()
        .points(&x, &y, &[Caption("Tur"), Color("red"), LineWidth(5.0)]);
    fg.save_to_png(&format!("plots/{}.png", filename), 350, 350)
        .unwrap();
}

fn main() {
    let turer = fs::read_to_string("turer.txt").unwrap();
    for (idx, tur) in turer
        .trim()
        .split("---")
        .collect::<Vec<&str>>()
        .iter()
        .enumerate()
    {
        let tur = tur.trim();
        plot_tur(&tur, &format!("tur_{}", idx));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plot_tur() {
        let tur = "3,0\n2,0\n1,0\n0,0\n0,1\n0,2\n0,3\n0,4\n0,5";
        plot_tur(&tur, "test_plot");
    }
}
