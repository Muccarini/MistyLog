use leptos::prelude::*;
use leptos_shadcn_avatar::{Avatar, AvatarFallback};
use leptos_shadcn_card::{Card, CardContent};
use leptos_shadcn_separator::Separator;

use crate::api::Review;
use crate::components::rating_stars::RatingStars;

#[component]
pub fn ReviewCard(review: Review) -> impl IntoView {
    let initials = review.display_name.as_ref()
        .unwrap_or(&review.username)
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect::<String>()
        .to_uppercase();

    let display = review.display_name.unwrap_or(review.username.clone());
    let date = review.created_at.get(..10).unwrap_or(&review.created_at).to_string();

    view! {
        <Card>
            <CardContent class="pt-6">
                // Header: avatar + name + rating
                <div class="flex items-start gap-3 mb-3">
                    <Avatar class="h-8 w-8 text-xs">
                        <AvatarFallback>{initials}</AvatarFallback>
                    </Avatar>
                    <div class="flex-1 min-w-0">
                        <div class="flex items-center justify-between">
                            <span class="font-medium text-sm">{display}</span>
                            <span class="text-xs text-muted-foreground">{date}</span>
                        </div>
                        <RatingStars rating=review.rating as f64 size="sm" />
                    </div>
                </div>

                // Title
                {review.title.map(|t| view! {
                    <h4 class="font-semibold mb-1">{t}</h4>
                })}

                <Separator class="my-3" />

                // Body
                <p class="text-sm text-muted-foreground leading-relaxed">
                    {review.body}
                </p>
            </CardContent>
        </Card>
    }
}
