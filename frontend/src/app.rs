use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::api::{self, User};
use crate::components::navbar::Navbar;
use crate::pages;

#[component]
pub fn App() -> impl IntoView {
    // Global auth state: fetch current user on mount
    let user: LocalResource<Option<User>> = LocalResource::new(|| api::fetch_me());

    provide_context(user);

    view! {
        <Router>
            <div class="min-h-screen flex flex-col">
                <Navbar />
                <main class="flex-1">
                    <Routes fallback=|| view! { <pages::not_found::NotFound /> }>
                        <Route path=path!("/") view=pages::home::Home />
                        <Route path=path!("/games") view=pages::game_list::GameList />
                        <Route path=path!("/games/:id") view=pages::game_detail::GameDetail />
                        <Route path=path!("/profile") view=pages::profile::Profile />
                    </Routes>
                </main>
                <footer class="border-t border-border py-6 text-center text-sm text-muted-foreground">
                    <p>"GameReview — a Letterboxd for games"</p>
                </footer>
            </div>
        </Router>
    }
}
