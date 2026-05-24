use wasm_bindgen::prelude::*;

const PEG_RADIUS: f32 = 6.0;
const BALL_RADIUS: f32 = 5.0;
const GRAVITY: f32 = 1100.0;
const BOUNCE: f32 = 0.55;
const AIR_DRAG: f32 = 0.995;

#[derive(Clone, Copy)]
struct Peg {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Clone, Copy)]
struct Ball {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
}

#[wasm_bindgen]
pub struct Game {
    width: f32,
    height: f32,
    bins: Vec<u32>,
    bin_scores: Vec<u32>,
    pegs: Vec<Peg>,
    balls: Vec<Ball>,
    score: u32,
    rng_state: u64,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f32, height: f32, bin_count: usize) -> Game {
        let mut game = Game {
            width,
            height,
            bins: vec![0; bin_count.max(3)],
            bin_scores: vec![],
            pegs: vec![],
            balls: vec![],
            score: 0,
            rng_state: 0xA5A5_1234_FF01_EE77,
        };
        game.bin_scores = build_bin_scores(game.bins.len());
        game.generate_pegs();
        game
    }

    pub fn reset(&mut self) {
        self.balls.clear();
        self.score = 0;
        for b in &mut self.bins {
            *b = 0;
        }
    }

    pub fn drop_ball(&mut self, x: f32) {
        let spawn_x = x.clamp(BALL_RADIUS + 2.0, self.width - BALL_RADIUS - 2.0);
        let jitter = (self.rand_f32() - 0.5) * 12.0;
        self.balls.push(Ball {
            x: spawn_x,
            y: 22.0,
            vx: jitter,
            vy: 0.0,
            r: BALL_RADIUS,
        });
    }

    pub fn auto_drop_center(&mut self) {
        let rnd = self.rand_f32();
        let x = self.width * 0.5 + (rnd - 0.5) * (self.width * 0.2);
        self.drop_ball(x);
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.033);
        if dt <= 0.0 {
            return;
        }

        let mut survivors: Vec<Ball> = Vec::with_capacity(self.balls.len());
        let bins_len = self.bins.len();
        let bin_w = self.width / bins_len as f32;

        for mut ball in self.balls.drain(..) {
            ball.vy += GRAVITY * dt;
            ball.vx *= AIR_DRAG;

            ball.x += ball.vx * dt;
            ball.y += ball.vy * dt;

            if ball.x - ball.r < 0.0 {
                ball.x = ball.r;
                ball.vx = -ball.vx * BOUNCE;
            }
            if ball.x + ball.r > self.width {
                ball.x = self.width - ball.r;
                ball.vx = -ball.vx * BOUNCE;
            }

            for peg in &self.pegs {
                collide_ball_with_peg(&mut ball, *peg);
            }

            let floor_y = self.height - 30.0;
            if ball.y + ball.r >= floor_y {
                let idx = ((ball.x / bin_w).floor() as usize).min(bins_len - 1);
                self.bins[idx] += 1;
                self.score += self.bin_scores[idx];
                continue;
            }

            if ball.y < self.height + 200.0 {
                survivors.push(ball);
            }
        }

        self.balls = survivors;
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn ball_count(&self) -> usize {
        self.balls.len()
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn bin_count(&self) -> usize {
        self.bins.len()
    }

    pub fn bin_scores(&self) -> Vec<u32> {
        self.bin_scores.clone()
    }

    pub fn bins(&self) -> Vec<u32> {
        self.bins.clone()
    }

    /// Flat array [x,y,r, x,y,r, ...]
    pub fn pegs_flat(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.pegs.len() * 3);
        for p in &self.pegs {
            out.push(p.x);
            out.push(p.y);
            out.push(p.r);
        }
        out
    }

    /// Flat array [x,y,r, x,y,r, ...]
    pub fn balls_flat(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.balls.len() * 3);
        for b in &self.balls {
            out.push(b.x);
            out.push(b.y);
            out.push(b.r);
        }
        out
    }

    fn generate_pegs(&mut self) {
        self.pegs.clear();

        let rows = 11;
        let spacing_x = self.width / 12.0;
        let spacing_y = (self.height * 0.65) / rows as f32;

        for row in 0..rows {
            let y = 80.0 + row as f32 * spacing_y;
            let offset = if row % 2 == 0 { 0.0 } else { spacing_x * 0.5 };

            let cols = 11;
            for col in 0..cols {
                let x = 40.0 + col as f32 * spacing_x + offset;
                if x > 20.0 && x < self.width - 20.0 && y < self.height - 70.0 {
                    self.pegs.push(Peg {
                        x,
                        y,
                        r: PEG_RADIUS,
                    });
                }
            }
        }
    }

    fn rand_u32(&mut self) -> u32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.rng_state >> 32) as u32
    }

    fn rand_f32(&mut self) -> f32 {
        (self.rand_u32() as f32) / (u32::MAX as f32)
    }
}

fn collide_ball_with_peg(ball: &mut Ball, peg: Peg) {
    let dx = ball.x - peg.x;
    let dy = ball.y - peg.y;
    let min_dist = ball.r + peg.r;
    let dist2 = dx * dx + dy * dy;

    if dist2 >= min_dist * min_dist || dist2 == 0.0 {
        return;
    }

    let dist = dist2.sqrt();
    let nx = dx / dist;
    let ny = dy / dist;

    let overlap = min_dist - dist;
    ball.x += nx * overlap;
    ball.y += ny * overlap;

    let v_dot_n = ball.vx * nx + ball.vy * ny;
    if v_dot_n < 0.0 {
        ball.vx -= (1.0 + BOUNCE) * v_dot_n * nx;
        ball.vy -= (1.0 + BOUNCE) * v_dot_n * ny;
    }

    let tangent_x = -ny;
    let tangent_y = nx;
    let tangent_speed = ball.vx * tangent_x + ball.vy * tangent_y;
    ball.vx -= tangent_speed * 0.04 * tangent_x;
    ball.vy -= tangent_speed * 0.04 * tangent_y;
}

fn build_bin_scores(bin_count: usize) -> Vec<u32> {
    let center = (bin_count as f32 - 1.0) * 0.5;
    let mut scores = Vec::with_capacity(bin_count);
    for i in 0..bin_count {
        let d = (i as f32 - center).abs();
        let max_d = center.max(1.0);
        let t = 1.0 - d / max_d;
        let score = (10.0 + t * 90.0).round() as u32;
        scores.push(score);
    }
    scores
}
