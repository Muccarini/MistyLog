use leptos::prelude::*;
use leptos_shadcn_button::{Button, ButtonVariant};
use leptos_shadcn_card::{Card, CardContent, CardHeader, CardTitle};

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl px-4 py-24 text-center">
            <Card class="mx-auto max-w-xl">
                <CardHeader>
                    <p class="text-6xl font-bold text-accent">"404"</p>
                    <CardTitle>"Page not found"</CardTitle>
                </CardHeader>
                <CardContent>
                    <p class="text-muted-foreground mb-6">
                        "This page doesn't exist. Maybe the game hasn't been released yet?"
                    </p>
                    <div class="flex items-center justify-center gap-3">
                        <a href="/">
                            <Button>"Back to Home"</Button>
                        </a>
                        <a href="/games">
                            <Button variant=ButtonVariant::Outline>"Browse Games"</Button>
                        </a>
                    </div>
                </CardContent>
            </Card>
        </div>
    }
}
