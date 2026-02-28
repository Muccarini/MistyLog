use leptos::prelude::*;
use leptos_shadcn_avatar::{Avatar, AvatarFallback};
use leptos_shadcn_card::{Card, CardContent, CardHeader, CardTitle};
use leptos_shadcn_separator::Separator;
use leptos_shadcn_skeleton::Skeleton;

use crate::api::User;

#[component]
pub fn Profile() -> impl IntoView {
    let user = expect_context::<LocalResource<Option<User>>>();

    view! {
        <div class="mx-auto max-w-3xl px-4 py-8">
            <Suspense fallback=move || view! { <ProfileSkeleton /> }>
                {move || Suspend::new(async move {
                    match user.await {
                        Some(u) => {
                            let initials = u.display_name.as_ref()
                                .or(Some(&u.username))
                                .map(|name| {
                                    name.chars()
                                        .filter(|c| c.is_alphanumeric())
                                        .take(2)
                                        .collect::<String>()
                                        .to_uppercase()
                                })
                                .unwrap_or_default();

                            view! {
                                <div>
                                    // Profile header
                                    <Card class="mb-8">
                                        <CardContent>
                                            <div class="flex items-center gap-6 py-4">
                                                <Avatar class="h-20 w-20 text-2xl">
                                                    <AvatarFallback>{initials}</AvatarFallback>
                                                </Avatar>
                                                <div>
                                                    <h1 class="text-2xl font-bold">
                                                        {u.display_name.unwrap_or(u.username.clone())}
                                                    </h1>
                                                    <p class="text-muted-foreground">{format!("@{}", u.username)}</p>
                                                    <p class="text-sm text-muted-foreground mt-1">
                                                        {format!("Member since {}", &u.created_at[..10])}
                                                    </p>
                                                </div>
                                            </div>
                                        </CardContent>
                                    </Card>

                                    <Separator />

                                    // Placeholder for user's reviews
                                    <section class="mt-8">
                                        <h2 class="text-xl font-bold mb-4">"Your Reviews"</h2>
                                        <Card>
                                            <CardContent>
                                                <p class="py-8 text-center text-muted-foreground">
                                                    "Your reviews will appear here."
                                                </p>
                                            </CardContent>
                                        </Card>
                                    </section>
                                </div>
                            }.into_any()
                        },
                        None => view! {
                            <Card>
                                <CardHeader>
                                    <CardTitle>"Not Signed In"</CardTitle>
                                </CardHeader>
                                <CardContent>
                                    <p class="text-muted-foreground">
                                        "Please sign in to view your profile."
                                    </p>
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
fn ProfileSkeleton() -> impl IntoView {
    view! {
        <Card>
            <CardContent>
                <div class="flex items-center gap-6 py-4">
                    <Skeleton class="h-20 w-20 rounded-full" />
                    <div class="space-y-2">
                        <Skeleton class="h-6 w-48" />
                        <Skeleton class="h-4 w-32" />
                    </div>
                </div>
            </CardContent>
        </Card>
    }
}
