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
            <Card class="overflow-hidden transition-all duration-200 group-hover:ring-2 group-hover:ring-accent/50 group-hover:shadow-lg h-full flex flex-col">
                // Cover image - smaller
                {match game.cover_image_url {
                    Some(url) => view! {
                        <div class="aspect-[2/3] overflow-hidden flex-shrink-0">
                            <img
                                src=url
                                alt=game.title.clone()
                                class="h-full w-full object-cover transition-transform duration-300 group-hover:scale-105"
                            />
                        </div>
                    }.into_any(),
                    None => view! {
                        <div class="aspect-[2/3] bg-muted flex items-center justify-center flex-shrink-0">
                            <span class="text-2xl">"🎮"</span>
                        </div>
                    }.into_any(),
                }}

                // Text content with more spacing
                <CardContent class="p-2 flex flex-col gap-2 flex-1">
                    <h3 class="font-semibold text-xs truncate group-hover:text-accent transition-colors">
                        {game.title}
                    </h3>
                    <div class="flex items-center justify-between gap-1 text-[10px]">
                        {if avg > 0.0 {
                            view! {
                                <div class="flex items-center gap-0.5 flex-1">
                                    <RatingStars rating=avg size="sm" />
                                    <span class="text-star font-medium">
                                        {format!("{:.1}", avg)}
                                    </span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <span class="text-muted-foreground flex-1">"No ratings"</span>
                            }.into_any()
                        }}
                        {game.genre.map(|g| view! {
                            <Badge class="text-[9px] px-1 py-0 flex-shrink-0">{g}</Badge>
                        })}
                    </div>
                </CardContent>
            </Card>
        </a>
    }
}
