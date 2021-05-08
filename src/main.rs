const K2: f64 = 5.0;
const R2: f64 = 2.0;
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 22;
const PI: f64 = 3.1415926;
const THETA_SPACING: f64 = 0.07;
const PHI_SPACING: f64 = 0.02;

fn main() {
    let (mut a, mut b) = (0.0, 0.0);

    loop {
        a += THETA_SPACING;
        b += PHI_SPACING;
        render_frame(a, b);
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn render_frame(a: f64, b: f64) {
    let ((sin_a, cos_a), (sin_b, cos_b)) = (a.sin_cos(), b.sin_cos());
    let mut output = [' '; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut z_buf = [0.0; SCREEN_WIDTH * SCREEN_HEIGHT];
    let mut theta = 0.0;

    while theta <= 2.0 * PI {
        let (sin_theta, cos_theta) = theta.sin_cos();
        let mut phi: f64 = 0.0;

        while phi <= 2.0 * PI {
            let (sin_phi, cos_phi) = phi.sin_cos();
            let circle_x = cos_theta + R2;
            let (d, t) = (
                1.0 / (sin_phi * circle_x * sin_a + sin_theta * cos_a + K2),
                sin_phi * circle_x * cos_a - sin_theta * sin_a,
            );
            let (x, y) = (
                ((SCREEN_WIDTH / 2) as f64 + 30.0 * d * (cos_phi * circle_x * cos_b - t * sin_b))
                    as usize,
                ((SCREEN_HEIGHT / 2) as f64 + 15.0 * d * (cos_phi * circle_x * sin_b + t * cos_b))
                    as usize,
            );
            let (index, n) = (
                x + SCREEN_WIDTH * y,
                8.0 * ((sin_theta * sin_a - sin_phi * cos_theta * cos_a) * cos_b
                    - sin_phi * cos_theta * sin_a
                    - sin_theta * cos_a
                    - cos_phi * cos_theta * sin_b),
            );
            if y < SCREEN_HEIGHT && x < SCREEN_WIDTH && d > z_buf[index] {
                z_buf[index] = d;
                output[index] = "cwkyydsflxg!"
                    .chars()
                    .nth(n as usize)
                    .or(Some('.'))
                    .unwrap();
            }
            phi += PHI_SPACING;
        }
        theta += THETA_SPACING;
    }
    print!(
        "\x1B[H{}",
        output
            .chunks(SCREEN_WIDTH)
            .map(|i| i.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    )
}
