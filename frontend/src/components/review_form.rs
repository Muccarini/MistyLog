use leptos::prelude::*;
use leptos_shadcn_button::Button;
use leptos_shadcn_card::{Card, CardContent, CardHeader, CardTitle};
use leptos_shadcn_input::Input;
use leptos_shadcn_label::Label;
use leptos_shadcn_textarea::Textarea;

use crate::api;

#[component]
pub fn ReviewForm(game_id: i32) -> impl IntoView {
    let (rating, set_rating) = signal(5i16);
    let (title, set_title) = signal(String::new());
    let (body, set_body) = signal(String::new());
    let (submitting, set_submitting) = signal(false);
    let (error, set_error) = signal(Option::<String>::None);
    let (success, set_success) = signal(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        set_submitting.set(true);
        set_error.set(None);

        let r = rating.get();
        let t = title.get();
        let b = body.get();

        leptos::task::spawn_local(async move {
            let title_opt = if t.is_empty() { None } else { Some(t) };
            match api::create_review(game_id, r, title_opt, &b).await {
                Ok(()) => {
                    set_success.set(true);
                    set_submitting.set(false);
                    set_title.set(String::new());
                    set_body.set(String::new());
                    set_rating.set(5);
                },
                Err(e) => {
                    set_error.set(Some(e));
                    set_submitting.set(false);
                },
            }
        });
    };

    view! {
        <Card>
            <CardHeader>
                <CardTitle>"Write a Review"</CardTitle>
            </CardHeader>
            <CardContent>
                <Show when=move || success.get()>
                    <div class="mb-4 rounded-md bg-accent/10 border border-accent p-3 text-sm text-accent">
                        "Review submitted successfully! Refresh to see it."
                    </div>
                </Show>

                <form on:submit=on_submit class="space-y-4">
                    // Rating
                    <div>
                        <Label>"Rating (1–10)"</Label>
                        <div class="mt-2 flex items-center gap-2">
                            {(1..=10).map(|i| {
                                let active = move || rating.get() >= i;
                                view! {
                                    <button
                                        type="button"
                                        class=move || format!(
                                            "w-8 h-8 rounded-full text-sm font-medium transition-colors {}",
                                            if active() { "bg-star text-background" } else { "bg-muted text-muted-foreground hover:bg-muted/80" }
                                        )
                                        on:click=move |_| set_rating.set(i)
                                    >
                                        {i}
                                    </button>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>

                    // Title
                    <div>
                        <Label>"Title (optional)"</Label>
                        <Input
                            class="mt-1"
                            placeholder="Give your review a title..."
                            on:input=move |ev| set_title.set(event_target_value(&ev))
                        />
                    </div>

                    // Body
                    <div>
                        <Label>"Review"</Label>
                        <Textarea
                            class="mt-1"
                            placeholder="What did you think of this game?"
                            on:input=move |ev| set_body.set(event_target_value(&ev))
                        />
                    </div>

                    // Error
                    {move || error.get().map(|e| view! {
                        <p class="text-sm text-destructive">{e}</p>
                    })}

                    // Submit
                    <Button
                        disabled=Signal::derive(move || submitting.get() || body.get().is_empty())
                    >
                        {move || if submitting.get() { "Submitting..." } else { "Submit Review" }}
                    </Button>
                </form>
            </CardContent>
        </Card>
    }
}
