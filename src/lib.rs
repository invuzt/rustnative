use macroquad::prelude::*;

#[macroquad::main("Odfiz")]
async fn main() {
    loop {
        // Kita pakai warna Ungu (PURPLE) kali ini
        clear_background(PURPLE);

        draw_text("ODFIZ RUST", 40.0, 100.0, 60.0, WHITE);
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 40.0, YELLOW);

        next_frame().await
    }
}
