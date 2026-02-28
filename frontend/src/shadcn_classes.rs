//! Tailwind CSS class safelist for leptos-shadcn component styles.
//! These string literals ensure Tailwind's JIT compiler generates all
//! utility classes used by the shadcn component crates (whose sources
//! live in the cargo registry and are invisible to the content scanner).
#![allow(dead_code)]

const _BUTTON: &[&str] = &[
    "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium",
    "ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2",
    "focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
    "bg-primary text-primary-foreground hover:bg-primary/90",
    "bg-destructive text-destructive-foreground hover:bg-destructive/90",
    "border border-input bg-background hover:bg-accent hover:text-accent-foreground",
    "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    "hover:bg-accent hover:text-accent-foreground",
    "text-primary underline-offset-4 hover:underline",
    "h-10 px-4 py-2", "h-9 rounded-md px-3", "h-11 rounded-md px-8", "h-10 w-10",
    "min-h-[44px] min-w-[44px]",
    "opacity-50 cursor-not-allowed",
    "mr-2 h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent",
    "h-9 md:h-10 px-3 md:px-4 py-2",
    "h-10 md:h-11 px-4 md:px-8 py-2",
    "h-8 md:h-9 px-2 md:px-3 py-1 md:py-2",
];

const _CARD: &[&str] = &[
    "rounded-lg border bg-card text-card-foreground shadow-sm",
    "flex flex-col space-y-1.5 p-6",
    "text-2xl font-semibold leading-none tracking-tight",
    "text-sm text-muted-foreground",
    "p-6 pt-0",
    "flex items-center p-6 pt-0",
    "cursor-pointer hover:shadow-md transition-shadow",
    "border-destructive bg-destructive/5",
    "focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
];

const _INPUT: &[&str] = &[
    "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm",
    "ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium",
    "placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2",
    "focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
];

const _BADGE: &[&str] = &[
    "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors",
    "focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2",
    "bg-primary text-primary-foreground hover:bg-primary/80",
    "bg-secondary text-secondary-foreground hover:bg-secondary/80",
    "bg-destructive text-destructive-foreground hover:bg-destructive/80",
    "border-transparent",
];

const _SEPARATOR: &[&str] = &[
    "shrink-0 bg-border",
    "h-[1px] w-full",
    "h-full w-[1px]",
];

const _SKELETON: &[&str] = &[
    "animate-pulse rounded-md bg-muted",
];

const _AVATAR: &[&str] = &[
    "relative flex shrink-0 overflow-hidden rounded-full",
    "h-10 w-10",
    "aspect-square h-full w-full",
    "flex h-full w-full items-center justify-center rounded-full bg-muted",
];

const _TEXTAREA: &[&str] = &[
    "flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm",
    "ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none",
    "focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
    "disabled:cursor-not-allowed disabled:opacity-50",
];

const _LABEL: &[&str] = &[
    "text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70",
];
