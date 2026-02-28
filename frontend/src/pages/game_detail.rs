use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_shadcn_badge::Badge;
use leptos_shadcn_card::{Card, CardContent, CardHeader, CardTitle};
use leptos_shadcn_separator::Separator;
use leptos_shadcn_skeleton::Skeleton;

use crate::api::{self, User};
use crate::components::rating_stars::RatingStars;
use crate::components::review_card::ReviewCard;
use crate::components::review_form::ReviewForm;

#[component]
pub fn GameDetail() -> impl IntoView {
    let params = use_params_map();
    let game_id = move || {
        params.with(|p| {
            p.get("id")
                .and_then(|id| id.parse::<i32>().ok())
                .unwrap_or(0)
        })
    };

    let game = LocalResource::new(move || {
        let id = game_id();
        async move { api::fetch_game(id).await }
    });

    let reviews = LocalResource::new(move || {
        let id = game_id();
        async move { api::fetch_reviews(id).await }
    });

    let user = expect_context::<LocalResource<Option<User>>>();

    view! {
        <div class="mx-auto max-w-7xl px-4 py-8">
            <Suspense fallback=move || view! { <GameDetailSkeleton /> }>
                {move || Suspend::new(async move {
                    match game.await {
                        Ok(g) => {
                            let avg = g.avg_rating.unwrap_or(0.0);
                            let title = g.title.clone();
                            let description = g.description.clone().unwrap_or_default();
                            let cover = g.cover_image_url.clone();
                            let genre = g.genre.clone();
                            let platform = g.platform.clone();
                            let release = g.release_date.clone();
                            let count = g.review_count;
                            let gid = g.id;

                            view! {
                                <div class="grid grid-cols-1 md:grid-cols-[300px_1fr] gap-8">
                                    // Cover image
                                    <div class="flex-shrink-0">
                                        {match cover {
                                            Some(url) => view! {
                                                <img
                                                    src=url
                                                    alt=title.clone()
                                                    class="w-full rounded-lg shadow-lg"
                                                />
                                            }.into_any(),
                                            None => view! {
                                                <div class="w-full aspect-[3/4] rounded-lg bg-muted flex items-center justify-center">
                                                    <span class="text-4xl">"🎮"</span>
                                                </div>
                                            }.into_any(),
                                        }}
                                    </div>

                                    // Game info
                                    <div>
                                        <h1 class="text-4xl font-bold mb-2">{title}</h1>

                                        <div class="flex flex-wrap items-center gap-3 mb-4">
                                            {genre.map(|g| view! {
                                                <Badge>{g}</Badge>
                                            })}
                                            {platform.map(|p| view! {
                                                <Badge>{p}</Badge>
                                            })}
                                            {release.map(|r| view! {
                                                <span class="text-sm text-muted-foreground">{r}</span>
                                            })}
                                        </div>

                                        // Rating summary
                                        <Card class="mb-6">
                                            <CardContent>
                                                <div class="flex items-center gap-4 py-2">
                                                    <RatingStars rating=avg />
                                                    <span class="text-2xl font-bold text-star">
                                                        {format!("{:.1}", avg)}
                                                    </span>
                                                    <span class="text-sm text-muted-foreground">
                                                        {format!("{} review{}", count, if count == 1 { "" } else { "s" })}
                                                    </span>
                                                </div>
                                            </CardContent>
                                        </Card>

                                        {if !description.is_empty() {
                                            view! {
                                                <p class="text-muted-foreground leading-relaxed mb-6">
                                                    {description}
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! {}.into_any()
                                        }}

                                        <Separator />

                                        // Reviews section
                                        <section class="mt-8">
                                            <h2 class="text-2xl font-bold mb-6">"Reviews"</h2>

                                            // Review form (only if logged in)
                                            <Suspense fallback=|| ()>
                                                {move || Suspend::new(async move {
                                                    let u = user.await;
                                                    match u {
                                                        Some(_) => view! {
                                                            <div class="mb-8">
                                                                <ReviewForm game_id=gid />
                                                            </div>
                                                        }.into_any(),
                                                        None => view! {
                                                            <Card class="mb-8">
                                                                <CardContent>
                                                                    <p class="py-4 text-center text-muted-foreground">
                                                                        <a href={api::login_url()} class="text-accent hover:underline">
                                                                            "Sign in"
                                                                        </a>
                                                                        " to write a review."
                                                                    </p>
                                                                </CardContent>
                                                            </Card>
                                                        }.into_any(),
                                                    }
                                                })}
                                            </Suspense>

                                            // Review list
                                            <Suspense fallback=move || view! {
                                                <div class="space-y-4">
                                                    {(0..3).map(|_| view! {
                                                        <Skeleton class="h-32 w-full rounded-lg" />
                                                    }).collect::<Vec<_>>()}
                                                </div>
                                            }>
                                                {move || Suspend::new(async move {
                                                    match reviews.await {
                                                        Ok(list) => {
                                                            if list.is_empty() {
                                                                view! {
                                                                    <p class="text-muted-foreground text-center py-8">
                                                                        "No reviews yet. Be the first to review!"
                                                                    </p>
                                                                }.into_any()
                                                            } else {
                                                                let cards = list.into_iter().map(|review| {
                                                                    view! { <ReviewCard review=review /> }
                                                                }).collect::<Vec<_>>();
                                                                view! {
                                                                    <div class="space-y-4">{cards}</div>
                                                                }.into_any()
                                                            }
                                                        },
                                                        Err(_) => view! {
                                                            <p class="text-destructive text-center py-4">
                                                                "Failed to load reviews."
                                                            </p>
                                                        }.into_any(),
                                                    }
                                                })}
                                            </Suspense>
                                        </section>
                                    </div>
                                </div>
                            }.into_any()
                        },
                        Err(e) => view! {
                            <Card>
                                <CardHeader>
                                    <CardTitle>"Error"</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-destructive">{format!("Failed to load game: {e}")}</p>
                                </CardContent>
                            </Card>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn GameDetailSkeleton() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-[300px_1fr] gap-8">
            <Skeleton class="h-[400px] w-full rounded-lg" />
            <div class="space-y-4">
                <Skeleton class="h-10 w-2/3" />
                <Skeleton class="h-6 w-1/3" />
                <Skeleton class="h-20 w-full" />
                <Skeleton class="h-32 w-full" />
            </div>
        </div>
    }
}
