use sea_orm::*;
use chrono::Utc;
use crate::models::{game, review, user};

pub async fn seed_mock_data(db: &DbConn) -> Result<(), DbErr> {
    // Create mock users
    let user1 = user::ActiveModel {
        id: Set(1),
        sub: Set("user1-zitadel-subject-id".to_string()),
        username: Set("alexgamer".to_string()),
        email: Set("alex@example.com".to_string()),
        display_name: Set(Some("Alex Gamer".to_string())),
        avatar_url: Set(Some("https://api.dicebear.com/7.x/avataaars/svg?seed=Alex".to_string())),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    let user2 = user::ActiveModel {
        id: Set(2),
        sub: Set("user2-zitadel-subject-id".to_string()),
        username: Set("luna_midnight".to_string()),
        email: Set("luna@example.com".to_string()),
        display_name: Set(Some("Luna".to_string())),
        avatar_url: Set(Some("https://api.dicebear.com/7.x/avataaars/svg?seed=Luna".to_string())),
        created_at: Set(Utc::now().naive_utc()),
        updated_at: Set(Utc::now().naive_utc()),
    };

    user1.insert(db).await.ok();
    user2.insert(db).await.ok();

    // Create mock games
    let games = vec![
        ("Baldur's Gate 3", "baldurs-gate-3", "An epic fantasy RPG with mind-bending storytelling and endless possibilities.", "RPG", "PC, PlayStation"),
        ("Elden Ring", "elden-ring", "A challenging action RPG set in a vast open world with breathtaking vistas.", "Action RPG", "PC, PlayStation, Xbox"),
        ("Starfield", "starfield", "Embark on an epic space exploration adventure across hundreds of star systems.", "Action RPG", "PC, Xbox"),
        ("Hollow Knight", "hollow-knight", "A hauntingly beautiful metroidvania about a small knight exploring a mysterious kingdom.", "Metroidvania", "PC, Nintendo Switch"),
        ("The Legend of Zelda: Tears of the Kingdom", "zelda-totk", "Link's latest adventure in a sprawling kingdom filled with secrets and puzzles.", "Action Adventure", "Nintendo Switch"),
        ("Cyberpunk 2077", "cyberpunk-2077", "An immersive open-world RPG set in a dystopian future city.", "Action RPG", "PC, PlayStation, Xbox"),
        ("Hades", "hades", "A roguelike dungeon crawler with incredible art, music, and storytelling.", "Roguelike", "PC, Nintendo Switch"),
        ("Stray", "stray", "Play as a cat in a neon-lit cyberpunk city and uncover its mysteries.", "Adventure", "PC, PlayStation"),
    ];

    for (i, (title, slug, desc, genre, platform)) in games.iter().enumerate() {
        let game = game::ActiveModel {
            id: Set((i as i32) + 1),
            title: Set(ToString::to_string(*title)),
            slug: Set(ToString::to_string(*slug)),
            description: Set(Some(ToString::to_string(*desc))),
            genre: Set(Some(ToString::to_string(*genre))),
            platform: Set(Some(ToString::to_string(*platform))),
            release_date: Set(None),
            cover_image_url: Set(None),
            rawg_id: Set(None),
            avg_rating: Set(None),
            review_count: Set(0),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        game.insert(db).await.ok();
    }

    // Create mock reviews
    let reviews = vec![
        (1, 1, 5, Some("Masterpiece!"), "This game is absolutely incredible. Every moment is magical."),
        (2, 1, 4, Some("Fantastic Adventure"), "Amazing gameplay and story, though it could use some optimization."),
        (1, 2, 5, Some("Gaming Perfection"), "Elden Ring redefined what open-world RPGs can be. Stunning!"),
        (2, 2, 5, Some("Challenging & Rewarding"), "The difficulty is punishing but fair. Every victory feels earned."),
        (1, 3, 4, Some("Space Exploration Done Right"), "A massive game with incredible ambition. Worth the journey."),
        (2, 4, 5, Some("Indie Masterpiece"), "Small indie game with massive heart. Beautiful pixel art and music."),
        (1, 5, 5, Some("A Link to Excellence"), "Link's adventure continues to amaze. Creative puzzles and exploration."),
        (2, 6, 4, Some("Much Improved"), "The updates have made this game so much better. Great RPG experience now."),
        (1, 7, 5, Some("Roguelike Perfection"), "Hades proves roguelikes can have amazing story AND great gameplay."),
        (2, 8, 5, Some("A Unique Experience"), "Playing as a cat in this world is delightful. Unexpected gem."),
    ];

    for (user_id, game_id, rating, title, body) in reviews.iter() {
        let review = review::ActiveModel {
            id: NotSet,
            user_id: Set(*user_id),
            game_id: Set(*game_id),
            rating: Set(*rating),
            title: Set(title.map(|s| ToString::to_string(s))),
            body: Set(ToString::to_string(*body)),
            created_at: Set(Utc::now().naive_utc()),
            updated_at: Set(Utc::now().naive_utc()),
        };
        review.insert(db).await.ok();
    }

    println!("✓ Mock data seeded successfully!");
    Ok(())
}
