use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/frontend-server-leptos.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <FlatRoutes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=Home/>
            </FlatRoutes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
    <section class="container grid items-center gap-6 pb-8 pt-6 md:py-10">
      <div class="flex max-w-[980px] flex-col items-start gap-2">
        <h1 class="text-3xl font-extrabold leading-tight tracking-tighter md:text-4xl">
          Плаформа по распространению данных <br class="hidden sm:inline" />
          поможет заработать каждому.
        </h1>
        <p class="max-w-[700px] text-lg text-muted-foreground">
          Данные для вашего бизнеса | Фриланс для Data Engineers
        </p>

      </div>
    </section>
    }
}
