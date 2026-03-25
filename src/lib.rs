use macroquad::prelude::*;

#[macroquad::main("Odfiz")]
async fn main() {
    loop {
        // Layar Biru agar beda dengan yang tadi hitam
        clear_background(BLUE);

        draw_text("ODFIZ RUST", 40.0, 100.0, 60.0, WHITE);
        draw_text("Status: Running!", 40.0, 160.0, 30.0, LIGHTGRAY);
        
        // Animasi lingkaran kecil biar kelihatan hidup
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 50.0, YELLOW);

        next_frame().await
    }
}
