use leptos::prelude::*;

/// Displays star icons for a rating value (0.0–10.0 scale, shown as 5 stars).
/// Each full star = 2 rating points.
#[component]
pub fn RatingStars(
    /// Rating on a 0–10 scale
    rating: f64,
    /// Size: "sm" | "md" (default: "md")
    #[prop(optional, into)]
    size: String,
) -> impl IntoView {
    let star_count = 5;
    let half = rating / 2.0; // convert 0-10 to 0-5

    let sz = if size == "sm" { "w-3 h-3" } else { "w-4 h-4" };

    let stars = (0..star_count)
        .map(|i| {
            let fill = if (i as f64) < half.floor() {
                "text-star"           // full star
            } else if (i as f64) < half.ceil() && half.fract() >= 0.25 {
                "text-star opacity-50" // half star (approximated)
            } else {
                "text-muted-foreground/30" // empty star
            };

            view! {
                <svg
                    class=format!("{sz} {fill}")
                    viewBox="0 0 24 24"
                    fill="currentColor"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                </svg>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="flex items-center gap-0.5">
            {stars}
        </div>
    }
}
