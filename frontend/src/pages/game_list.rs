use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonVariant};
use leptos_shadcn_card::{Card, CardContent};
use leptos_shadcn_input::Input;
use leptos_shadcn_skeleton::Skeleton;

use crate::api;
use crate::components::game_card::GameCard;
use crate::mock_data;

#[component]
pub fn GameList() -> impl IntoView {
    let (search, set_search) = signal(String::new());
    let (page, set_page) = signal(1u64);

    let games = LocalResource::new(move || {
        let p = page.get();
        let s = search.get();
        async move {
            let query = if s.is_empty() { None } else { Some(s) };
            api::fetch_games(p, query.as_deref(), None, None).await
        }
    });

    let on_search = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        set_search.set(value);
        set_page.set(1);
    };

    view! {
        <div class="mx-auto max-w-7xl px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">"Games"</h1>

            // Search bar
            <div class="mb-8 max-w-md">
                <Input
                    placeholder="Search games..."
                    on:input=on_search
                />
            </div>

            // Game grid
            <Suspense fallback=move || view! { <GameListSkeleton /> }>
                {move || Suspend::new(async move {
                    match games.await {
                        Ok(list) => {
                            let total_pages = (list.total + list.per_page - 1) / list.per_page;
                            let current_page = list.page;
                            let cards = list.games.into_iter().map(|game| {
                                view! { <GameCard game=game /> }
                            }).collect::<Vec<_>>();

                            if cards.is_empty() {
                                view! {
                                    <Card>
                                        <CardContent>
                                            <p class="py-12 text-center text-muted-foreground">
                                                "No games found. Try a different search."
                                            </p>
                                        </CardContent>
                                    </Card>
                                }.into_any()
                            } else {
                                view! {
                                    <div>
                                        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                                            {cards}
                                        </div>

                                        // Pagination
                                        <div class="mt-8 flex items-center justify-center gap-2">
                                            <Button
                                                variant=ButtonVariant::Outline
                                                disabled=Signal::derive(move || current_page <= 1)
                                                on_click=Callback::new(move |_| set_page.set(page.get().saturating_sub(1)))
                                            >
                                                "← Previous"
                                            </Button>
                                            <span class="px-4 text-sm text-muted-foreground">
                                                {format!("Page {} of {}", current_page, total_pages)}
                                            </span>
                                            <Button
                                                variant=ButtonVariant::Outline
                                                disabled=Signal::derive(move || current_page >= total_pages)
                                                on_click=Callback::new(move |_| set_page.set(page.get() + 1))
                                            >
                                                "Next →"
                                            </Button>
                                        </div>
                                    </div>
                                }.into_any()
                            }
                        },
                        Err(_) => {
                            // Fallback to mock data when API is unavailable
                            let mock_games = mock_data::get_mock_games();
                            let cards = mock_games.into_iter().map(|game| {
                                view! { <GameCard game=game /> }
                            }).collect::<Vec<_>>();

                            view! {
                                <div>
                                    <div class="mb-4 p-3 bg-accent/10 border border-accent rounded-lg text-sm text-accent">
                                        "📝 Showing demo games (backend unavailable)"
                                    </div>
                                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                                        {cards}
                                    </div>
                                </div>
                            }.into_any()
                        },
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn GameListSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
            {(0..10).map(|_| view! {
                <div class="space-y-3">
                    <Skeleton class="h-[260px] w-full rounded-lg" />
                    <Skeleton class="h-4 w-3/4" />
                    <Skeleton class="h-3 w-1/2" />
                </div>
            }).collect::<Vec<_>>()}
        </div>
    }
}
