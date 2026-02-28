use leptos::prelude::*;
use leptos_shadcn_alert::{Alert, AlertDescription, AlertTitle};
use leptos_shadcn_badge::{Badge, BadgeVariant};
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
    let (genre, set_genre) = signal(String::new());
    let (platform, set_platform) = signal(String::new());
    let (page, set_page) = signal(1u64);

    let games = LocalResource::new(move || {
        let p = page.get();
        let s = search.get();
        let g = genre.get();
        let pl = platform.get();
        async move {
            let query = if s.is_empty() { None } else { Some(s) };
            let genre_filter = if g.is_empty() { None } else { Some(g) };
            let platform_filter = if pl.is_empty() { None } else { Some(pl) };
            api::fetch_games(p, query.as_deref(), genre_filter.as_deref(), platform_filter.as_deref()).await
        }
    });

    let on_search = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        set_search.set(value);
        set_page.set(1);
    };

    let on_genre_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        set_genre.set(if value == "all" { String::new() } else { value });
        set_page.set(1);
    };

    let on_platform_change = move |ev: web_sys::Event| {
        let value = event_target_value(&ev);
        set_platform.set(if value == "all" { String::new() } else { value });
        set_page.set(1);
    };

    let clear_filters = move |_| {
        set_search.set(String::new());
        set_genre.set(String::new());
        set_platform.set(String::new());
        set_page.set(1);
    };

    let clear_search = move |_| {
        set_search.set(String::new());
        set_page.set(1);
    };

    let clear_genre = move |_| {
        set_genre.set(String::new());
        set_page.set(1);
    };

    let clear_platform = move |_| {
        set_platform.set(String::new());
        set_page.set(1);
    };

    view! {
        <div class="mx-auto max-w-7xl px-4 py-8">
            <h1 class="text-3xl font-bold mb-8">"Games"</h1>

            // Search + filters
            <div class="mb-8 grid grid-cols-1 md:grid-cols-4 gap-3 items-end">
                <div class="md:col-span-2">
                    <Input
                        placeholder="Search games..."
                        prop:value=move || search.get()
                        on:input=on_search
                    />
                </div>

                <div>
                    <select
                        class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                        prop:value=move || genre.get()
                        on:change=on_genre_change
                    >
                        <option value="all">"All genres"</option>
                        <option value="RPG">"RPG"</option>
                        <option value="Action">"Action"</option>
                        <option value="Adventure">"Adventure"</option>
                        <option value="Shooter">"Shooter"</option>
                        <option value="Strategy">"Strategy"</option>
                        <option value="Platformer">"Platformer"</option>
                    </select>
                </div>

                <div>
                    <select
                        class="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2"
                        prop:value=move || platform.get()
                        on:change=on_platform_change
                    >
                        <option value="all">"All platforms"</option>
                        <option value="PC">"PC"</option>
                        <option value="PlayStation 5">"PlayStation 5"</option>
                        <option value="Xbox Series X/S">"Xbox Series X/S"</option>
                        <option value="Nintendo Switch">"Nintendo Switch"</option>
                        <option value="PlayStation 4">"PlayStation 4"</option>
                        <option value="Xbox One">"Xbox One"</option>
                    </select>
                </div>
            </div>

            <Show when=move || !search.get().is_empty() || !genre.get().is_empty() || !platform.get().is_empty()>
                <div class="mb-6 flex flex-wrap items-center justify-between gap-3">
                    <div class="flex flex-wrap items-center gap-2">
                        <Show when=move || !search.get().is_empty()>
                            <Badge variant=BadgeVariant::Secondary class="gap-2 pr-1">
                                <span>{move || format!("Search: {}", search.get())}</span>
                                <button
                                    type="button"
                                    class="rounded-full px-1 text-xs leading-none hover:bg-muted"
                                    on:click=clear_search
                                    aria-label="Remove search filter"
                                >
                                    "×"
                                </button>
                            </Badge>
                        </Show>

                        <Show when=move || !genre.get().is_empty()>
                            <Badge variant=BadgeVariant::Secondary class="gap-2 pr-1">
                                <span>{move || format!("Genre: {}", genre.get())}</span>
                                <button
                                    type="button"
                                    class="rounded-full px-1 text-xs leading-none hover:bg-muted"
                                    on:click=clear_genre
                                    aria-label="Remove genre filter"
                                >
                                    "×"
                                </button>
                            </Badge>
                        </Show>

                        <Show when=move || !platform.get().is_empty()>
                            <Badge variant=BadgeVariant::Secondary class="gap-2 pr-1">
                                <span>{move || format!("Platform: {}", platform.get())}</span>
                                <button
                                    type="button"
                                    class="rounded-full px-1 text-xs leading-none hover:bg-muted"
                                    on:click=clear_platform
                                    aria-label="Remove platform filter"
                                >
                                    "×"
                                </button>
                            </Badge>
                        </Show>
                    </div>

                    <Button variant=ButtonVariant::Ghost on_click=Callback::new(clear_filters)>
                        "Reset filters"
                    </Button>
                </div>
            </Show>

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
                                    <Alert class="mb-4">
                                        <AlertTitle>"Demo mode"</AlertTitle>
                                        <AlertDescription>
                                            "Showing mock games because the backend is currently unavailable."
                                        </AlertDescription>
                                    </Alert>
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
