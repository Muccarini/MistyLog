use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonVariant};
use leptos_shadcn_card::{Card, CardContent, CardHeader, CardTitle};
use leptos_shadcn_separator::Separator;
use leptos_shadcn_skeleton::Skeleton;

use crate::api;
use crate::components::game_card::GameCard;
use crate::mock_data;

#[component]
pub fn Home() -> impl IntoView {
    let games = LocalResource::new(|| api::fetch_games(1, None, None, None));

    view! {
        <div class="mx-auto max-w-7xl px-4 py-8">
            // Hero
            <section class="py-16 text-center">
                <h1 class="text-5xl font-bold tracking-tight">
                    <span class="text-accent">"Track"</span>
                    " games you've played."
                </h1>
                <p class="mt-4 text-lg text-muted-foreground max-w-2xl mx-auto">
                    "Rate, review, and discover games. Build your gaming diary and share your thoughts with the community."
                </p>
                <div class="mt-8 flex items-center justify-center gap-4">
                    <a href="/games">
                        <Button variant=ButtonVariant::Default>
                            "Browse Games"
                        </Button>
                    </a>
                    <a href={api::login_url()}>
                        <Button variant=ButtonVariant::Outline>
                            "Get Started"
                        </Button>
                    </a>
                </div>
            </section>

            <Separator />

            // Trending games
            <section class="py-12">
                <div class="flex items-center justify-between mb-8">
                    <h2 class="text-2xl font-bold">"Popular Games"</h2>
                    <a href="/games" class="text-sm text-accent hover:underline">"View all →"</a>
                </div>

                <Suspense fallback=move || view! { <GameGridSkeleton /> }>
                    {move || Suspend::new(async move {
                        match games.await {
                            Ok(list) => {
                                let cards = list.games.into_iter().take(8).map(|game| {
                                    view! { <GameCard game=game /> }
                                }).collect::<Vec<_>>();
                                view! {
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-6">
                                        {cards}
                                    </div>
                                }.into_any()
                            },
                            Err(_) => {
                                // Fallback to mock data
                                let cards = mock_data::get_mock_games().into_iter().take(8).map(|game| {
                                    view! { <GameCard game=game /> }
                                }).collect::<Vec<_>>();
                                view! {
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-6">
                                        {cards}
                                    </div>
                                }.into_any()
                            },
                        }
                    })}
                </Suspense>
            </section>

            <Separator />

            // Features section
            <section class="py-12">
                <h2 class="text-2xl font-bold mb-8 text-center">"Your Gaming Journal"</h2>
                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <Card>
                        <CardHeader>
                            <CardTitle>"📝 Review"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-muted-foreground">
                                "Share your thoughts on every game you play. Rate from 1–10 and write detailed reviews."
                            </p>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>"🎮 Discover"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-muted-foreground">
                                "Browse thousands of games powered by the RAWG database. Find your next favourite title."
                            </p>
                        </CardContent>
                    </Card>
                    <Card>
                        <CardHeader>
                            <CardTitle>"⭐ Track"</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <p class="text-muted-foreground">
                                "Build your personal gaming profile. Keep a record of everything you've played."
                            </p>
                        </CardContent>
                    </Card>
                </div>
            </section>
        </div>
    }
}

#[component]
fn GameGridSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-6">
            {(0..8).map(|_| view! {
                <div class="space-y-3">
                    <Skeleton class="h-[260px] w-full rounded-lg" />
                    <Skeleton class="h-4 w-3/4" />
                    <Skeleton class="h-3 w-1/2" />
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
