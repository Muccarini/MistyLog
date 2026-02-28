use leptos::prelude::*;
use leptos_shadcn_avatar::{Avatar, AvatarFallback};
use leptos_shadcn_button::{Button, ButtonVariant};

use crate::api::{self, User};

#[component]
pub fn Navbar() -> impl IntoView {
    let user_resource = expect_context::<LocalResource<Option<User>>>();

    let auth_section = move || {
        Suspend::new(async move {
            let user = user_resource.await;
            match user {
                Some(u) => {
                    let initials = u.display_name.as_ref()
                        .unwrap_or(&u.username)
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .take(2)
                        .collect::<String>()
                        .to_uppercase();
                    let display = u.display_name.unwrap_or(u.username);

                    view! {
                        <div class="flex items-center gap-3">
                            <a href="/profile" class="flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground transition-colors">
                                <Avatar class="h-7 w-7 text-[10px]">
                                    <AvatarFallback>{initials}</AvatarFallback>
                                </Avatar>
                                <span class="hidden sm:inline">{display}</span>
                            </a>
                            <Button
                                variant=ButtonVariant::Ghost
                                on_click=Callback::new(move |_| {
                                    leptos::task::spawn_local(async {
                                        if let Ok(redirect) = api::logout().await {
                                            let _ = web_sys::window()
                                                .unwrap()
                                                .location()
                                                .set_href(&redirect);
                                        }
                                    });
                                })
                            >
                                "Log out"
                            </Button>
                        </div>
                    }.into_any()
                },
                None => view! {
                    <a href={api::login_url()}>
                        <Button>"Sign in"</Button>
                    </a>
                }.into_any(),
            }
        })
    };

    view! {
        <nav class="sticky top-0 z-50 border-b border-border bg-background/80 backdrop-blur-md">
            <div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-3">
                // Logo
                <a href="/" class="text-xl font-bold tracking-tight text-foreground">
                    <span class="text-accent">"Misty"</span>"Log"
                </a>

                // Navigation links
                <div class="hidden md:flex items-center gap-6">
                    <a href="/games" class="text-sm text-muted-foreground hover:text-foreground transition-colors">
                        "Games"
                    </a>
                </div>

                // Auth section
                <div class="flex items-center gap-4">
                    <Suspense fallback=move || view! {
                        <div class="h-8 w-20 animate-pulse rounded bg-muted"></div>
                    }>
                        {auth_section}
                    </Suspense>
                </div>
            </div>
        </nav>
    }
}
