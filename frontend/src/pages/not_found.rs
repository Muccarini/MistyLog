use leptos::prelude::*;
use leptos_shadcn_button::Button;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="mx-auto max-w-7xl px-4 py-24 text-center">
            <h1 class="text-8xl font-bold text-accent mb-4">"404"</h1>
            <p class="text-xl text-muted-foreground mb-8">
                "This page doesn't exist. Maybe the game hasn't been released yet?"
            </p>
            <a href="/">
                <Button>"Back to Home"</Button>
            </a>
        </div>
    }
}
