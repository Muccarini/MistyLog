use leptos::prelude::*;
use leptos_shadcn_badge::Badge;
use leptos_shadcn_card::{Card, CardContent};

use crate::api::Game;
use crate::components::rating_stars::RatingStars;

#[component]
pub fn GameCard(game: Game) -> impl IntoView {
    let avg = game.avg_rating.unwrap_or(0.0);
    let href = format!("/games/{}", game.id);

    view! {
        <a href=href class="group block">
            <Card class="overflow-hidden transition-all duration-200 group-hover:ring-2 group-hover:ring-accent/50 group-hover:shadow-lg">
                // Cover image
                {match game.cover_image_url {
                    Some(url) => view! {
                        <div class="aspect-[3/4] overflow-hidden">
                            <img
                                src=url
                                alt=game.title.clone()
                                class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                            />
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="aspect-[3/4] bg-muted flex items-center justify-center">
                            <span class="text-4xl">"🎮"</span>
                        </div>
                    }.into_any(),
                }}

                <CardContent class="p-3">
                    <h3 class="font-semibold text-sm truncate mb-1 group-hover:text-accent transition-colors">
                        {game.title}
                    </h3>
                    <div class="flex items-center justify-between">
                        {if avg > 0.0 {
                            view! {
                                <div class="flex items-center gap-1">
                                    <RatingStars rating=avg size="sm" />
                                    <span class="text-xs text-star font-medium">
                                        {format!("{:.1}", avg)}
                                    </span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <span class="text-xs text-muted-foreground">"No ratings"</span>
                            }.into_any()
                        }}
                        {game.genre.map(|g| view! {
                            <Badge class="text-[10px] px-1.5 py-0">{g}</Badge>
                        })}
                    </div>
                </CardContent>
            </Card>
        </a>
    }
}
