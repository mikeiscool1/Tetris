use crate::game::Game;
use macroquad::prelude::*;

pub fn draw_game(game: &Game) {
    clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

    let window_w = screen_width();
    let window_h = screen_height();

    // Automatically adjust tile size and offset to fit the game area
    let tile_size = (window_w / game.width as f32).min(window_h / game.height as f32);
    let offset_x = (window_w - tile_size * game.width as f32) / 2.0;

    // Draw grid
    for x in 0..game.width {
        for y in 0..game.height {
            let rect = Rect::new(
                offset_x + x as f32 * tile_size,
                y as f32 * tile_size,
                tile_size,
                tile_size,
            );
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, GRAY);
        }
    }

    // Draw shadow
    if let Some(shadow) = game.get_active_shadow() {
        for &pos in &shadow {
            let rect = Rect::new(
                offset_x + pos.x as f32 * tile_size + 2.0,
                pos.y as f32 * tile_size + 2.0,
                tile_size - 4.0,
                tile_size - 4.0,
            );
            
            let color = game.blocks[game.active_block_i.unwrap()].color;
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, color.with_alpha(0.5));

            let shine_color = Color::new((color.r + 0.3).min(1.0), (color.g + 0.3).min(1.0), (color.b + 0.3).min(1.0), 0.5);

            // Top edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + 2.0, 3.0, shine_color);
            // Left edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + 2.0, rect.y + rect.h - 2.0, 3.0, shine_color);

            let shadow_color = Color::new((color.r - 0.3).max(0.0), (color.g - 0.3).max(0.0), (color.b - 0.3).max(0.0), 0.5);
            // Bottom edge
            draw_line(rect.x + 2.0, rect.y + rect.h - 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
            // Right edge
            draw_line(rect.x + rect.w - 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
        }
    }

    // Draw blocks
    for block in &game.blocks {
        for &pos in &block.structure {
            // draw inner rectangle
            let rect = Rect::new(
                offset_x + pos.x as f32 * tile_size + 1.0,
                pos.y as f32 * tile_size + 1.0,
                tile_size - 2.0,
                tile_size - 2.0,
            );
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, block.color);

            let shine_color = Color::new((block.color.r + 0.3).min(1.0), (block.color.g + 0.3).min(1.0), (block.color.b + 0.3).min(1.0), 1.0);

            // Top edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + 2.0, 3.0, shine_color);
            // Left edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + 2.0, rect.y + rect.h - 2.0, 3.0, shine_color);

            let shadow_color = Color::new((block.color.r - 0.3).max(0.0), (block.color.g - 0.3).max(0.0), (block.color.b - 0.3).max(0.0), 1.0);
            // Bottom edge
            draw_line(rect.x + 2.0, rect.y + rect.h - 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
            // Right edge
            draw_line(rect.x + rect.w - 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
        }
    }

    if window_w > 400.0 {
        // Draw stats on the left
        let text_size = window_w / 20.0;
        let text_offset_x = offset_x / 2.0 - text_size * 2.0;
        let text_offset_y = text_size * 2.0;
        draw_text(&format!("Level: {}", game.level), text_offset_x, text_offset_y, text_size, WHITE);
        draw_text(&format!("Score: {}", game.score), text_offset_x, text_offset_y + text_size, text_size, WHITE);

        // Draw next block preview on the right
        let preview_size = tile_size * 4.0;
        let preview_offset_x = offset_x + tile_size * game.width as f32 + (offset_x / 2.0 - preview_size / 2.0);
        let preview_offset_y = text_offset_y;
        draw_text("Next:", preview_offset_x, preview_offset_y, text_size, WHITE);

        for &pos in &game.next_block.structure {
            let rect = Rect::new(
                preview_offset_x + pos.x as f32 * tile_size + 1.0,
                preview_offset_y + text_size + pos.y as f32 * tile_size + 1.0,
                tile_size - 2.0,
                tile_size - 2.0,
            );
            
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, game.next_block.color);

            let shine_color = Color::new((game.next_block.color.r + 0.3).min(1.0), (game.next_block.color.g + 0.3).min(1.0), (game.next_block.color.b + 0.3).min(1.0), 0.5);

            // Top edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + 2.0, 3.0, shine_color);
            // Left edge
            draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + 2.0, rect.y + rect.h - 2.0, 3.0, shine_color);

            let shadow_color = Color::new((game.next_block.color.r - 0.3).max(0.0), (game.next_block.color.g - 0.3).max(0.0), (game.next_block.color.b - 0.3).max(0.0), 0.5);
            // Bottom edge
            draw_line(rect.x + 2.0, rect.y + rect.h - 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
            // Right edge
            draw_line(rect.x + rect.w - 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
        }

        //Draw hold block right below next block preview
        let hold_offset_y = preview_offset_y + text_size + preview_size;
        draw_text("Hold:", preview_offset_x, hold_offset_y, text_size, WHITE);
        if let Some(hold_block) = &game.hold_block {
            for &pos in &hold_block.structure {
                let rect = Rect::new(
                    preview_offset_x + pos.x as f32 * tile_size + 1.0,
                    hold_offset_y + text_size + pos.y as f32 * tile_size + 1.0,
                    tile_size - 2.0,
                    tile_size - 2.0,
                );
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, hold_block.color);

                let shine_color = Color::new((hold_block.color.r + 0.3).min(1.0), (hold_block.color.g + 0.3).min(1.0), (hold_block.color.b + 0.3).min(1.0), 0.5);

                // Top edge
                draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + 2.0, 3.0, shine_color);
                // Left edge
                draw_line(rect.x + 2.0, rect.y + 2.0, rect.x + 2.0, rect.y + rect.h - 2.0, 3.0, shine_color);

                let shadow_color = Color::new((hold_block.color.r - 0.3).max(0.0), (hold_block.color.g - 0.3).max(0.0), (hold_block.color.b - 0.3).max(0.0), 0.5);
                // Bottom edge
                draw_line(rect.x + 2.0, rect.y + rect.h - 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
                // Right edge
                draw_line(rect.x + rect.w - 2.0, rect.y + 2.0, rect.x + rect.w - 2.0, rect.y + rect.h - 2.0, 3.0, shadow_color);
                }
        }
    }
}